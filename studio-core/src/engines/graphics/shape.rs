// src/engines/graphics/shape.rs

pub fn render_shape_svg(
    kind: &str,
    path_data: &Option<String>,
    width: &str,
    height: &str,
    fill: &Option<String>,
    stroke: &Option<String>,
    stroke_width: &Option<String>,
    rotate: &Option<String>,
) -> Result<Vec<u8>, String> {
    let fill_attr = fill.as_deref().unwrap_or("none");
    let stroke_attr = stroke.as_deref().unwrap_or("none");
    let sw_attr = stroke_width.as_deref().unwrap_or("1");
    let rotate_attr = rotate.as_deref().unwrap_or("0");

    // Add rotation transform around the center (50, 50) of the 100x100 viewBox
    let transform = if rotate_attr != "0" {
        format!(r#" transform="rotate({} 50 50)""#, rotate_attr)
    } else {
        String::new()
    };

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="{}" height="{}">"#,
        width, height
    );

    let shape_tag = match kind.to_lowercase().as_str() {
        "rect" => format!(
            r#"<rect x="10" y="10" width="80" height="80" fill="{}" stroke="{}" stroke-width="{}"{}/>"#,
            fill_attr, stroke_attr, sw_attr, transform
        ),
        "circle" => format!(
            r#"<circle cx="50" cy="50" r="40" fill="{}" stroke="{}" stroke-width="{}"{}/>"#,
            fill_attr, stroke_attr, sw_attr, transform
        ),
        "triangle" => format!(
            r#"<polygon points="50,10 90,90 10,90" fill="{}" stroke="{}" stroke-width="{}"{}/>"#,
            fill_attr, stroke_attr, sw_attr, transform
        ),
        "path" => {
            if let Some(d) = path_data {
                format!(
                    r#"<path d="{}" fill="{}" stroke="{}" stroke-width="{}"{}/>"#,
                    d, fill_attr, stroke_attr, sw_attr, transform
                )
            } else {
                return Err("Path shape requires 'path_data'".to_string());
            }
        }
        _ => return Err(format!("Unsupported shape kind: {}", kind)),
    };

    svg.push_str(&shape_tag);
    svg.push_str("</svg>");

    Ok(svg.into_bytes())
}
