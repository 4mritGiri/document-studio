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
            let align_css = alignment
                .as_deref()
                .map(|a| format!(" style=\"text-align: {}\"", a))
                .unwrap_or_default();
            let text = format_inline_html(content, data);
            Ok(format!("<p{}>{}</p>\n", align_css, text))
        }
        Node::Heading { level, content, .. } => {
            let text = format_inline_html(content, data);
            Ok(format!("<h{level}>{text}</h{level}>\n"))
        }
        // ... handle tables, lists, etc. using standard HTML tags ...
        _ => Ok(String::new()),
    }
}
