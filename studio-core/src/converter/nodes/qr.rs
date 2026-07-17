// src/converter/nodes/qr.rs

use crate::converter::context::{safe_typst_token, wrap_alignment_raw};
use crate::domain::InlineContent;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use serde_json::Value;
use typst::foundations::Bytes;

/// A QR code's pixel pattern IS the data it encodes — unlike a static logo,
/// it cannot be resolved at layout-generation time and baked into the
/// cached layout, or every request that hits a cache HIT on this layout
/// would get the FIRST request's QR code regardless of its own data (e.g.
/// every invoice after the first would embed the wrong payment QR). So
/// `render_qr_placeholder` only reserves an asset path and records what
/// needs to be resolved later; the actual bytes get generated fresh on
/// every request by `resolve_all` (called after the cache lookup, in
/// `engines/typst/mod.rs`), never cached.
///
/// This is now the ONLY QR rendering path — the old `Node::QrCode` variant
/// is kept in the domain model for backward compatibility, but
/// `builder.rs` converts it into a call to this same function instead of
/// using its own separate (and previously cache-unsafe, unvalidated-size)
/// implementation. Don't reintroduce a second QR code path; route
/// everything through here.
#[derive(Debug, Clone)]
pub struct QrRequest {
    pub asset_path: String,
    pub content: Vec<InlineContent>,
    pub fill: Option<String>,
    pub background: Option<String>,
    pub error_correction: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn render_qr_placeholder(
    content: &[InlineContent],
    size: &Option<String>,
    fill: &Option<String>,
    background: &Option<String>,
    error_correction: &Option<String>,
    alignment: &Option<String>,
    qr_requests: &mut Vec<QrRequest>,
) -> String {
    let asset_path = format!("__qr_{}.svg", qr_requests.len());
    qr_requests.push(QrRequest {
        asset_path: asset_path.clone(),
        content: content.to_vec(),
        fill: fill.clone(),
        background: background.clone(),
        error_correction: error_correction.clone(),
    });

    // size was previously interpolated raw — same injection class fixed
    // everywhere else in this pass. See context.rs's safe_typst_token.
    let size = safe_typst_token(size.as_deref().unwrap_or("3cm"), "3cm");
    let expr = format!(
        "image(\"{}\", format: \"svg\", width: {})",
        asset_path, size
    );
    format!(
        "{}\n\n",
        wrap_alignment_raw(&format!("#{}", expr), alignment)
    )
}

/// Resolves every recorded QR request against the REAL, live request data
/// and generates its SVG bytes. Call this once per request, after retrieving
/// (or generating) the cached layout — never cache the result of this.
pub fn resolve_all(requests: &[QrRequest], data: &Value) -> Result<Vec<(String, Bytes)>, String> {
    let mut out = Vec::with_capacity(requests.len());
    for req in requests {
        let text = resolve_plain(&req.content, data);
        let svg_bytes = generate_qr_svg(&text, &req.fill, &req.background, &req.error_correction)?;
        out.push((req.asset_path.clone(), Bytes::new(svg_bytes)));
    }
    Ok(out)
}

/// Resolves InlineContent into a plain (unescaped) string — this is the
/// literal payload to encode into the QR code, not Typst markup, so none of
/// the Typst-escaping rules that apply elsewhere apply here. Also reused
/// directly by the HTML engine, which has no caching layer and so can
/// resolve+generate QR codes inline without the split used above.
pub fn resolve_plain(items: &[InlineContent], data: &Value) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            InlineContent::Text(t) => out.push_str(&t.text),
            InlineContent::Variable(v) => {
                if let Some(val) = get_value_by_path(data, &v.key) {
                    out.push_str(&plain_value(val));
                }
            }
            // A page number has no meaning inside a QR payload.
            InlineContent::PageNumber(_) => {}
        }
    }
    out
}

fn get_value_by_path<'a>(data: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = data;
    for part in path.split('.') {
        match current {
            Value::Object(map) => current = map.get(part)?,
            Value::Array(arr) => current = arr.get(part.parse::<usize>().ok()?)?,
            _ => return None,
        }
    }
    Some(current)
}

fn plain_value(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}

fn is_safe_color(c: &str) -> bool {
    !c.is_empty() && c.len() <= 32 && c.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '#')
}

pub fn generate_qr_svg(
    text: &str,
    fill: &Option<String>,
    background: &Option<String>,
    error_correction: &Option<String>,
) -> Result<Vec<u8>, String> {
    if text.is_empty() {
        return Err("QR code content resolved to an empty string".to_string());
    }
    if text.len() > crate::config::MAX_QR_PAYLOAD_BYTES {
        return Err(format!(
            "QR code content exceeds the maximum of {} bytes",
            crate::config::MAX_QR_PAYLOAD_BYTES
        ));
    }

    let ec = match error_correction.as_deref() {
        Some("low") => EcLevel::L,
        Some("quartile") => EcLevel::Q,
        Some("high") => EcLevel::H,
        _ => EcLevel::M,
    };

    let code = QrCode::with_error_correction_level(text.as_bytes(), ec)
        .map_err(|e| format!("QR encoding failed: {:?}", e))?;

    let dark = fill.as_deref().unwrap_or("#000000");
    let dark = if is_safe_color(dark) { dark } else { "#000000" };
    let light = background.as_deref().unwrap_or("#ffffff");
    let light = if is_safe_color(light) {
        light
    } else {
        "#ffffff"
    };

    let svg_string = code
        .render()
        .dark_color(svg::Color(dark))
        .light_color(svg::Color(light))
        .build();

    Ok(svg_string.into_bytes())
}
