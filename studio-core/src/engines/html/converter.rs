// src/engines/html/converter.rs

use crate::config::MAX_TABLE_LOOP_ROWS;
use crate::domain::{
    InlineContent, Node, PageHeaderFooter, PageSettings, TableCellContent, TableStyle,
    WatermarkSettings,
};
use serde_json::Value;

/// Escapes text for safe placement in HTML element content or a
/// double-quoted attribute value.
pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Allowlist-based validator for CSS length/keyword tokens (e.g. "12px",
/// "1.5cm", "100%", "1fr", "auto"). Rejects anything containing characters
/// that could break out of a `style="..."` attribute or inject additional
/// CSS/HTML — quotes, semicolons, parens, backslashes, etc. Falls back to a
/// safe default rather than ever embedding an unvalidated value.
fn safe_css_token(value: &str, fallback: &str) -> String {
    let ok = !value.is_empty()
        && value.len() <= 32
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '%' | ' '));
    if ok {
        value.to_string()
    } else {
        fallback.to_string()
    }
}

fn safe_css_color(value: &str, fallback: &str) -> String {
    let ok = !value.is_empty()
        && value.len() <= 32
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '#');
    if ok {
        value.to_string()
    } else {
        fallback.to_string()
    }
}

/// Only `data:image/...` URIs and `http(s)://` URLs are accepted as image
/// sources. Anything else (e.g. `javascript:`, `file://`) is rejected
/// outright rather than being escaped-and-hoped-for-the-best.
fn safe_image_src(src: &str) -> Option<String> {
    if src.starts_with("data:image/") || src.starts_with("https://") || src.starts_with("http://") {
        Some(html_escape(src))
    } else {
        None
    }
}

/// Keeps generated filenames/titles restricted to a safe character set —
/// `template_id` is client-controlled and ends up in a `Content-Disposition`
/// filename and an HTML `<title>`, so it needs the same treatment as any
/// other untrusted input.
pub fn sanitize_filename(id: &str) -> String {
    let cleaned: String = id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.is_empty() {
        "document".to_string()
    } else {
        cleaned.chars().take(100).collect()
    }
}

fn get_value_by_path<'a>(data: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = data;
    for part in path.split('.') {
        match current {
            Value::Object(map) => current = map.get(part)?,
            Value::Array(arr) => current = arr.get(part.parse::<usize>().ok()?)?,
            _ => return None,
        }
    }
    Some(current)
}

