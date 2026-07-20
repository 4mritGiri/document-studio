// src/converter/nodes/media.rs

use crate::assets::resolver::{resolve_image_source, sniff_image_format};
use crate::config::MAX_IMAGE_BYTES;
use crate::converter::context::{color_expr, wrap_alignment_raw};
use std::collections::HashMap;
use typst::foundations::Bytes;

pub fn render_image(
    src: &str,
    width: &Option<String>,
    height: &Option<String>,
    alignment: &Option<String>,
    assets: &mut HashMap<String, Bytes>,
) -> Result<String, String> {
    let bytes = resolve_image_source(src)?;
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err("Image too large".to_string());
    }
    let format = sniff_image_format(&bytes);
    let path = format!("__asset_{}.{}", assets.len(), format);
    assets.insert(path.clone(), bytes);

    let mut args = vec![format!("format: \"{}\"", format)];
    if let Some(w) = width {
        args.push(format!("width: {}", w));
    }
    if let Some(h) = height {
        args.push(format!("height: {}", h));
    }

    Ok(format!(
        "{}\n\n",
        wrap_alignment_raw(
            &format!("#image(\"{}\", {})", path, args.join(", ")),
            alignment
        )
    ))
}

pub fn render_shape(
    kind: &str,
    path_data: &Option<String>,
    width: &str,
    height: &str,
    fill: &Option<String>,
    stroke: &Option<String>,
    stroke_width: &Option<String>,
    rotate: &Option<String>,
    assets: &mut HashMap<String, Bytes>,
) -> Result<String, String> {
    // If it's a custom path or has a stroke, we MUST render it as an SVG asset
    if kind.to_lowercase() == "path" || stroke.is_some() {
        let svg_bytes = crate::engines::graphics::shape::render_shape_svg(
            kind,
            path_data,
            width,
            height,
            fill,
            stroke,
            stroke_width,
            rotate,
        )?;
        let asset_path = format!("__shape_{}.svg", assets.len());
        assets.insert(asset_path.clone(), Bytes::new(svg_bytes));

        let img_expr = format!(
            "image(\"{}\", width: {}, height: {})",
            asset_path, width, height
        );
        return Ok(format!("#{}\n\n", img_expr));
    }

    // Fallback to native Typst shapes for basic rect/circle/triangle without strokes
    let fill_expr = color_expr(fill.as_deref().unwrap_or("black"));
    let shape = match kind.to_lowercase().as_str() {
        "triangle" => {
            format!("polygon(fill: {fill_expr}, (0pt, 0pt), ({width}, 0pt), (0pt, {height}))")
        }
        "circle" => {
            format!("circle(width: {width}, height: {height}, fill: {fill_expr}, stroke: none)")
        }
        _ => format!("rect(width: {width}, height: {height}, fill: {fill_expr}, stroke: none)"),
    };

    match rotate {
        Some(r) => Ok(format!("#rotate({})[#{}]\n\n", r, shape)),
        None => Ok(format!("#{}\n\n", shape)),
    }
}
