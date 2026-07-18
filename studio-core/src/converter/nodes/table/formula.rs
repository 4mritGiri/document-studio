// src/converter/nodes/table/formula.rs

use crate::converter::calculations::{transpile_to_typst, TranspileContext};

pub fn translate_to_typst(expr: &str, ctx: &TranspileContext) -> String {
    transpile_to_typst(expr, ctx).unwrap_or_else(|e| format!("/* Formula Error: {} */", e))
}

pub fn is_aggregation(expr: &str) -> bool {
    let upper = expr.to_uppercase();
    upper.contains("SUM(")
        || upper.contains("AVG(")
        || upper.contains("MIN(")
        || upper.contains("MAX(")
        || upper.contains("COUNT(")
        || upper.contains("IF(")
}