fn format_value_html(val: &Value) -> String {
    match val {
        Value::String(s) => html_escape(s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}

/// Resolves a variable against a local (loop-item) scope first, falling
/// back to the global document data — the same local-then-global semantics
/// as the Typst engine's `item`/`data`, so a template behaves identically
/// whether it's rendered to PDF or HTML.
fn resolve_key(key: &str, local: &Value, global: &Value) -> String {
    if let Some(v) = get_value_by_path(local, key) {
        return format_value_html(v);
    }
    if let Some(v) = get_value_by_path(global, key) {
        return format_value_html(v);
    }
    format!("[MISSING: {}]", html_escape(key))
}

/// Renders the document body plus, if configured, a letterhead
/// (`page.background`), a watermark, and a header/footer.
///
/// HTML has no native concept of discrete pages — it's one continuously
/// scrolling document. This is a real, structural limitation compared to
/// PDF, not something that can be fully worked around, so here's exactly
/// what this does and doesn't give you:
///
/// - `header` / `footer`: rendered once, and marked with a print stylesheet
///   (`position: fixed` inside `@media print`) so that when the page is
///   actually printed (or "printed to PDF" from a browser), it repeats on
///   every physical page — this is a well-established technique and works
///   in real browser print engines. On screen, it just sits at the top/
///   bottom of the document.
/// - `first_page_content` / `first_page_only`: honored for the single
///   HTML "page" (the whole document counts as page 1).
/// - `skip_first_page`: since there is no page after "page 1" in a
///   continuous HTML view, this renders nothing. If you print the result
///   to multiple physical pages, the same header still repeats on all of
///   them — CSS has no reliable, cross-browser way to detect physical page
///   number and vary content accordingly (this is exactly the kind of
///   thing a real paginating engine like the Typst/PDF path handles
///   correctly and generic HTML/CSS fundamentally cannot).
/// - `background` (letterhead shapes/logo) and `watermark`: rendered once,
///   positioned relative to the top of the document via the existing
///   `Placed` mechanics.
pub fn json_to_html(
    content: &[Node],
    data: &Value,
    page: &Option<PageSettings>,
) -> Result<String, String> {
    let header = page.as_ref().and_then(|p| p.header.as_ref());
    let footer = page.as_ref().and_then(|p| p.footer.as_ref());
    let background = page.as_ref().and_then(|p| p.background.as_ref());
    let watermark = page.as_ref().and_then(|p| p.watermark.as_ref());

    let mut out = String::new();
    out.push_str(&print_style_block(header.is_some(), footer.is_some()));

    out.push_str("<div style=\"position:relative;\">\n");

    if let Some(bg_nodes) = background {
        for n in bg_nodes {
            out.push_str(&render_node(n, data, data)?);
        }
    }
    if let Some(wm) = watermark {
        out.push_str(&render_watermark(wm));
    }
    if let Some(h) = header {
        out.push_str(&render_header_footer(h, data, "doc-header"));
    }

    for node in content {
        out.push_str(&render_node(node, data, data)?);
    }

    if let Some(f) = footer {
        out.push_str(&render_header_footer(f, data, "doc-footer"));
    }

    out.push_str("</div>\n");
    Ok(out)
}

fn print_style_block(has_header: bool, has_footer: bool) -> String {
    let top_margin = if has_header { "3cm" } else { "0" };
    let bottom_margin = if has_footer { "3cm" } else { "0" };
    format!(
        r#"<style>
  .doc-header, .doc-footer {{ width: 100%; box-sizing: border-box; padding: 4px 12px; }}
  @media print {{
    body {{ margin-top: {top}; margin-bottom: {bottom}; }}
    .doc-header {{ position: fixed; top: 0; left: 0; right: 0; }}
    .doc-footer {{ position: fixed; bottom: 0; left: 0; right: 0; }}
  }}
</style>
"#,
        top = top_margin,
        bottom = bottom_margin
    )
}

fn render_header_footer(hf: &PageHeaderFooter, data: &Value, css_class: &str) -> String {
    if hf.skip_first_page.unwrap_or(false) {
        // See the doc comment on json_to_html: there's no second "page" to
        // show this on in a continuous HTML view, so there's nothing to render.
        return String::new();
    }

    let content_items: &[InlineContent] = hf
        .first_page_content
        .as_deref()
        .unwrap_or(hf.content.as_slice());

    let text = render_inline(content_items, data, data);
    let align = match hf.alignment.as_deref() {
        Some("left") => "left",
        Some("right") => "right",
        _ => "center",
    };

    format!(
        "<div class=\"{class}\" style=\"text-align:{align};\">{text}</div>\n",
        class = css_class,
        align = align,
        text = text
    )
}

fn render_watermark(wm: &WatermarkSettings) -> String {
    let opacity = wm.opacity.unwrap_or(0.2).clamp(0.0, 1.0);
    let angle = wm.angle.unwrap_or(-45.0);
    let size = safe_css_token(wm.font_size.as_deref().unwrap_or("50pt"), "50pt");
    let color = safe_css_color(wm.color.as_deref().unwrap_or("gray"), "gray");
    let text = html_escape(&wm.text);

    // angle/opacity come from strongly-typed f32 fields (already validated by
    // serde deserialization), so no injection risk there — only the
    // string-typed fields (font_size, color) need the safe_css_* filtering.
    let (position_css, transform) = match wm.position.as_deref() {
        Some("top-left") => ("top:5%; left:5%;", format!("rotate({}deg)", angle)),
        Some("top-right") => ("top:5%; right:5%;", format!("rotate({}deg)", angle)),
        Some("bottom-left") => ("bottom:5%; left:5%;", format!("rotate({}deg)", angle)),
        Some("bottom-right") => ("bottom:5%; right:5%;", format!("rotate({}deg)", angle)),
        _ => (
            "top:50%; left:50%;",
            format!("translate(-50%, -50%) rotate({}deg)", angle),
        ),
    };

    format!(
        "<div style=\"position:absolute; {pos} transform:{transform}; font-size:{size}; \
         color:{color}; opacity:{opacity}; font-weight:bold; white-space:nowrap; \
         pointer-events:none; z-index:0;\">{text}</div>\n",
        pos = position_css,
        transform = transform,
        size = size,
        color = color,
        opacity = opacity,
        text = text
    )
}

fn render_node(node: &Node, local: &Value, global: &Value) -> Result<String, String> {
    match node {
        Node::Paragraph { content, alignment } => Ok(format!(
            "<p{}>{}</p>\n",
            align_style(alignment),
            render_inline(content, local, global)
        )),
        Node::Heading {
            level,
            content,
            alignment,
        } => {
            let lvl = (*level).clamp(1, 6);
            Ok(format!(
                "<h{lvl}{}>{}</h{lvl}>\n",
                align_style(alignment),
                render_inline(content, local, global)
            ))
        }
        Node::BulletList { items } => {
            let mut html = String::from("<ul>\n");
            for item in items {
                html.push_str(&format!(
                    "  <li>{}</li>\n",
                    render_inline(item, local, global)
                ));
            }
            html.push_str("</ul>\n");
            Ok(html)
        }
        Node::Table {
            headers,
            rows,
            loop_data,
            row_template,
            footer,
            style,
        } => Ok(render_table(
            headers,
            rows,
            loop_data,
            row_template,
            footer,
            style,
            local,
            global,
        )),
        Node::PageBreak => Ok("<div style=\"page-break-after: always;\"></div>\n".to_string()),
        Node::Spacer { height } => {
            let h = safe_css_token(height, "1em");
            Ok(format!("<div style=\"height:{}\"></div>\n", h))
        }
        Node::Image {
            src,
            width,
            height,
            alignment,
        } => Ok(render_image(src, width, height, alignment)),
        Node::Shape {
            kind,
            width,
            height,
            fill,
            rotate,
        } => Ok(render_shape(kind, width, height, fill, rotate)),
        Node::Placed {
            anchor,
            dx,
            dy,
            content,
        } => render_placed(anchor, dx, dy, content, local, global),
        Node::Columns {
            items,
            column_widths,
            gutter,
        } => render_columns(items, column_widths, gutter, local, global),

        Node::QrCode {
            data,
            width,
            alignment,
        } => render_qr_code_html(data, width, alignment),

        Node::Qr {
            content,
            size,
            fill,
            background,
            error_correction,
            alignment,
        } => Ok(render_qr(
            content,
            size,
            fill,
            background,
            error_correction,
            alignment,
            global,
        )),
    }
}

fn render_qr(
    content: &[InlineContent],
    size: &Option<String>,
    fill: &Option<String>,
    background: &Option<String>,
    error_correction: &Option<String>,
    alignment: &Option<String>,
    data: &Value,
) -> String {
    let text = crate::converter::nodes::qr::resolve_plain(content, data);
    let svg_bytes = match crate::converter::nodes::qr::generate_qr_svg(
        &text,
        fill,
        background,
        error_correction,
    ) {
        Ok(b) => b,
        Err(e) => return format!("<!-- QR code error: {} -->\n", html_escape(&e)),
    };

    let svg_str = String::from_utf8_lossy(&svg_bytes).replacen(
        "<svg",
        "<svg style=\"width:100%;height:auto;display:block;\"",
        1,
    );

    let size_css = safe_css_token(size.as_deref().unwrap_or("3cm"), "3cm");
    let wrapper = match alignment.as_deref() {
        Some("right") => " style=\"text-align:right;\"",
        Some("center") => " style=\"text-align:center;\"",
        _ => "",
    };

    format!(
        "<div{}><div style=\"width:{}; display:inline-block;\">{}</div></div>\n",
        wrapper, size_css, svg_str
    )
}

fn align_style(alignment: &Option<String>) -> String {
    match alignment.as_deref() {
        Some("right") => " style=\"text-align:right\"".to_string(),
        Some("center") => " style=\"text-align:center\"".to_string(),
        Some("justify") => " style=\"text-align:justify\"".to_string(),
        _ => String::new(),
    }
}

fn render_inline(items: &[InlineContent], local: &Value, global: &Value) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => {
                let mut text = html_escape(&t.text).replace('\n', "<br/>");
                if t.bold.unwrap_or(false) {
                    text = format!("<strong>{}</strong>", text);
                }
                if t.italic.unwrap_or(false) {
                    text = format!("<em>{}</em>", text);
                }
                out.push_str(&text);
            }
            InlineContent::Variable(v) => out.push_str(&resolve_key(&v.key, local, global)),
            // HTML output has no fixed pagination, so there's no true page
            // number to render; leave a placeholder token a client-side
            // paginator (or a "print" stylesheet) could substitute later.
            InlineContent::PageNumber(_) => out.push_str("{{page}}"),
        }
    }
    out
}

