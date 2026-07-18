// src/converter/nodes/chart.rs

use crate::converter::context::{safe_typst_token, wrap_alignment_raw};
use crate::domain::ChartDataPoint;
use std::collections::HashMap;
use typst::foundations::Bytes;

pub fn render_chart(
    chart_type: &str,
    title: &Option<String>,
    data: &[ChartDataPoint],
    width: &Option<String>,
    height: &Option<String>,
    colors: &Option<Vec<String>>,
    alignment: &Option<String>,
    assets: &mut HashMap<String, Bytes>,
) -> Result<String, String> {
    let title_str = title.as_deref().unwrap_or("Chart");
    let svg_bytes =
        crate::engines::graphics::chart::render_chart_svg(chart_type, title_str, data, colors)?;

    let asset_path = format!("__chart_{}.svg", assets.len());
    assets.insert(asset_path.clone(), Bytes::new(svg_bytes));

    let w = safe_typst_token(width.as_deref().unwrap_or("10cm"), "10cm");
    let h = safe_typst_token(height.as_deref().unwrap_or("6cm"), "6cm");

    let img_expr = format!("image(\"{}\", width: {}, height: {})", asset_path, w, h);
    Ok(format!(
        "{}\n\n",
        wrap_alignment_raw(&format!("#{}", img_expr), alignment)
    ))
}
