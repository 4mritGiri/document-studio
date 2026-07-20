// src/converter/nodes/table/cell.rs

use crate::converter::calculations::{evaluate_formula, format_number};
use crate::converter::context::{color_expr, escape_typst, resolve_variable_to_typst};
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
    row_index: usize,
) -> String {
    match cell {
        TableCellContent::Variable { key, bold, .. } => {
            let value = if key == "__index" {
                (row_index + 1).to_string()
            } else if key == "__index_0" {
                row_index.to_string()
            } else {
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
            } else if let Ok(num) = raw_result.parse::<f64>() {
                format_number(num, loc, dec)
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
            } else if let Ok(num) = raw_result.parse::<f64>() {
                format_number(num, loc, dec)
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

/// Extracts and validates this cell's own background color, if any.
/// Routed through `color_expr` — the same allowlist validator used for
/// every other color field in the converter — since `fill` is
/// client-controlled and must never be interpolated raw into generated
/// Typst source.
fn resolve_cell_fill(cell: &TableCellContent) -> Option<String> {
    let fill = match cell {
        TableCellContent::Variable { fill, .. } => fill,
        TableCellContent::Text { fill, .. } => fill,
        TableCellContent::Formula { fill, .. } => fill,
    };
    fill.as_deref().map(color_expr)
}

/// Wraps a rendered cell value in `table.cell(...)` if it needs a colspan,
/// rowspan, or fill. All three combine into a single `table.cell()` call —
/// never nested — matching the same fix applied earlier for the Typst
/// engine's striping/colspan interaction.
///
/// `fill_override` is for automatic, engine-driven fills (e.g. forcing a
/// footer row to a neutral background so `striped_rows` can't bleed into
/// totals). A cell's own explicit `fill` field always takes priority over
/// that automatic override — if the caller explicitly asked for a color,
/// that's what they get.
pub fn wrap_cell_span(cell: &TableCellContent, value: &str, fill_override: Option<&str>) -> String {
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
    let fill = resolve_cell_fill(cell).or_else(|| fill_override.map(|s| s.to_string()));

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
