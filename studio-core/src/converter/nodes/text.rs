// src/converter/nodes/text.rs

use crate::converter::context::{format_inline_content, wrap_alignment};
use crate::domain::InlineContent;
use serde_json::Value;

pub fn render_paragraph(
    content: &[InlineContent],
    alignment: &Option<String>,
    data: &Value,
) -> String {
    let text = format_inline_content(content, data);
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
    data: &Value,
) -> String {
    let prefix = "=".repeat(level as usize);
    format!(
        "{}\n\n",
        wrap_alignment(
            &format!("{prefix} {}", format_inline_content(content, data)),
            alignment
        )
    )
}

pub fn render_bullet_list(items: &[Vec<InlineContent>], data: &Value) -> String {
    let mut out = String::from("#list(\n  indent: 1cm,\n");
    for item in items {
        out.push_str(&format!("  [{}],\n", format_inline_content(item, data)));
    }
    out.push_str(")\n\n");
    out
}
