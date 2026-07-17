// src/converter/nodes/table/cell.rs

use crate::converter::context::{escape_typst, resolve_variable_to_typst};
use crate::converter::nodes::table::formula;
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
) -> String {
    match cell {
        TableCellContent::Variable { key, bold, .. } => {
            let value = if key == "__index" {
                "#{__idx + 1}".to_string()
            } else if key == "__index_0" {
                "#{__idx}".to_string()
            } else {
                format!(
                    "#{{ safe-get(item, \"{}\") }}",
                    key.replace('\\', "\\\\").replace('"', "\\\"")
                )
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
            if formula.starts_with('=') {
                let expr = formula[1..].trim();
                if formula::is_aggregation(expr) {
                    return wrap_cell_align(&escape_typst(formula), style, col_idx);
                }
                let typst_expr = formula::translate_to_typst(expr);
                let value = format!("#{{ {} }}", typst_expr);
                let wrapped = wrap_cell_align(&value, style, col_idx);
                return if bold.unwrap_or(false) {
                    format!("*{}*", wrapped)
                } else {
                    wrapped
                };
            }
            let wrapped = wrap_cell_align(&escape_typst(formula), style, col_idx);
            if bold.unwrap_or(false) {
                format!("*{}*", wrapped)
            } else {
                wrapped
            }
        }
    }
}

/// Renders a cell in the footer (evaluates aggregations in Rust).
pub fn render_footer_cell(
    cell: &TableCellContent,
    style: &Option<TableStyle>,
    col_idx: usize,
    all_rows: &[Value],
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
            bold,
            ..
        } => {
            let raw_result =
                crate::converter::calculations::evaluate_formula(formula, None, all_rows);
            let formatted = if let Some(f) = fmt {
                f.replace("{value}", &raw_result)
            } else {
                raw_result
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
/// FIX: Now correctly extracts colspan/rowspan from ALL variants, including Formula.
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
        } => (*colspan, *rowspan), // FIXED
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

pub fn wrap_cell_align(content: &str, style: &Option<TableStyle>, idx: usize) -> String {
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
