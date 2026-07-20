// src/converter/builder.rs

use crate::config::MAX_NODE_DEPTH;
use crate::converter::context::{
    build_header_footer_block, color_expr_with_alpha, escape_typst, safe_typst_token,
};
use crate::converter::nodes::qr::{self, QrRequest};
use crate::converter::nodes::table::TableRequest;
use crate::converter::nodes::{layout, media, table, text};
use crate::domain::{Node, PageSettings};
use serde_json::Value;
use std::collections::HashMap;
use typst::foundations::Bytes;

pub fn json_to_typst(
    content: &[Node],
    _data: &Value,
    page: &Option<PageSettings>,
) -> Result<
    (
        String,
        HashMap<String, Bytes>,
        Vec<QrRequest>,
        Vec<TableRequest>,
    ),
    String,
> {
    let mut typst_code = String::new();
    let mut assets = HashMap::new();
    let mut qr_requests = Vec::new();
    let mut table_requests = Vec::new();

    typst_code.push_str(
        r#"
#let item = (:) // Empty dictionary for non-loop contexts

// NEW: Helper function to safely convert none to 0 for math operations
#let num(val) = if val == none { 0 } else { val }

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

    typst_code.push_str(&build_page_preamble(
        page,
        _data,
        &mut assets,
        &mut qr_requests,
        &mut table_requests,
    )?);

    for node in content {
        typst_code.push_str(&render_node(
            node,
            _data,
            &mut assets,
            &mut qr_requests,
            &mut table_requests,
            0,
        )?);
    }

    Ok((typst_code, assets, qr_requests, table_requests))
}

fn build_page_preamble(
    page: &Option<PageSettings>,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
    qr_requests: &mut Vec<QrRequest>,
    table_requests: &mut Vec<TableRequest>,
) -> Result<String, String> {
    let header = page.as_ref().and_then(|p| p.header.as_ref());
    let footer = page.as_ref().and_then(|p| p.footer.as_ref());
    let background = page.as_ref().and_then(|p| p.background.as_ref());
    let watermark = page.as_ref().and_then(|p| p.watermark.as_ref());

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

    let mut bg_content = String::new();

    if let Some(bg) = background {
        for n in bg {
            bg_content.push_str(&render_node(
                n,
                data,
                assets,
                qr_requests,
                table_requests,
                0,
            )?);
        }
    }

    if let Some(wm) = watermark {
        let opacity = wm.opacity.unwrap_or(0.2);
        let angle = wm.angle.unwrap_or(-45.0);
        // font_size is a client-controlled string and was previously
        // interpolated raw — same injection class described in context.rs.
        let size = safe_typst_token(wm.font_size.as_deref().unwrap_or("50pt"), "50pt");
        let color = wm.color.as_deref().unwrap_or("gray");
        let position = wm.position.as_deref().unwrap_or("center");

        let align_expr = match position {
            "top-left" => "top + left",
            "top-right" => "top + right",
            "bottom-left" => "bottom + left",
            "bottom-right" => "bottom + right",
            "top-center" => "top + center",
            "bottom-center" => "bottom + center",
            "center" => "center + horizon",
            _ => "center + horizon",
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

    if !bg_content.is_empty() {
        out.push_str(&format!(",\n  background: [\n{}\n]", bg_content));
    }

    out.push_str("\n)\n#set text(font: \"Times New Roman\", size: 12pt)\n\n");
    Ok(out)
}

pub fn render_node(
    node: &Node,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
    qr_requests: &mut Vec<QrRequest>,
    table_requests: &mut Vec<TableRequest>,
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
            table::format_table(
                headers,
                rows,
                loop_data,
                row_template,
                footer,
                style,
                table_requests,
            )?
        )),
        Node::PageBreak => Ok("#pagebreak()\n\n".to_string()),
        Node::Spacer { height } => Ok(format!("#v({})\n\n", safe_typst_token(height, "1em"))),
        Node::Image {
            src,
            width,
            height,
            alignment,
        } => media::render_image(src, width, height, alignment, assets),
        Node::Shape {
            kind,
            path_data,
            width,
            height,
            fill,
            stroke,
            stroke_width,
            rotate,
        } => media::render_shape(
            kind,
            path_data,
            width,
            height,
            fill,
            stroke,
            stroke_width,
            rotate,
            assets,
        ),

        Node::Chart {
            chart_type,
            title,
            data,
            width,
            height,
            colors,
        } => crate::converter::nodes::chart::render_chart(
            chart_type, title, data, width, height, colors, &None, assets,
        ),
        Node::Placed {
            anchor,
            dx,
            dy,
            content,
        } => layout::render_placed(
            anchor,
            dx,
            dy,
            content,
            data,
            assets,
            qr_requests,
            table_requests,
            depth + 1,
        ),
        Node::Columns {
            items,
            column_widths,
            gutter,
        } => layout::render_columns(
            items,
            column_widths,
            gutter,
            data,
            assets,
            qr_requests,
            table_requests,
            depth + 1,
        ),
        Node::Qr {
            content,
            size,
            fill,
            background,
            error_correction,
            alignment,
        } => Ok(qr::render_qr_placeholder(
            content,
            size,
            fill,
            background,
            error_correction,
            alignment,
            qr_requests,
        )),
        // QrCode is kept only for backward compatibility with existing
        // templates — it now delegates to the exact same safe, deferred
        // path as `Qr` instead of its own (previously cache-unsafe,
        // unvalidated-size) implementation. See qr.rs for why.
        Node::QrCode {
            data: qr_data,
            width,
            alignment,
        } => Ok(qr::render_qr_placeholder(
            &[crate::domain::InlineContent::Text(
                crate::domain::TextNode {
                    text: qr_data.clone(),
                    ..Default::default()
                },
            )],
            width,
            &None,
            &None,
            &None,
            alignment,
            qr_requests,
        )),
    }
}
