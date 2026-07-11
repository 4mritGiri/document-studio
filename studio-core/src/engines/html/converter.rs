// src/engines/html/converter.rs

use crate::domain::{InlineContent, Node};
use serde_json::Value;

pub fn json_to_html(content: &[Node], data: &Value) -> Result<String, String> {
    let mut html = String::new();
    for node in content {
        html.push_str(&render_node(node, data)?);
    }
    Ok(html)
}

fn render_node(node: &Node, data: &Value) -> Result<String, String> {
    match node {
        Node::Paragraph { content, alignment } => {
            let style = alignment
                .as_deref()
                .map(|a| format!(" style=\"text-align:{}\"", a))
                .unwrap_or_default();
            let text = format_inline_html(content, data);
            Ok(format!("<p{}>{}</p>\n", style, text))
        }
        Node::Heading {
            level,
            content,
            alignment,
        } => {
            let style = alignment
                .as_deref()
                .map(|a| format!(" style=\"text-align:{}\"", a))
                .unwrap_or_default();
            let text = format_inline_html(content, data);
            Ok(format!("<h{level}{style}>{text}</h{level}>\n"))
        }
        Node::BulletList { items } => {
            let mut html = String::from("<ul>\n");
            for item in items {
                html.push_str(&format!("  <li>{}</li>\n", format_inline_html(item, data)));
            }
            html.push_str("</ul>\n");
            Ok(html)
        }
        Node::PageBreak => Ok("<hr />\n".to_string()),
        Node::Spacer { height } => Ok(format!("<div style=\"height:{}\"></div>\n", height)),
        _ => Ok("<!-- Unsupported node type -->\n".to_string()),
    }
}

fn format_inline_html(items: &[InlineContent], data: &Value) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => {
                let mut text = html_escape(&t.text);
                if t.bold.unwrap_or(false) {
                    text = format!("<strong>{}</strong>", text);
                }
                if t.italic.unwrap_or(false) {
                    text = format!("<em>{}</em>", text);
                }
                out.push_str(&text);
            }
            InlineContent::Variable(v) => {
                if let Some(val) = resolve_var(&v.key, data) {
                    out.push_str(&html_escape(&val));
                }
            }
            InlineContent::PageNumber(_) => out.push_str("{{page}}"),
        }
    }
    out
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn resolve_var(key: &str, data: &Value) -> Option<String> {
    let mut current = data;
    for part in key.split('.') {
        match current {
            Value::Object(map) => current = map.get(part)?,
            _ => return None,
        }
    }
    match current {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}
