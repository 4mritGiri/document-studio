// src/converter/calculations.rs

use serde_json::Value;

/// Evaluates a formula. Handles aggregations, inline math, and combinations of both.
pub fn evaluate_formula(formula: &str, current_row: Option<&Value>, all_rows: &[Value]) -> String {
    let formula = formula.trim();
    if !formula.starts_with('=') {
        return formula.to_string();
    }
    let mut expr = formula[1..].trim().to_string();

    // 1. Resolve all aggregations first (e.g., sum(price) -> 600000.0000)
    let agg_funcs = ["sum", "avg", "min", "max", "count"];
    for func in agg_funcs {
        let pattern = format!(r"{}\(([^)]+)\)", func);
        if let Ok(re) = regex::Regex::new(&pattern) {
            expr = re
                .replace_all(&expr, |caps: &regex::Captures| {
                    let field = caps[1].trim();
                    let val = match func {
                        "sum" => sum_field(field, all_rows),
                        "avg" => avg_field(field, all_rows),
                        "min" => min_field(field, all_rows),
                        "max" => max_field(field, all_rows),
                        "count" => count_field(field, all_rows) as f64,
                        _ => 0.0,
                    };
                    format!("{:.4}", val) // Keep precision for intermediate math
                })
                .to_string();
        }
    }

    // 2. Evaluate the remaining simple math (e.g., 600000.0000 * 0.13)
    if let Some(result) = evaluate_simple_math(&expr, current_row) {
        return format!("{:.2}", result);
    }

    // If it was just a single aggregation, format it cleanly to 2 decimal places
    if let Ok(val) = expr.parse::<f64>() {
        return format!("{:.2}", val);
    }

    expr
}

fn evaluate_simple_math(expr: &str, row: Option<&Value>) -> Option<f64> {
    let ops = ['+', '-', '*', '/'];
    for op in ops {
        if let Some((left, right)) = expr.split_once(op) {
            let left_val = resolve_operand(left.trim(), row);
            let right_val = resolve_operand(right.trim(), row);

            return Some(match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => {
                    if right_val != 0.0 {
                        left_val / right_val
                    } else {
                        0.0
                    }
                }
                _ => unreachable!(),
            });
        }
    }
    None
}

fn resolve_operand(operand: &str, row: Option<&Value>) -> f64 {
    if let Ok(num) = operand.parse::<f64>() {
        return num;
    }
    get_num(row, operand).unwrap_or(0.0)
}

// --- Aggregation Helpers ---

fn sum_field(field: &str, rows: &[Value]) -> f64 {
    rows.iter()
        .map(|row| get_num(Some(row), field).unwrap_or(0.0))
        .sum()
}

fn avg_field(field: &str, rows: &[Value]) -> f64 {
    if rows.is_empty() {
        return 0.0;
    }
    let sum: f64 = rows
        .iter()
        .map(|row| get_num(Some(row), field).unwrap_or(0.0))
        .sum();
    sum / rows.len() as f64
}

fn min_field(field: &str, rows: &[Value]) -> f64 {
    rows.iter()
        .filter_map(|row| get_num(Some(row), field))
        .fold(f64::INFINITY, f64::min)
}

fn max_field(field: &str, rows: &[Value]) -> f64 {
    rows.iter()
        .filter_map(|row| get_num(Some(row), field))
        .fold(f64::NEG_INFINITY, f64::max)
}

fn count_field(field: &str, rows: &[Value]) -> usize {
    rows.iter()
        .filter(|row| get_num(Some(row), field).is_some())
        .count()
}

pub fn get_all_rows(data: &Value, path: &str) -> Option<Vec<Value>> {
    let mut current = data;
    for part in path.split('.') {
        match current {
            Value::Object(map) => current = map.get(part)?,
            Value::Array(arr) => current = arr.get(part.parse::<usize>().ok()?)?,
            _ => return None,
        }
    }
    if let Value::Array(arr) = current {
        Some(arr.clone())
    } else {
        None
    }
}

fn get_num(row: Option<&Value>, key: &str) -> Option<f64> {
    row.and_then(|r| r.get(key)).and_then(|v| {
        if let Some(n) = v.as_f64() {
            return Some(n);
        }
        if let Some(s) = v.as_str() {
            return s.replace(',', "").parse::<f64>().ok();
        }
        None
    })
}
