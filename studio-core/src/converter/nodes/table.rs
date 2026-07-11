// src/converter/nodes/table.rs

use crate::config::MAX_TABLE_LOOP_ROWS;
use crate::converter::context::{
    color_expr, escape_typst, get_value_by_path, resolve_variable_with_context,
};
use crate::domain::{TableCellContent, TableStyle};
use serde_json::Value;

pub fn format_table(
    headers: &Option<Vec<String>>,
    rows: &Option<Vec<Vec<TableCellContent>>>,
    loop_data: &Option<String>,
    row_template: &Option<Vec<TableCellContent>>,
    footer: &Option<Vec<TableCellContent>>,
    data: &Value,
    style: &Option<TableStyle>,
) -> Result<String, String> {
    if let Some(path) = loop_data {
        if let Some(Value::Array(items)) = get_value_by_path(data, path) {
            if items.len() > MAX_TABLE_LOOP_ROWS {
                return Err(format!("Table exceeds {} rows", MAX_TABLE_LOOP_ROWS));
            }
        }
    }

    let mut out = "#table(\n".to_string();
    let col_count = headers
        .as_ref()
        .map(|h| h.len())
        .or_else(|| row_template.as_ref().map(|t| t.len()))
        .unwrap_or(1);
    let col_defs = style
        .as_ref()
        .and_then(|s| s.columns.as_ref())
        .map(|c| c.join(", "))
        .unwrap_or_else(|| vec!["1fr"; col_count].join(", "));

    out.push_str(&format!("  columns: ({col_defs}),\n"));
    out.push_str(&format!(
        "  inset: {},\n",
        style
            .as_ref()
            .and_then(|s| s.inset.as_deref())
            .unwrap_or("8pt")
    ));
    out.push_str(&format!(
        "  stroke: {},\n",
        style
            .as_ref()
            .and_then(|s| s.stroke.as_deref())
            .unwrap_or("0.5pt")
    ));

    if let Some(hdrs) = headers {
        let bg = style.as_ref().and_then(|s| s.header_bg.as_deref());
        for (idx, h) in hdrs.iter().enumerate() {
            let h = wrap_cell_align(&escape_typst(h), style, idx);
            if let Some(bg) = bg {
                out.push_str(&format!(
                    "  [#table.cell(fill: {})[*{}*]],\n",
                    color_expr(bg),
                    h
                ));
            } else {
                out.push_str(&format!("  [*{}*],\n", h));
            }
        }
    }

    if let Some(static_rows) = rows {
        for row in static_rows {
            for (idx, cell) in row.iter().enumerate() {
                let val = wrap_cell_align(&render_cell(cell, data, data, None), style, idx);
                out.push_str(&format!("  [{}],\n", val));
            }
        }
    }

    if let (Some(path), Some(template)) = (loop_data, row_template) {
        if let Some(Value::Array(items)) = get_value_by_path(data, path) {
            for (row_idx, item) in items.iter().enumerate() {
                for (col_idx, cell) in template.iter().enumerate() {
                    let val = wrap_cell_align(
                        &render_cell(cell, item, data, Some(row_idx)),
                        style,
                        col_idx,
                    );
                    out.push_str(&format!("  [{}],\n", val));
                }
            }
        }
    }

    if let Some(ftr) = footer {
        for (idx, cell) in ftr.iter().enumerate() {
            let val = wrap_cell_align(&render_cell(cell, data, data, None), style, idx);
            out.push_str(&format!("  [{}],\n", val));
        }
    }

    out.push_str(")");
    if let Some(width) = style.as_ref().and_then(|s| s.width.as_deref()) {
        out = format!("#block(width: {})[\n{}\n]", width, out);
    }
    Ok(out)
}

fn render_cell(
    cell: &TableCellContent,
    local: &Value,
    global: &Value,
    index: Option<usize>,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold } => {
            let val = if key == "__index" {
                index.map(|i| (i + 1).to_string()).unwrap_or_default()
            } else if key == "__index_0" {
                index.map(|i| i.to_string()).unwrap_or_default()
            } else {
                resolve_variable_with_context(key, local, global)
            };
            if bold.unwrap_or(false) {
                format!("*{}*", val)
            } else {
                val
            }
        }
        TableCellContent::Text { text, bold } => {
            let t = escape_typst(text);
            if bold.unwrap_or(false) {
                format!("*{}*", t)
            } else {
                t
            }
        }
    }
}

fn wrap_cell_align(content: &str, style: &Option<TableStyle>, idx: usize) -> String {
    match style
        .as_ref()
        .and_then(|s| s.column_align.as_ref())
        .and_then(|a| a.get(idx))
        .map(|s| s.as_str())
    {
        Some("right") => format!("#align(right)[{}]", content),
        Some("center") => format!("#align(center)[{}]", content),
        _ => content.to_string(),
    }
}
