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
    width: &str,
    height: &str,
    fill: &Option<String>,
    rotate: &Option<String>,
) -> String {
    let fill_expr = color_expr(fill.as_deref().unwrap_or("black"));
    let shape = match kind {
        "triangle" => {
            format!("polygon(fill: {fill_expr}, (0pt, 0pt), ({width}, 0pt), (0pt, {height}))")
        }
        "circle" => {
            format!("circle(width: {width}, height: {height}, fill: {fill_expr}, stroke: none)")
        }
        _ => format!("rect(width: {width}, height: {height}, fill: {fill_expr}, stroke: none)"),
    };
    match rotate {
        Some(r) => format!("#rotate({})[#{}]\n\n", r, shape),
        None => format!("#{}\n\n", shape),
    }
}