fn cell_align(style: &Option<TableStyle>, idx: usize) -> &'static str {
    style
        .as_ref()
        .and_then(|s| s.column_align.as_ref())
        .and_then(|a| a.get(idx))
        .map(|s| match s.as_str() {
            "right" => "right",
            "center" => "center",
            _ => "left",
        })
        .unwrap_or("left")
}

#[allow(clippy::too_many_arguments)]
fn render_table(
    headers: &Option<Vec<String>>,
    rows: &Option<Vec<Vec<TableCellContent>>>,
    loop_data: &Option<String>,
    row_template: &Option<Vec<TableCellContent>>,
    footer: &Option<Vec<TableCellContent>>,
    style: &Option<TableStyle>,
    local: &Value,
    global: &Value,
) -> String {
    let width = style
        .as_ref()
        .and_then(|s| s.width.as_deref())
        .map(|w| safe_css_token(w, "100%"))
        .unwrap_or_else(|| "100%".to_string());

    let stroke = style
        .as_ref()
        .and_then(|s| s.stroke.as_deref())
        .unwrap_or("0.5pt");
    let border_css = if stroke == "none" {
        "none".to_string()
    } else {
        format!("{} solid #444", safe_css_token(stroke, "1px"))
    };

    let inset = style
        .as_ref()
        .and_then(|s| s.inset.as_deref())
        .map(|i| safe_css_token(i, "8px"))
        .unwrap_or_else(|| "8px".to_string());

    let header_bg = style
        .as_ref()
        .and_then(|s| s.header_bg.as_deref())
        .map(|c| safe_css_color(c, "#f3f4f6"));

    let mut html = format!(
        "<table style=\"width:{}; border-collapse: collapse;\">\n",
        width
    );

    if let Some(hdrs) = headers {
        html.push_str("  <thead>\n    <tr>\n");
        for (idx, h) in hdrs.iter().enumerate() {
            let align = cell_align(style, idx);
            let bg = header_bg
                .as_deref()
                .map(|c| format!("background-color:{};", c))
                .unwrap_or_default();
            html.push_str(&format!(
                "      <th style=\"border:{}; padding:{}; text-align:{}; {}\">{}</th>\n",
                border_css,
                inset,
                align,
                bg,
                html_escape(h)
            ));
        }
        html.push_str("    </tr>\n  </thead>\n");
    }

    html.push_str("  <tbody>\n");

    if let Some(static_rows) = rows {
        for row in static_rows {
            html.push_str("    <tr>\n");
            for (idx, cell) in row.iter().enumerate() {
                html.push_str(&render_table_cell(
                    cell,
                    style,
                    idx,
                    &border_css,
                    &inset,
                    local,
                    global,
                    None,
                ));
            }
            html.push_str("    </tr>\n");
        }
    }

    if let (Some(path), Some(template)) = (loop_data, row_template) {
        if let Some(Value::Array(items)) = get_value_by_path(global, path) {
            // Bound how many rows a single table can generate — this is the
            // HTML-engine equivalent of the row cap enforced elsewhere; a
            // multi-million-row `data` array shouldn't be able to produce a
            // multi-million-row HTML table.
            for (row_idx, item) in items.iter().take(MAX_TABLE_LOOP_ROWS).enumerate() {
                html.push_str("    <tr>\n");
                for (col_idx, cell) in template.iter().enumerate() {
                    html.push_str(&render_table_cell(
                        cell,
                        style,
                        col_idx,
                        &border_css,
                        &inset,
                        item,
                        global,
                        Some(row_idx),
                    ));
                }
                html.push_str("    </tr>\n");
            }
        }
    }

    html.push_str("  </tbody>\n");

    if let Some(ftr) = footer {
        html.push_str("  <tfoot>\n    <tr>\n");
        for (idx, cell) in ftr.iter().enumerate() {
            html.push_str(&render_table_cell(
                cell,
                style,
                idx,
                &border_css,
                &inset,
                local,
                global,
                None,
            ));
        }
        html.push_str("    </tr>\n  </tfoot>\n");
    }

    html.push_str("</table>\n");
    html
}

