// src/converter/builder.rs

use crate::config::MAX_NODE_DEPTH;
use crate::converter::context::{build_header_footer_block, color_expr_with_alpha, escape_typst};
use crate::converter::nodes::{layout, media, table, text};
use crate::domain::{Node, PageSettings};
use serde_json::Value;
use std::collections::HashMap;
use typst::foundations::Bytes;

pub fn json_to_typst(
    content: &[Node],
    _data: &Value,
    page: &Option<PageSettings>,
) -> Result<(String, HashMap<String, Bytes>), String> {
    let mut typst_code = String::new();
    let mut assets = HashMap::new();

    typst_code.push_str(
        r#"
#let item = (:) // Empty dictionary for non-loop contexts

#let safe-get(obj, path) = {
  let parts = path.split(".")
  let current = obj
  for part in parts {
    // 1. Handle Dictionaries (e.g., data.customer_name)
    if type(current) == dictionary {
      if part in current.keys() {
        current = current.at(part)
      } else {
        return none
      }
    }
    // 2. Handle Arrays (e.g., data.collaterals.0.type)
    else if type(current) == array {
      // Check if the path part is a valid integer string (e.g., "0", "1")
      if part.match(regex("^\\d+$")) != none {
        let idx = int(part)
        if idx >= 0 and idx < current.len() {
          current = current.at(idx)
        } else {
          return none
        }
      } else {
        return none
      }
    }
    // 3. If it's neither a dict nor an array, the path is invalid
    else {
      return none
    }
  }
  current
}
"#,
    );

    typst_code.push_str(&build_page_preamble(page, _data, &mut assets)?);

    for node in content {
        typst_code.push_str(&render_node(node, _data, &mut assets, 0)?);
    }

    Ok((typst_code, assets))
}

fn build_page_preamble(
    page: &Option<PageSettings>,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
) -> Result<String, String> {
    let header = page.as_ref().and_then(|p| p.header.as_ref());
    let footer = page.as_ref().and_then(|p| p.footer.as_ref());
    let background = page.as_ref().and_then(|p| p.background.as_ref());
    let watermark = page.as_ref().and_then(|p| p.watermark.as_ref()); // NEW

    let top = if header.is_some() { "3cm" } else { "2cm" };
    let bottom = if footer.is_some() { "3cm" } else { "2cm" };

    let mut out = format!(
        "#set page(width: 210mm, height: 297mm, margin: (top: {top}, bottom: {bottom}, x: 2cm)"
    );

    if let Some(h) = header {
        out.push_str(&format!(
            ",\n  header: {}",
            build_header_footer_block(h, data)
        ));
    }
    if let Some(f) = footer {
        out.push_str(&format!(
            ",\n  footer: {}",
            build_header_footer_block(f, data)
        ));
    }

    // --- NEW: Build the Background (Letterhead + Watermark) ---
    let mut bg_content = String::new();

    // 1. Add Letterhead/Background shapes (logos, triangles, etc.)
    if let Some(bg) = background {
        for n in bg {
            bg_content.push_str(&render_node(n, data, assets, 0)?);
        }
    }

    // 2. Add the Watermark (if configured)
    if let Some(wm) = watermark {
        let opacity = wm.opacity.unwrap_or(0.2);
        let angle = wm.angle.unwrap_or(-45.0);
        let size = wm.font_size.as_deref().unwrap_or("50pt");
        let color = wm.color.as_deref().unwrap_or("gray");

        let position = wm.position.as_deref().unwrap_or("center");

        // FIX: Use Typst's alignment syntax correctly
        // For true center, we need to use both horizontal and vertical alignment
        let align_expr = match position {
            "top-left" => "top + left",
            "top-right" => "top + right",
            "bottom-left" => "bottom + left",
            "bottom-right" => "bottom + right",
            "top-center" => "top + center",
            "bottom-center" => "bottom + center",
            "center" => "center + horizon", // FIX: Use center + horizon for true center
            _ => "center + horizon",        // Default to true center
        };

        let text = escape_typst(&wm.text);
        let alpha_hex = format!("{:02x}", (opacity.clamp(0.0, 1.0) * 255.0) as u8);
        let fill_color = color_expr_with_alpha(color, &alpha_hex);

        bg_content.push_str(&format!(
            r#"
            #place(
              {},
              text(
                fill: {},
                size: {},
                weight: "bold"
              )[
                #rotate({}deg)[{}]
              ]
            )
            "#,
            align_expr, fill_color, size, angle, text
        ));
    }

    // 3. Inject the combined background into the page settings
    if !bg_content.is_empty() {
        out.push_str(&format!(",\n  background: [\n{}\n]", bg_content));
    }
    let font_family = page
        .as_ref()
        .and_then(|p| p.default_font.as_deref())
        .unwrap_or("Times New Roman");
    out.push_str(&format!(
        "\n)\n#set text(font: \"{}\", size: 12pt)\n\n",
        font_family
    ));
    Ok(out)
}

pub fn render_node(
    node: &Node,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
    depth: usize,
) -> Result<String, String> {
    if depth > MAX_NODE_DEPTH {
        return Err("Max nesting depth exceeded".to_string());
    }
    match node {
        Node::Paragraph { content, alignment } => {
            Ok(text::render_paragraph(content, alignment, data))
        }

        Node::Heading {
            level,
            content,
            alignment,
        } => Ok(text::render_heading(*level, content, alignment, data)),

        Node::BulletList { items } => Ok(text::render_bullet_list(items, data)),

        Node::Table {
            headers,
            rows,
            loop_data,
            row_template,
            footer,
            style,
        } => Ok(format!(
            "{}\n\n",
            table::format_table(headers, rows, loop_data, row_template, footer, data, style)?
        )),

        Node::PageBreak => Ok("#pagebreak()\n\n".to_string()),

        Node::Spacer { height } => Ok(format!("#v({})\n\n", height)),

        Node::Image {
            src,
            width,
            height,
            alignment,
        } => media::render_image(src, width, height, alignment, assets),

        Node::Shape {
            kind,
            width,
            height,
            fill,
            rotate,
        } => Ok(media::render_shape(kind, width, height, fill, rotate)),

        Node::Placed {
            anchor,
            dx,
            dy,
            content,
        } => layout::render_placed(anchor, dx, dy, content, data, assets, depth + 1),

        Node::Columns {
            items,
            column_widths,
            gutter,
        } => layout::render_columns(items, column_widths, gutter, data, assets, depth + 1),

        Node::QrCode {
            data: qr_data,
            width,
            alignment,
        } => crate::converter::nodes::qr::render_qr_code(qr_data, width, alignment, assets),
    }
}
