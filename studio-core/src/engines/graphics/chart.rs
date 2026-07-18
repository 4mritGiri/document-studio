// src/engines/graphics/chart.rs

use crate::domain::ChartDataPoint;

/// Parses a hex color string, ensuring it's valid.
fn parse_color(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let (Ok(_), Ok(_), Ok(_)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return format!("#{}", hex);
        }
    }
    "#3b82f6".to_string() // Fallback to Tailwind Blue-500
}

/// Resolves the color palette, cycling through user colors or defaults.
fn get_colors(data_len: usize, user_colors: &Option<Vec<String>>) -> Vec<String> {
    let default_palette = [
        "#3b82f6", "#10b981", "#f59e0b", "#ef4444", "#8b5cf6", "#ec4899",
    ];
    let base: Vec<String> = if let Some(c) = user_colors {
        c.iter().map(|x| parse_color(x)).collect()
    } else {
        default_palette.iter().map(|s| s.to_string()).collect()
    };
    (0..data_len)
        .map(|i| base[i % base.len()].clone())
        .collect()
}

pub fn render_chart_svg(
    chart_type: &str,
    title: &str,
    data: &[ChartDataPoint],
    colors: &Option<Vec<String>>,
) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("Chart data is empty".to_string());
    }

    let mut svg = String::new();
    svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 400" width="100%" height="100%">"#);
    svg.push_str(r#"<rect width="600" height="400" fill="white"/>"#);

    svg.push_str(&format!(
        r##"<text x="300" y="30" font-family="sans-serif" font-size="20" font-weight="bold" text-anchor="middle" fill="#1f2937">{}</text>"##,
        title.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    ));

    let chart_colors = get_colors(data.len(), colors);
    let max_val = data.iter().map(|d| d.value).fold(0.0f64, f64::max) * 1.2;
    if max_val == 0.0 {
        return Err("Chart data values are all zero".to_string());
    }

    // Chart area bounds
    let left = 60.0;
    let right = 580.0;
    let top = 50.0;
    let bottom = 340.0;
    let width = right - left;
    let height = bottom - top;

    match chart_type.to_lowercase().as_str() {
        "bar" => {
            let n = data.len() as f64;
            let slot_width = width / n;
            let bar_width = slot_width * 0.6;

            for (i, d) in data.iter().enumerate() {
                let x = left + (i as f64 * slot_width) + (slot_width - bar_width) / 2.0;
                let bar_height = (d.value / max_val) * height;
                let y = bottom - bar_height;
                let color = &chart_colors[i];

                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="4"/>"#,
                    x, y, bar_width, bar_height, color
                ));

                // X-axis label
                svg.push_str(&format!(
                    r##"<text x="{}" y="{}" font-family="sans-serif" font-size="12" text-anchor="middle" fill="#4b5563">{}</text>"##,
                    x + bar_width / 2.0, bottom + 20.0, d.label.replace('&', "&amp;")
                ));

                // Value label on top of bar
                svg.push_str(&format!(
                    r##"<text x="{}" y="{}" font-family="sans-serif" font-size="12" font-weight="bold" text-anchor="middle" fill="#1f2937">{:.1}</text>"##,
                    x + bar_width / 2.0, y - 5.0, d.value
                ));
            }
        }
        "line" => {
            let n = data.len();
            let step = if n > 1 { width / (n - 1) as f64 } else { 0.0 };
            let color = &chart_colors[0];

            let mut points = String::new();
            for (i, d) in data.iter().enumerate() {
                let x = left + (i as f64 * step);
                let y = bottom - (d.value / max_val) * height;
                points.push_str(&format!("{},{} ", x, y));

                svg.push_str(&format!(
                    r##"<text x="{}" y="{}" font-family="sans-serif" font-size="12" text-anchor="middle" fill="#4b5563">{}</text>"##,
                    x, bottom + 20.0, d.label.replace('&', "&amp;")
                ));

                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="5" fill="{}" stroke="white" stroke-width="2"/>"#,
                    x, y, color
                ));
            }

            svg.push_str(&format!(
                r#"<polyline points="{}" fill="none" stroke="{}" stroke-width="3" stroke-linejoin="round"/>"#,
                points.trim(), color
            ));
        }
        "pie" => {
            let total: f64 = data.iter().map(|d| d.value).sum();
            if total == 0.0 {
                return Err("Pie chart total is zero".to_string());
            }

            let cx = 300.0;
            let cy = 220.0;
            let r = 130.0;
            let mut start_angle = -90.0; // Start at top

            for (i, d) in data.iter().enumerate() {
                let slice_angle = (d.value / total) * 360.0;
                let end_angle = start_angle + slice_angle;
                let color = &chart_colors[i];

                let start_rad = start_angle.to_radians();
                let end_rad = end_angle.to_radians();

                let x1 = cx + r * start_rad.cos();
                let y1 = cy + r * start_rad.sin();
                let x2 = cx + r * end_rad.cos();
                let y2 = cy + r * end_rad.sin();

                let large_arc = if slice_angle > 180.0 { 1 } else { 0 };

                let path = format!(
                    "M {} {} L {} {} A {} {} 0 {} 1 {} {} Z",
                    cx, cy, x1, y1, r, r, large_arc, x2, y2
                );

                svg.push_str(&format!(
                    r#"<path d="{}" fill="{}" stroke="white" stroke-width="2"/>"#,
                    path, color
                ));

                // Percentage label inside the slice
                let mid_angle = (start_angle + end_angle) / 2.0;
                let mid_rad = mid_angle.to_radians();
                let label_r = r * 0.65;
                let lx = cx + label_r * mid_rad.cos();
                let ly = cy + label_r * mid_rad.sin();

                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-family="sans-serif" font-size="12" font-weight="bold" text-anchor="middle" fill="white">{:.1}%</text>"#,
                    lx, ly, (d.value / total) * 100.0
                ));

                start_angle = end_angle;
            }
        }
        _ => return Err(format!("Unsupported chart type: {}", chart_type)),
    }

    svg.push_str("</svg>");
    Ok(svg.into_bytes())
}