#[allow(clippy::too_many_arguments)]
fn render_table_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    idx: usize,
    border_css: &str,
    inset: &str,
    local: &Value,
    global: &Value,
    index: Option<usize>,
) -> String {
    let align = cell_align(style, idx);
    let (value, bold) = match cell {
        TableCellContent::Variable { key, bold } => {
            let v = if key == "__index" {
                index.map(|i| (i + 1).to_string()).unwrap_or_default()
            } else if key == "__index_0" {
                index.map(|i| i.to_string()).unwrap_or_default()
            } else {
                resolve_key(key, local, global)
            };
            (v, bold.unwrap_or(false))
        }
        TableCellContent::Text { text, bold } => (html_escape(text), bold.unwrap_or(false)),
    };
    let value = if bold {
        format!("<strong>{}</strong>", value)
    } else {
        value
    };
    format!(
        "      <td style=\"border:{}; padding:{}; text-align:{};\">{}</td>\n",
        border_css, inset, align, value
    )
}

fn render_image(
    src: &str,
    width: &Option<String>,
    height: &Option<String>,
    alignment: &Option<String>,
) -> String {
    let safe_src = match safe_image_src(src) {
        Some(s) => s,
        None => return "<!-- image omitted: unsupported src scheme -->\n".to_string(),
    };

    let mut style = String::new();
    if let Some(w) = width {
        style.push_str(&format!("width:{};", safe_css_token(w, "auto")));
    }
    if let Some(h) = height {
        style.push_str(&format!("height:{};", safe_css_token(h, "auto")));
    }

    let wrapper_style = match alignment.as_deref() {
        Some("right") => " style=\"text-align:right;\"",
        Some("center") => " style=\"text-align:center;\"",
        _ => "",
    };

    format!(
        "<div{}><img src=\"{}\" style=\"{}\" alt=\"\" /></div>\n",
        wrapper_style, safe_src, style
    )
}

