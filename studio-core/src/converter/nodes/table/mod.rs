// src/converter/nodes/table/mod.rs

pub mod cell;
pub mod formula;

use crate::converter::calculations;
use crate::converter::context::{color_expr, escape_typst, safe_typst_token};
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
    let mut table_code = "#table(\n".to_string();

    let col_count = headers
        .as_ref()
        .map(|h| h.len())
        .or_else(|| row_template.as_ref().map(|t| t.len()))
        .or_else(|| rows.as_ref().and_then(|r| r.first()).map(|row| row.len()))
        .unwrap_or(1);

    let col_defs = if let Some(s) = style.as_ref() {
        if let Some(cols) = &s.columns {
            cols.iter()
                .map(|c| safe_typst_token(c, "1fr"))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            vec!["1fr"; col_count].join(", ")
        }
    } else {
        vec!["1fr"; col_count].join(", ")
    };
    table_code.push_str(&format!("  columns: ({col_defs}),\n"));

    let inset = safe_typst_token(
        style
            .as_ref()
            .and_then(|s| s.inset.as_deref())
            .unwrap_or("8pt"),
        "8pt",
    );
    table_code.push_str(&format!("  inset: {},\n", inset));

    let stroke_raw = style
        .as_ref()
        .and_then(|s| s.stroke.as_deref())
        .unwrap_or("0.5pt");
    let stroke = if stroke_raw == "none" {
        "none".to_string()
    } else {
        safe_typst_token(stroke_raw, "0.5pt")
    };
    table_code.push_str(&format!("  stroke: {},\n", stroke));

    let header_bg = style.as_ref().and_then(|s| s.header_bg.as_deref());
    let striped = style.as_ref().and_then(|s| s.striped_rows.as_ref());
    let repeat = style
        .as_ref()
        .and_then(|s| s.repeat_header)
        .unwrap_or(false);

    if let Some(stripes) = striped {
        if !stripes.is_empty() {
            let stripe_colors: Vec<String> = stripes.iter().map(|c| color_expr(c)).collect();
            let header_color = header_bg
                .map(color_expr)
                .unwrap_or_else(|| stripe_colors[0].clone());
            let has_header = headers.is_some();
            table_code.push_str(&format!(
                "  fill: (col, row) => if {has_header} and row == 0 {{ {header} }} else {{ \
                 let stripes = ({colors},); stripes.at(calc.rem(row - {offset}, stripes.len())) }},\n",
                has_header = has_header, header = header_color, colors = stripe_colors.join(", "), offset = if has_header { 1 } else { 0 },
            ));
        }
    }

    if let Some(hdrs) = headers {
        if repeat {
            table_code.push_str("  table.header(\n");
        }
        for (idx, h) in hdrs.iter().enumerate() {
            let h_escaped = escape_typst(h);

            let h_aligned = match style
                .as_ref()
                .and_then(|s| s.column_align.as_ref())
                .and_then(|a| a.get(idx))
                .map(|s| s.as_str())
            {
                Some("right") => format!("#align(right)[{}]", h_escaped),
                Some("center") => format!("#align(center)[{}]", h_escaped),
                _ => h_escaped,
            };

            if striped.is_some() {
                table_code.push_str(&format!("    [*{}*],\n", h_aligned));
            } else if let Some(bg) = header_bg {
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

    let all_rows = if let (Some(path), Some(_)) = (loop_data, row_template) {
        calculations::get_all_rows(data, path).unwrap_or_default()
    } else {
        Vec::new()
    };

    if let Some(static_rows) = rows {
        for row in static_rows {
            for (idx, c) in row.iter().enumerate() {
                let val = cell::render_static_cell(c, style, idx);
                table_code.push_str(&format!("  {},\n", cell::wrap_cell_span(c, &val, None)));
            }
        }
    }

    if let (Some(path), Some(template)) = (loop_data, row_template) {
        let escaped_path = path.replace('\\', "\\\\").replace('"', "\\\"");
        table_code.push_str(&format!(
            "  {{\n    let __rows = safe-get(data, \"{}\")\n    if type(__rows) == array {{\n      for __idx in range(__rows.len()) {{\n        let item = __rows.at(__idx)\n",
            escaped_path
        ));
        for (idx, c) in template.iter().enumerate() {
            let val = cell::render_loop_cell(c, style, idx);
            table_code.push_str(&format!(
                "        {}\n",
                cell::wrap_cell_span(c, &val, None)
            ));
        }
        table_code.push_str("      }\n    }\n  },\n");
    }

    if let Some(ftr) = footer {
        for (idx, c) in ftr.iter().enumerate() {
            let val = cell::render_footer_cell(c, style, idx, &all_rows);
            let fill_override = if striped.is_some() {
                Some("white")
            } else {
                None
            };
            table_code.push_str(&format!(
                "  {},\n",
                cell::wrap_cell_span(c, &val, fill_override)
            ));
        }
    }

    table_code.push_str(")\n");

    if let Some(width) = style.as_ref().and_then(|s| s.width.as_deref()) {
        let safe_width = safe_typst_token(width, "100%");
        table_code = format!("#block(width: {})[\n{}\n]", safe_width, table_code);
    }

    Ok(table_code)
}
