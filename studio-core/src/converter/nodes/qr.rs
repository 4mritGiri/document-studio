// src/converter/nodes/qr.rs

use crate::converter::context::wrap_alignment_raw;
use qrcode::QrCode;
use std::collections::HashMap;
use typst::foundations::Bytes;

pub fn render_qr_code(
    data: &str,
    width: &Option<String>,
    alignment: &Option<String>,
    assets: &mut HashMap<String, Bytes>,
) -> Result<String, String> {
    // 1. Generate the QR Code matrix
    let code =
        QrCode::new(data.as_bytes()).map_err(|e| format!("QR code generation failed: {}", e))?;

    let matrix_size = code.width();

    // 2. Build a highly optimized SVG string.
    // Instead of creating thousands of <rect> elements, we use a single <path>
    // with move/draw commands. This keeps the file size tiny and rendering instant.
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" shape-rendering="crispEdges">"#,
        size = matrix_size
    );

    let mut path_data = String::new();
    for y in 0..matrix_size {
        for x in 0..matrix_size {
            if code[(x, y)] == qrcode::Color::Dark {
                // M (move to) x y, h1 (draw horizontal 1), v1 (draw vertical 1), h-1 (close back)
                path_data.push_str(&format!("M{} {}h1v1h-1z", x, y));
            }
        }
    }
    svg.push_str(&format!(r#"<path d="{}" fill="black"/>"#, path_data));
    svg.push_str("</svg>");

    // 3. Inject the SVG into the virtual assets map
    let virtual_path = format!("__asset_qr_{}.svg", assets.len());
    assets.insert(virtual_path.clone(), Bytes::new(svg.into_bytes()));

    // 4. Generate Typst markup
    let width_val = width.as_deref().unwrap_or("2.5cm");
    let img_expr = format!("image(\"{}\", width: {})", virtual_path, width_val);

    Ok(format!(
        "{}\n\n",
        wrap_alignment_raw(&format!("#{}", img_expr), alignment)
    ))
}