fn render_shape(
    kind: &str,
    width: &str,
    height: &str,
    fill: &Option<String>,
    rotate: &Option<String>,
) -> String {
    let w = safe_css_token(width, "1cm");
    let h = safe_css_token(height, "1cm");
    let color = safe_css_color(fill.as_deref().unwrap_or("black"), "black");
    let rotate_css = rotate
        .as_ref()
        .map(|r| format!("transform: rotate({});", safe_css_token(r, "0deg")))
        .unwrap_or_default();

    match kind {
        "circle" => format!(
            "<div style=\"width:{w}; height:{h}; background:{color}; border-radius:50%; {rotate_css}\"></div>\n"
        ),
        "triangle" => format!(
            "<div style=\"width:0; height:0; border-left:{w} solid transparent; border-bottom:{h} solid {color}; {rotate_css}\"></div>\n"
        ),
        _ => format!(
            "<div style=\"width:{w}; height:{h}; background:{color}; {rotate_css}\"></div>\n"
        ),
    }
}

fn render_placed(
    anchor: &Option<String>,
    dx: &Option<String>,
    dy: &Option<String>,
    content: &Node,
    local: &Value,
    global: &Value,
) -> Result<String, String> {
    let inner = render_node(content, local, global)?;
    let (vert, horiz) = match anchor.as_deref() {
        Some("top-right") => ("top:0;", "right:0;"),
        Some("top-center") => ("top:0;", "left:50%; transform:translateX(-50%);"),
        Some("bottom-left") => ("bottom:0;", "left:0;"),
        Some("bottom-right") => ("bottom:0;", "right:0;"),
        Some("bottom-center") => ("bottom:0;", "left:50%; transform:translateX(-50%);"),
        Some("center") => ("top:50%;", "left:50%; transform:translate(-50%,-50%);"),
        _ => ("top:0;", "left:0;"),
    };
    let dx_css = dx
        .as_ref()
        .map(|v| format!("margin-left:{};", safe_css_token(v, "0")))
        .unwrap_or_default();
    let dy_css = dy
        .as_ref()
        .map(|v| format!("margin-top:{};", safe_css_token(v, "0")))
        .unwrap_or_default();

    Ok(format!(
        "<div style=\"position:absolute; {} {} {} {}\">{}</div>\n",
        vert, horiz, dx_css, dy_css, inner
    ))
}

