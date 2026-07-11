// src/engines/html/mod.rs

pub mod converter;

use crate::domain::{DocumentRequest, InlineContent, Node};
use crate::engines::{DocumentEngine, RenderOutput};
use serde_json::Value;

pub struct HtmlEngine;

impl DocumentEngine for HtmlEngine {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String> {
        // 1. Convert DOM to HTML body
        let html_body = render_nodes(&request.content, &request.data)?;

        // 2. Wrap in a basic HTML boilerplate
        let full_html = format!(
            "<!DOCTYPE html><html><head><meta charset='UTF-8'><title>{}</title></head><body>{}</body></html>",
            request.template_id, html_body
        );

        Ok(RenderOutput {
            bytes: full_html.into_bytes(),
            mime_type: "text/html".to_string(),
            suggested_filename: format!("{}.html", request.template_id),
        })
    }
}

// --- HTML Rendering Logic ---

fn render_nodes(nodes: &[Node], data: &Value) -> Result<String, String> {
    let mut out = String::new();
    for node in nodes {
        out.push_str(&render_node(node, data)?);
    }
    Ok(out)
}

fn render_node(node: &Node, data: &Value) -> Result<String, String> {
    match node {
        Node::Paragraph { content, alignment } => {
            let style = alignment
                .as_deref()
                .map(|a| format!(" style=\"text-align:{}\"", a))
                .unwrap_or_default();
            let text = render_inline(content, data);
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
            let text = render_inline(content, data);
            Ok(format!("<h{level}{style}>{text}</h{level}>\n"))
        }
        Node::BulletList { items } => {
            let mut html = String::from("<ul>\n");
            for item in items {
                html.push_str(&format!("  <li>{}</li>\n", render_inline(item, data)));
            }
            html.push_str("</ul>\n");
            Ok(html)
        }
        Node::PageBreak => Ok("<hr />\n".to_string()),
        Node::Spacer { height } => Ok(format!("<div style=\"height:{}\"></div>\n", height)),
        // You can easily add Table, Image, etc. here later
        _ => Ok("<!-- Unsupported node type -->\n".to_string()),
    }
}

fn render_inline(items: &[InlineContent], data: &Value) -> String {
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
