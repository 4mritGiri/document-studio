// src/converter/nodes/layout.rs

use crate::converter::context::safe_typst_token;
use crate::converter::nodes::qr::QrRequest;
use crate::domain::Node;
use serde_json::Value;
use std::collections::HashMap;
use typst::foundations::Bytes;

#[allow(clippy::too_many_arguments)]
pub fn render_placed(
    anchor: &Option<String>,
    dx: &Option<String>,
    dy: &Option<String>,
    content: &Node,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
    qr_requests: &mut Vec<QrRequest>,
    depth: usize,
) -> Result<String, String> {
    let inner = crate::converter::builder::render_node(content, data, assets, qr_requests, depth)?;
    let mut args = vec![anchor_expr(anchor).to_string()];
    if let Some(dx) = dx {
        args.push(format!("dx: {}", safe_typst_token(dx, "0pt")));
    }
    if let Some(dy) = dy {
        args.push(format!("dy: {}", safe_typst_token(dy, "0pt")));
    }
    Ok(format!(
        "#place(\n  {},\n)[\n{}\n]\n\n",
        args.join(",\n  "),
        inner
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn render_columns(
    items: &[Vec<Node>],
    column_widths: &Option<Vec<String>>,
    gutter: &Option<String>,
    data: &Value,
    assets: &mut HashMap<String, Bytes>,
    qr_requests: &mut Vec<QrRequest>,
    depth: usize,
) -> Result<String, String> {
    let col_defs = column_widths
        .as_ref()
        .map(|w| {
            w.iter()
                .map(|x| safe_typst_token(x, "1fr"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| vec!["1fr"; items.len()].join(", "));
    let gutter_safe = safe_typst_token(gutter.as_deref().unwrap_or("1em"), "1em");
    let mut out = format!(
        "#grid(\n  columns: ({}),\n  gutter: {},\n",
        col_defs, gutter_safe
    );
    for column in items {
        let mut cell = String::new();
        for n in column {
            cell.push_str(&crate::converter::builder::render_node(
                n,
                data,
                assets,
                qr_requests,
                depth,
            )?);
        }
        out.push_str(&format!("  [{}],\n", cell));
    }
    out.push_str(")\n\n");
    Ok(out)
}

fn anchor_expr(anchor: &Option<String>) -> &'static str {
    match anchor.as_deref() {
        Some("top-right") => "top + right",
        Some("top-center") => "top + center",
        Some("bottom-left") => "bottom + left",
        Some("bottom-right") => "bottom + right",
        Some("bottom-center") => "bottom + center",
        Some("center") => "center",
        _ => "top + left",
    }
}
