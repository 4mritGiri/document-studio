// src/converter/context.rs

use crate::domain::{InlineContent, PageHeaderFooter};
use serde_json::Value;

pub fn escape_typst(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\\' | '*' | '_' | '#' | '@' | '$' | '<' | '>' | '[' | ']' | '`' | '~' | '^' | '\''
            | '"' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

pub fn wrap_alignment(text: &str, alignment: &Option<String>) -> String {
    match alignment.as_deref() {
        Some("right") => format!("#align(right)[\n{}\n]", text),
        Some("center") => format!("#align(center)[\n{}\n]", text),
        Some("justify") => format!("#align(justify)[\n{}\n]", text),
        _ => text.to_string(),
    }
}

pub fn wrap_alignment_raw(raw: &str, alignment: &Option<String>) -> String {
    match alignment.as_deref() {
        Some("right") => format!("#align(right)[{}]", raw),
        Some("center") => format!("#align(center)[{}]", raw),
        _ => raw.to_string(),
    }
}

pub fn color_expr(fill: &str) -> String {
    if fill.starts_with('#') {
        format!("rgb(\"{}\")", fill)
    } else {
        fill.to_string()
    }
}

pub fn format_inline_content(items: &[InlineContent], data: &Value) -> String {
    let mut result = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => {
                let mut text = escape_typst(&t.text).replace("\n", "\n\n");
                if t.bold.unwrap_or(false) {
                    text = format!("*{}*", text);
                }
                if t.italic.unwrap_or(false) {
                    text = format!("_{}_", text);
                }
                result.push_str(&text);
            }
            InlineContent::Variable(v) => result.push_str(&resolve_variable(&v.key, data)),
            InlineContent::PageNumber(p) => result.push_str(&render_page_number_format(
                p.format.as_deref().unwrap_or("{current}"),
            )),
        }
    }
    result
}

pub fn build_header_footer_block(hf: &PageHeaderFooter, data: &Value) -> String {
    let align = match hf.alignment.as_deref() {
        Some("left") => "left",
        Some("right") => "right",
        _ => "center",
    };
    let default_text = format_header_footer_inline(&hf.content, data);
    let default_block = format!("align({})[{}]", align, default_text);
    let empty_block = "[]".to_string();

    let (first, rest) = if hf.first_page_only.unwrap_or(false) {
        (default_block.clone(), empty_block)
    } else if hf.skip_first_page.unwrap_or(false) {
        (empty_block, default_block.clone())
    } else if let Some(fp) = &hf.first_page_content {
        (
            format!(
                "align({})[{}]",
                align,
                format_header_footer_inline(fp, data)
            ),
            default_block.clone(),
        )
    } else {
        (default_block.clone(), default_block)
    };

    format!("context {{\n  let pg = counter(page).get().first()\n  if pg == 1 {{ {first} }} else {{ {rest} }}\n}}")
}

fn format_header_footer_inline(items: &[InlineContent], data: &Value) -> String {
    let mut result = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => {
                let mut text = escape_typst(&t.text.replace('\n', " "));
                if t.bold.unwrap_or(false) {
                    text = format!("*{}*", text);
                }
                if t.italic.unwrap_or(false) {
                    text = format!("_{}_", text);
                }
                result.push_str(&text);
            }
            InlineContent::Variable(v) => result.push_str(&resolve_variable(&v.key, data)),
            InlineContent::PageNumber(p) => result.push_str(&render_page_number_format(
                p.format.as_deref().unwrap_or("{current}"),
            )),
        }
    }
    result
}

fn render_page_number_format(fmt: &str) -> String {
    let mut result = String::new();
    let mut rest = fmt;
    loop {
        let next = match (rest.find("{current}"), rest.find("{total}")) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        match next {
            None => {
                result.push_str(&escape_typst(rest));
                break;
            }
            Some(idx) => {
                result.push_str(&escape_typst(&rest[..idx]));
                if rest[idx..].starts_with("{current}") {
                    result.push_str("#counter(page).display()");
                    rest = &rest[idx + 9..];
                } else {
                    result.push_str("#context counter(page).final().first()");
                    rest = &rest[idx + 7..];
                }
            }
        }
    }
    result
}

pub fn get_value_by_path<'a>(data: &'a Value, path: &str) -> Option<&'a Value> {
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

pub fn resolve_variable_with_context(key: &str, local: &Value, global: &Value) -> String {
    if let Some(val) = get_value_by_path(local, key) {
        return format_value(val);
    }
    if let Some(val) = get_value_by_path(global, key) {
        return format_value(val);
    }
    format!("[MISSING: {}]", key)
}

fn format_value(val: &Value) -> String {
    match val {
        Value::String(s) => escape_typst(s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => "[INVALID TYPE]".to_string(),
    }
}

pub fn resolve_variable(key: &str, data: &Value) -> String {
    if let Some(val) = get_value_by_path(data, key) {
        format_value(val)
    } else {
        format!("[MISSING: {}]", key)
    }
}
