// src/converter/nodes/table.rs

use crate::converter::context::{color_expr, escape_typst, resolve_variable_to_typst};
use crate::domain::{TableCellContent, TableStyle};
use serde_json::Value;

pub fn format_table(
    headers: &Option<Vec<String>>,
    rows: &Option<Vec<Vec<TableCellContent>>>,
    loop_data: &Option<String>,
    row_template: &Option<Vec<TableCellContent>>,
    footer: &Option<Vec<TableCellContent>>,
    _data: &Value,
    style: &Option<TableStyle>,
) -> Result<String, String> {
    let mut table_code = "#table(\n".to_string();

    let col_count = headers
        .as_ref()
        .map(|h| h.len())
        .or_else(|| row_template.as_ref().map(|t| t.len()))
        .or_else(|| rows.as_ref().and_then(|r| r.first()).map(|row| row.len()))
        .unwrap_or(1);

    let col_defs = if let Some(s) = style.as_ref() {
        if let Some(cols) = &s.columns {
            cols.join(", ")
        } else {
            vec!["1fr"; col_count].join(", ")
        }
    } else {
        vec!["1fr"; col_count].join(", ")
    };
    table_code.push_str(&format!("  columns: ({}),\n", col_defs));

    let inset = style
        .as_ref()
        .and_then(|s| s.inset.as_deref())
        .unwrap_or("8pt");
    table_code.push_str(&format!("  inset: {},\n", inset));

    let stroke = style
        .as_ref()
        .and_then(|s| s.stroke.as_deref())
        .unwrap_or("0.5pt");
    table_code.push_str(&format!("  stroke: {},\n", stroke));

    if let Some(hdrs) = headers {
        let header_bg = style.as_ref().and_then(|s| s.header_bg.as_deref());
        let repeat = style
            .as_ref()
            .and_then(|s| s.repeat_header)
            .unwrap_or(false);

        if repeat {
            table_code.push_str("  table.header(\n");
        }

        for (idx, h) in hdrs.iter().enumerate() {
            let h_escaped = escape_typst(h);
            let h_aligned = wrap_cell_align(&h_escaped, style, idx);
            if let Some(bg) = header_bg {
                table_code.push_str(&format!(
                    "    [#table.cell(fill: {})[*{}*]],\n",
                    color_expr(bg),
                    h_aligned
                ));
            } else {
                table_code.push_str(&format!("    [*{}*],\n", h_aligned));
            }
        }

        if repeat {
            table_code.push_str("  ),\n");
        }
    }

    if let Some(static_rows) = rows {
        for row in static_rows {
            for (idx, cell) in row.iter().enumerate() {
                let value = render_table_cell(cell, style, idx);
                table_code.push_str(&format!("  [{}],\n", value));
            }
        }
    }

    if let (Some(path), Some(template)) = (loop_data, row_template) {
        let escaped_path = path.replace('\\', "\\\\").replace('"', "\\\"");

        // FIX: Use ..{ ... } to spread the array returned by the for loop
        table_code.push_str(&format!(
            "  ..{{\n    let __rows = safe-get(data, \"{}\")\n    if type(__rows) == array {{\n      for __idx in range(__rows.len()) {{\n        let item = __rows.at(__idx)\n        (\n",
            escaped_path
        ));

        for (col_idx, cell) in template.iter().enumerate() {
            let value = render_table_cell_loop(cell, style, col_idx);
            // FIX: Ensure every cell is wrapped in [ ... ]
            table_code.push_str(&format!("          [{}],\n", value));
        }

        table_code.push_str("        )\n      }\n    } else { () }\n  },\n");
    }

    if let Some(ftr) = footer {
        for (idx, cell) in ftr.iter().enumerate() {
            let value = render_table_cell(cell, style, idx);
            table_code.push_str(&format!("  [{}],\n", value));
        }
    }

    table_code.push_str(")\n");

    if let Some(width) = style.as_ref().and_then(|s| s.width.as_deref()) {
        table_code = format!("#block(width: {})[\n{}\n]", width, table_code);
    }

    Ok(table_code)
}

fn render_table_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold } => {
            let value = if key == "__index" || key == "__index_0" {
                "0".to_string()
            } else {
                resolve_variable_to_typst(key)
            };
            let wrapped = wrap_cell_align(&value, style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Text { text, bold } => {
            let text = escape_typst(text);
            let wrapped = wrap_cell_align(&text, style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

fn render_table_cell_loop(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold } => {
            let value = if key == "__index" {
                "#{__idx + 1}".to_string()
            } else if key == "__index_0" {
                "#{__idx}".to_string()
            } else {
                let escaped_key = key.replace('\\', "\\\\").replace('"', "\\\"");
                format!("#{{ safe-get(item, \"{}\") }}", escaped_key)
            };

            let wrapped = wrap_cell_align(&value, style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Text { text, bold } => {
            let text = escape_typst(text);
            let wrapped = wrap_cell_align(&text, style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

fn wrap_cell_align(content: &str, style: &Option<TableStyle>, idx: usize) -> String {
    let align = style
        .as_ref()
        .and_then(|s| s.column_align.as_ref())
        .and_then(|aligns| aligns.get(idx))
        .map(|s| s.as_str());

    match align {
        Some("right") => format!("#align(right)[{}]", content),
        Some("center") => format!("#align(center)[{}]", content),
        _ => content.to_string(),
    }
}
