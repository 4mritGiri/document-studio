// src/converter/nodes/table/cell.rs

use crate::converter::calculations::{evaluate_formula, format_number};
use crate::converter::context::{escape_typst, resolve_variable_to_typst};
use crate::domain::{TableCellContent, TableStyle};
use serde_json::Value;

/// Renders a cell for the static rows.
pub fn render_static_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold, .. } => {
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
        TableCellContent::Text { text, bold, .. } => {
            let wrapped = wrap_cell_align(&escape_typst(text), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Formula { formula, bold, .. } => {
            let wrapped = wrap_cell_align(&escape_typst(formula), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

/// Renders a cell inside the dynamic loop.
pub fn render_loop_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
    _ctx: &crate::converter::calculations::TranspileContext,
    current_row: &Value,
    all_rows: &[Value],
    row_index: usize, // NEW: Pass the row index from Rust
) -> String {
    match cell {
        TableCellContent::Variable { key, bold, .. } => {
            // Resolve entirely in Rust since we are generating static rows
            let value = if key == "__index" {
                (row_index + 1).to_string()
            } else if key == "__index_0" {
                row_index.to_string()
            } else {
                // Try to get as string/number, fallback to MISSING
                current_row
                    .get(key)
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => v.to_string(),
                    })
                    .unwrap_or_else(|| format!("[MISSING: {}]", key))
            };
            let wrapped = wrap_cell_align(&escape_typst(&value), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Text { text, bold, .. } => {
            let wrapped = wrap_cell_align(&escape_typst(text), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Formula {
            formula,
            format: fmt,
            locale,
            decimal_places,
            bold,
            ..
        } => {
            // Evaluate in Rust
            let raw_result = evaluate_formula(formula, Some(current_row), all_rows, "");

            let loc = locale.as_deref().unwrap_or("en-US");
            let dec = *decimal_places;

            let formatted = if let Some(f) = fmt {
                if let Ok(num) = raw_result.parse::<f64>() {
                    let formatted_num = format_number(num, loc, dec);
                    f.replace("{value}", &formatted_num)
                } else {
                    f.replace("{value}", &raw_result)
                }
            } else {
                if let Ok(num) = raw_result.parse::<f64>() {
                    format_number(num, loc, dec)
                } else {
                    raw_result
                }
            };

            let wrapped = wrap_cell_align(&escape_typst(&formatted), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

pub fn render_footer_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
    all_rows: &[Value],
    _ctx: &crate::converter::calculations::TranspileContext,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold, .. } => {
            let wrapped = wrap_cell_align(&resolve_variable_to_typst(key), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Text { text, bold, .. } => {
            let wrapped = wrap_cell_align(&escape_typst(text), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
        TableCellContent::Formula {
            formula,
            format: fmt,
            locale,
            decimal_places,
            bold,
            ..
        } => {
            let raw_result = evaluate_formula(formula, None, all_rows, "");

            let loc = locale.as_deref().unwrap_or("en-US");
            let dec = *decimal_places;

            let formatted = if let Some(f) = fmt {
                if let Ok(num) = raw_result.parse::<f64>() {
                    let formatted_num = format_number(num, loc, dec);
                    f.replace("{value}", &formatted_num)
                } else {
                    f.replace("{value}", &raw_result)
                }
            } else {
                if let Ok(num) = raw_result.parse::<f64>() {
                    format_number(num, loc, dec)
                } else {
                    raw_result
                }
            };

            let wrapped = wrap_cell_align(&escape_typst(&formatted), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

/// Wraps a rendered cell value in `table.cell(...)` if it needs a colspan, rowspan, or fill override.
pub fn wrap_cell_span(cell: &TableCellContent, value: &str, fill: Option<&str>) -> String {
    let (colspan, rowspan) = match cell {
        TableCellContent::Variable {
            colspan, rowspan, ..
        } => (*colspan, *rowspan),
        TableCellContent::Text {
            colspan, rowspan, ..
        } => (*colspan, *rowspan),
        TableCellContent::Formula {
            colspan, rowspan, ..
        } => (*colspan, *rowspan),
    };
    let colspan = colspan.filter(|c| *c > 1);
    let rowspan = rowspan.filter(|r| *r > 1);

    if colspan.is_none() && rowspan.is_none() && fill.is_none() {
        return format!("[{}]", value);
    }

    let mut args = Vec::new();
    if let Some(c) = colspan {
        args.push(format!("colspan: {}", c));
    }
    if let Some(r) = rowspan {
        args.push(format!("rowspan: {}", r));
    }
    if let Some(f) = fill {
        args.push(format!("fill: {}", f));
    }
    format!("table.cell({})[{}]", args.join(", "), value)
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