fn render_columns(
    items: &[Vec<Node>],
    column_widths: &Option<Vec<String>>,
    gutter: &Option<String>,
    local: &Value,
    global: &Value,
) -> Result<String, String> {
    let cols_css = match column_widths {
        Some(widths) => widths
            .iter()
            .map(|w| safe_css_token(w, "1fr"))
            .collect::<Vec<_>>()
            .join(" "),
        None => vec!["1fr"; items.len()].join(" "),
    };
    let gap = gutter
        .as_ref()
        .map(|g| safe_css_token(g, "1em"))
        .unwrap_or_else(|| "1em".to_string());

    let mut html = format!(
        "<div style=\"display:grid; grid-template-columns:{}; gap:{};\">\n",
        cols_css, gap
    );
    for column in items {
        html.push_str("  <div>\n");
        for n in column {
            html.push_str(&render_node(n, local, global)?);
        }
        html.push_str("  </div>\n");
    }
    html.push_str("</div>\n");
    Ok(html)
}

/// Generates an inline SVG QR code for HTML output.
fn render_qr_code_html(
    data: &str,
    width: &Option<String>,
    alignment: &Option<String>,
) -> Result<String, String> {
    use qrcode::QrCode;

    // 1. Generate the QR Code matrix
    let code =
        QrCode::new(data.as_bytes()).map_err(|e| format!("QR code generation failed: {}", e))?;

    let matrix_size = code.width();

    // 2. Build a highly optimized SVG string using a single <path>
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" shape-rendering="crispEdges">"#,
        size = matrix_size
    );

    let mut path_data = String::new();
    for y in 0..matrix_size {
        for x in 0..matrix_size {
            if code[(x, y)] == qrcode::Color::Dark {
                path_data.push_str(&format!("M{} {}h1v1h-1z", x, y));
            }
        }
    }
    svg.push_str(&format!(r#"<path d="{}" fill="black"/>"#, path_data));
    svg.push_str("</svg>");

    // 3. Wrap in a div for alignment and sizing
    let w = width
        .as_deref()
        .map(|w| safe_css_token(w, "2.5cm"))
        .unwrap_or_else(|| "2.5cm".to_string());

    let wrapper_style = match alignment.as_deref() {
        Some("right") => " style=\"text-align:right;\"",
        Some("center") => " style=\"text-align:center;\"",
        _ => "",
    };

    Ok(format!(
        "<div{}><div style=\"width:{}; display:inline-block;\">{}</div></div>\n",
        wrapper_style, w, svg
    ))
}
