// src/converter/nodes/table/formula.rs

use regex::Regex;

/// Translates an inline math formula into safe Typst code.
/// Fixes the "cannot apply 'or' to integer" error by using Typst's native if-expressions.
pub fn translate_to_typst(expr: &str) -> String {
    let re = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    re.replace_all(expr, |caps: &regex::Captures| {
        let word = &caps[0];
        let safe_key = word.replace('\\', "\\\\").replace('"', "\\\"");
        // Typst doesn't have a coalescing operator for numbers.
        // We use an if-expression to default to 0 if the field is missing.
        format!(
            "({{ let v = safe-get(item, \"{}\"); if v == none {{ 0 }} else {{ v }} }})",
            safe_key
        )
    })
    .to_string()
}

/// Checks if a formula contains aggregation functions (which belong in footers, not rows).
pub fn is_aggregation(expr: &str) -> bool {
    expr.contains("sum(")
        || expr.contains("avg(")
        || expr.contains("min(")
        || expr.contains("max(")
        || expr.contains("count(")
}
