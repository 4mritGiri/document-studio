// src/converter/nodes/table/mod.rs

pub mod cell;
pub mod formula;

use crate::converter::calculations;
use crate::converter::context::{color_expr, escape_typst, safe_typst_token};
use crate::domain::{TableCellContent, TableStyle};
use serde_json::Value;

/// A table fragment whose content depends on live request data (rows
/// generated from `loop_data`, and/or footer aggregations like SUM/AVG)
/// that must NOT be resolved while building the cached layout.
///
/// `loop_data` is a path string into `request.data` (e.g. "items") — it is
/// NOT part of the cache key (`hash(request.content, request.page)`), so
/// two requests sharing the same template but different `data` would
/// otherwise silently share resolved row values on a cache hit. This was
/// a real, reproduced bug: a customer's invoice line items and computed
/// totals could leak into a different customer's document whenever both
/// used the same template structure.
///
/// This mirrors `qr::QrRequest` exactly, for exactly the same reason:
/// `resolve_deferred` (called from `engines/typst/mod.rs`) must run once
/// per request, AFTER the cache lookup, against the live `request.data` —
/// never before, and its result must never be cached.
#[derive(Debug, Clone)]
pub enum TableRequest {
    Rows {
        token: String,
        loop_data: String,
        row_template: Vec<TableCellContent>,
        style: Option<TableStyle>,
    },
    Footer {
        token: String,
        footer: Vec<TableCellContent>,
        loop_data: Option<String>,
        style: Option<TableStyle>,
    },
}

/// Resolves every deferred table fragment against the REAL, live request
/// data, returning (token, rendered_markup) pairs. The caller splices
/// each pair into the cached layout string via a plain string replace.
/// Call once per request, after retrieving (or generating) the cached
/// layout — never cache the result of this function.
pub fn resolve_deferred(requests: &[TableRequest], data: &Value) -> Vec<(String, String)> {
    let mut out = Vec::with_capacity(requests.len());
    for req in requests {
        match req {
            TableRequest::Rows {
                token,
                loop_data,
                row_template,
                style,
            } => {
                let all_rows = calculations::get_all_rows(data, loop_data).unwrap_or_default();
                let ctx = calculations::TranspileContext {
                    is_loop: true,
                    loop_path: loop_data.clone(),
                };
                let mut rendered = String::new();
                for (row_index, item) in all_rows.iter().enumerate() {
                    for (idx, c) in row_template.iter().enumerate() {
                        let val =
                            cell::render_loop_cell(c, style, idx, &ctx, item, &all_rows, row_index);
                        rendered.push_str(&format!(
                            "          {},\n",
                            cell::wrap_cell_span(c, &val, None)
                        ));
                    }
                }
                out.push((token.clone(), rendered));
            }
            TableRequest::Footer {
                token,
                footer,
                loop_data,
                style,
            } => {
                let all_rows = loop_data
                    .as_ref()
                    .and_then(|p| calculations::get_all_rows(data, p))
                    .unwrap_or_default();
                let ctx = calculations::TranspileContext {
                    is_loop: false,
                    loop_path: loop_data.clone().unwrap_or_default(),
                };
                let striped = style.as_ref().and_then(|s| s.striped_rows.as_ref());
                let mut rendered = String::new();
                for (idx, c) in footer.iter().enumerate() {
                    let val = cell::render_footer_cell(c, style, idx, &all_rows, &ctx);
                    let fill_override = if striped.is_some() {
                        Some("white")
                    } else {
                        None
                    };
                    rendered.push_str(&format!(
                        "  {},\n",
                        cell::wrap_cell_span(c, &val, fill_override)
                    ));
                }
                out.push((token.clone(), rendered));
            }
        }
    }
    out
}

/// Builds the static structure of a `#table(...)` call. Deliberately takes
/// NO reference to `request.data` anywhere in this function — that is the
/// entire point of the fix. Everything data-dependent is recorded into
/// `table_requests` as a placeholder token instead, resolved later by
/// `resolve_deferred` against live data, outside the cache.
pub fn format_table(
    headers: &Option<Vec<String>>,
    rows: &Option<Vec<Vec<TableCellContent>>>,
    loop_data: &Option<String>,
    row_template: &Option<Vec<TableCellContent>>,
    footer: &Option<Vec<TableCellContent>>,
    style: &Option<TableStyle>,
    table_requests: &mut Vec<TableRequest>,
) -> Result<String, String> {
    let mut table_code = "#table(\n".to_string();

    // 1. Determine column count safely
    let col_count = headers
        .as_ref()
        .map(|h| h.len())
        .or_else(|| row_template.as_ref().map(|t| t.len()))
        .or_else(|| rows.as_ref().and_then(|r| r.first()).map(|row| row.len()))
        .unwrap_or(1);

    // 2. Build column definitions
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

    // 3. Base styling
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

    // 4. Row striping logic
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

    // 5. Header generation — safe: header text lives directly in
    // `content` (hashed into the cache key), not in `data`.
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

    // 6. Static rows — also safe: their content (Text/Formula strings,
    // Variable keys) lives directly in `content`, not resolved from live
    // `data` here (Variable cells defer resolution to Typst compile time
    // via resolve_variable_to_typst, same as elsewhere in the converter).
    if let Some(static_rows) = rows {
        for row in static_rows {
            for (idx, c) in row.iter().enumerate() {
                let val = cell::render_static_cell(c, style, idx);
                table_code.push_str(&format!("  {},\n", cell::wrap_cell_span(c, &val, None)));
            }
        }
    }

    // 7. Dynamic loop rows — DEFERRED. This is the fix: previously this
    // resolved concrete values from `data` right here, baking them into
    // the string that gets cached. Now it only records what to resolve
    // later and drops in a placeholder token.
    if let (Some(path), Some(template)) = (loop_data, row_template) {
        let token = format!("@@TABLE_ROWS_{}@@", table_requests.len());
        table_requests.push(TableRequest::Rows {
            token: token.clone(),
            loop_data: path.clone(),
            row_template: template.clone(),
            style: style.clone(),
        });
        table_code.push_str(&format!("  {}\n", token));
    }

    // 8. Footer rows — DEFERRED for the same reason (SUM/AVG/etc need
    // `all_rows` resolved from live data). Deferred unconditionally, even
    // for footers with only Text/Variable cells, to keep this uniformly
    // safe rather than trying to partially defer per-cell-type.
    if let Some(ftr) = footer {
        let token = format!("@@TABLE_FOOTER_{}@@", table_requests.len());
        table_requests.push(TableRequest::Footer {
            token: token.clone(),
            footer: ftr.clone(),
            loop_data: loop_data.clone(),
            style: style.clone(),
        });
        table_code.push_str(&format!("  {}\n", token));
    }

    table_code.push_str(")\n");

    // 9. Width wrapper
    if let Some(width) = style.as_ref().and_then(|s| s.width.as_deref()) {
        let safe_width = safe_typst_token(width, "100%");
        table_code = format!("#block(width: {})[\n{}\n]", safe_width, table_code);
    }

    Ok(table_code)
}
