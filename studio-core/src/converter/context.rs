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

/// Converts a color string (hex or named) into a Typst rgb() expression with an alpha channel.
/// `alpha_hex` should be a 2-digit hex string (e.g., "33" for 20% opacity).
pub fn color_expr_with_alpha(fill: &str, alpha_hex: &str) -> String {
    if fill.starts_with('#') {
        // If it's already an 8-digit hex (e.g., "#ff000033"), use it directly
        if fill.len() == 9 {
            format!("rgb(\"{}\")", fill)
        } else {
            // Otherwise, strip the '#' and append the alpha hex
            format!("rgb(\"#{}{}\")", &fill[1..], alpha_hex)
        }
    } else {
        // Map common named colors to their 6-digit hex equivalents + alpha
        let hex = match fill.to_lowercase().as_str() {
            "gray" | "grey" => "808080",
            "black" => "000000",
            "white" => "ffffff",
            "red" => "ff0000",
            "green" => "008000",
            "blue" => "0000ff",
            "yellow" => "ffff00",
            _ => "808080", // Default to gray
        };
        format!("rgb(\"#{}{}\")", hex, alpha_hex)
    }
}

pub fn format_inline_content(items: &[InlineContent], _data: &Value) -> String {
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
            InlineContent::Variable(v) => result.push_str(&resolve_variable_to_typst(&v.key)),
            InlineContent::PageNumber(p) => result.push_str(&render_page_number_format(
                p.format.as_deref().unwrap_or("{current}"),
            )),
        }
    }
    result
}

pub fn build_header_footer_block(hf: &PageHeaderFooter, _data: &Value) -> String {
    let align = match hf.alignment.as_deref() {
        Some("left") => "left",
        Some("right") => "right",
        _ => "center",
    };

    // FIX: Pass _data, but it won't be used for variable resolution anymore
    let default_text = format_header_footer_inline(&hf.content, _data);
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
                format_header_footer_inline(fp, _data)
            ),
            default_block.clone(),
        )
    } else {
        (default_block.clone(), default_block)
    };

    format!("context {{\n  let pg = counter(page).get().first()\n  if pg == 1 {{ {first} }} else {{ {rest} }}\n}}")
}

fn format_header_footer_inline(items: &[InlineContent], _data: &Value) -> String {
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
            // FIX: Defer resolution to Typst!
            InlineContent::Variable(v) => result.push_str(&resolve_variable_to_typst(&v.key)),
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

pub fn resolve_variable_to_typst(key: &str) -> String {
    let escaped_key = key.replace('\\', "\\\\").replace('"', "\\\"");

    format!(
        r#"#{{
  let __val = safe-get(item, "{}")
  if __val != none {{ __val }} else {{ safe-get(data, "{}") }}
}}"#,
        escaped_key, escaped_key
    )
}