//
//
//
// // src/engines/graphics/chart.rs

// use crate::domain::ChartDataPoint;
// use plotters::prelude::*;

// /// Parses a hex color string (e.g., "#1f2937") into a plotters RGBColor.
// fn parse_color(hex: &str) -> RGBColor {
//     let hex = hex.trim_start_matches('#');
//     if hex.len() == 6 {
//         if let (Ok(r), Ok(g), Ok(b)) = (
//             u8::from_str_radix(&hex[0..2], 16),
//             u8::from_str_radix(&hex[2..4], 16),
//             u8::from_str_radix(&hex[4..6], 16),
//         ) {
//             return RGBColor(r, g, b);
//         }
//     }
//     // Fallback to a nice default blue if parsing fails
//     RGBColor(59, 130, 246)
// }

// /// Resolves the final color palette, cycling through user colors or falling back to defaults.
// fn get_colors(data_len: usize, user_colors: &Option<Vec<String>>) -> Vec<RGBColor> {
//     let default_palette = [
//         RGBColor(59, 130, 246), // Blue
//         RGBColor(16, 185, 129), // Green
//         RGBColor(245, 158, 11), // Amber
//         RGBColor(239, 68, 68),  // Red
//         RGBColor(139, 92, 246), // Purple
//         RGBColor(236, 72, 153), // Pink
//     ];

//     let base_colors: Vec<RGBColor> = if let Some(colors) = user_colors {
//         colors.iter().map(|c| parse_color(c)).collect()
//     } else {
//         default_palette.to_vec()
//     };

//     // Cycle through the colors to ensure we have enough for all data points
//     (0..data_len)
//         .map(|i| base_colors[i % base_colors.len()].clone())
//         .collect()
// }

// pub fn render_chart_svg(
//     chart_type: &str,
//     title: &str,
//     data: &[ChartDataPoint],
//     colors: &Option<Vec<String>>,
// ) -> Result<Vec<u8>, String> {
//     if data.is_empty() {
//         return Err("Chart data is empty".to_string());
//     }

//     let mut svg_string = String::new();
//     {
//         let root = SVGBackend::new(&mut svg_string, (600, 400)).into_drawing_area();
//         root.fill(&WHITE).map_err(|e| e.to_string())?;

//         let max_val = data.iter().map(|d| d.value).fold(0.0f64, f64::max) * 1.2;
//         if max_val == 0.0 {
//             return Err("Chart data values are all zero".to_string());
//         }

//         // Standard, compatible font declaration for plotters
//         let font = ("sans-serif", 25).into_font();

//         let mut chart = ChartBuilder::on(&root)
//             .caption(title, font)
//             .margin(20)
//             .x_label_area_size(40)
//             .y_label_area_size(50)
//             .build_cartesian_2d(0f32..data.len() as f32, 0f32..max_val as f32)
//             .map_err(|e| e.to_string())?;

//         chart.configure_mesh().draw().map_err(|e| e.to_string())?;

//         match chart_type.to_lowercase().as_str() {
//             "bar" => {
//                 let chart_colors = get_colors(data.len(), colors);
//                 chart
//                     .draw_series(data.iter().enumerate().map(|(i, d)| {
//                         Rectangle::new(
//                             [(i as f32 - 0.4, 0.0), (i as f32 + 0.4, d.value as f32)],
//                             chart_colors[i].filled(),
//                         )
//                     }))
//                     .map_err(|e| e.to_string())?;
//             }
//             "line" => {
//                 let chart_colors = get_colors(1, colors);
//                 let series_color = &chart_colors[0];
//                 chart
//                     .draw_series(LineSeries::new(
//                         data.iter()
//                             .enumerate()
//                             .map(|(i, d)| (i as f32, d.value as f32)),
//                         series_color,
//                     ))
//                     .map_err(|e| e.to_string())?;
//             }
//             "pie" => {
//                 let total: f64 = data.iter().map(|d| d.value).sum();
//                 if total == 0.0 {
//                     return Err("Pie chart total is zero".to_string());
//                 }

//                 let center = (300, 200);
//                 let radius = 150.0;
//                 let sizes: Vec<f64> = data.iter().map(|d| d.value / total).collect();
//                 let chart_colors = get_colors(data.len(), colors);
//                 let labels: Vec<&str> = data.iter().map(|d| d.label.as_str()).collect();

//                 let pie = Pie::new(&center, &radius, &sizes, &chart_colors, &labels);
//                 root.draw(&pie).map_err(|e| e.to_string())?;
//             }
//             _ => return Err(format!("Unsupported chart type: {}", chart_type)),
//         }

//         chart
//             .configure_series_labels()
//             .draw()
//             .map_err(|e| e.to_string())?;
//         root.present().map_err(|e| e.to_string())?;
//     }

//     Ok(svg_string.into_bytes())
// }
