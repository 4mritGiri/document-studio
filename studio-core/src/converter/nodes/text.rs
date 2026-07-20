// src/converter/nodes/text.rs

use crate::converter::context::{
    color_expr, escape_typst, escape_typst_string_literal, resolve_variable_to_typst,
    safe_typst_token, wrap_alignment,
};
use crate::domain::{InlineContent, TextNode};
use serde_json::Value;

pub fn render_paragraph(
    content: &[InlineContent],
    alignment: &Option<String>,
    _data: &Value,
) -> String {
    let text = render_inline(content);
    if text.is_empty() {
        String::new()
    } else {
        format!("{}\n\n", wrap_alignment(&text, alignment))
    }
}

pub fn render_heading(
    level: u8,
    content: &[InlineContent],
    alignment: &Option<String>,
    _data: &Value,
) -> String {
    let prefix = "=".repeat(level as usize);
    let text = render_inline(content);
    format!(
        "{}\n\n",
        wrap_alignment(&format!("{prefix} {text}"), alignment)
    )
}

pub fn render_bullet_list(items: &[Vec<InlineContent>], _data: &Value) -> String {
    let mut out = String::from("#list(\n  indent: 1cm,\n");
    for item in items {
        out.push_str(&format!("  [{}],\n", render_inline(item)));
    }
    out.push_str(")\n\n");
    out
}

fn render_inline(items: &[InlineContent]) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => out.push_str(&render_text_run(t)),
            // Resolution is deferred to Typst compile time (via safe-get
            // against the live `item`/`data`), matching how this already
            // works elsewhere in the converter — keeps the layout cacheable
            // independent of the actual data.
            InlineContent::Variable(v) => out.push_str(&resolve_variable_to_typst(&v.key)),
            InlineContent::PageNumber(_) => out.push_str("#counter(page).display()"),
        }
    }
    out
}

/// Renders one styled text run. Order matters here: bold/italic are markup
/// syntax (`*_`) applied to the escaped text first; underline/strike/link
/// are Typst function calls that each need their own content-block
/// argument; size/color/font all collapse into a single `#text(...)`
/// wrapper. Every user-controlled value goes through the validator that
/// matches its context — safe_typst_token for lengths, color_expr for
/// colors, escape_typst_string_literal for anything inside a Typst string
/// literal (font names, the link URL) — never interpolated raw. See the
/// injection proof/fix in context.rs for why this matters.
fn render_text_run(t: &TextNode) -> String {
    let mut content = escape_typst(&t.text).replace('\n', "\n\n");

    if t.bold.unwrap_or(false) {
        content = format!("*{}*", content);
    }
    if t.italic.unwrap_or(false) {
        content = format!("_{}_", content);
    }
    if t.underline.unwrap_or(false) {
        content = format!("#underline[{}]", content);
    }
    if t.strike.unwrap_or(false) {
        content = format!("#strike[{}]", content);
    }

    let mut text_args = Vec::new();
    if let Some(f) = &t.font_family {
        text_args.push(format!("font: \"{}\"", escape_typst_string_literal(f)));
    }
    if let Some(s) = &t.size {
        text_args.push(format!("size: {}", safe_typst_token(s, "12pt")));
    }
    if let Some(c) = &t.color {
        text_args.push(format!("fill: {}", color_expr(c)));
    }
    if !text_args.is_empty() {
        content = format!("#text({})[{}]", text_args.join(", "), content);
    }

    if let Some(url) = &t.link {
        content = format!(
            "#link(\"{}\")[{}]",
            escape_typst_string_literal(url),
            content
        );
    }

    content
}
