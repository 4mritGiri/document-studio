// src/engines/typst/mod.rs

pub mod compiler;
pub mod fonts;
pub mod security;
pub mod world;

use crate::converter::builder::json_to_typst;
use crate::converter::nodes::qr::{self, QrRequest};
use crate::domain::DocumentRequest;
use crate::engines::{DocumentEngine, RenderOutput};
use moka::sync::Cache;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::OnceLock;
use typst::foundations::Bytes;

// The cached value is purely structural: markup + static assets (logos,
// letterhead shapes) + a list of QR *requests* (not resolved QR bytes).
// QR requests get resolved against live request data on every call to
// `render`, after the cache lookup — see the comment on `qr::QrRequest`
// for why this split exists.
type CachedLayout = (String, HashMap<String, Bytes>, Vec<QrRequest>);

// Global LRU Cache for compiled layouts (holds up to 100 templates)
static LAYOUT_CACHE: OnceLock<Cache<String, CachedLayout>> = OnceLock::new();

fn get_cache() -> &'static Cache<String, CachedLayout> {
    LAYOUT_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build()
    })
}

pub struct TypstEngine;

impl DocumentEngine for TypstEngine {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String> {
        let layout_key =
            serde_json::to_string(&(&request.content, &request.page)).map_err(|e| e.to_string())?;
        let mut hasher = Sha256::new();
        hasher.update(layout_key.as_bytes());
        let layout_hash = format!("{:x}", hasher.finalize());

        let (layout_markup, mut assets, qr_requests) = match get_cache().get(&layout_hash) {
            Some(cached) => {
                tracing::debug!("Cache HIT for layout {}", layout_hash);
                cached
            }
            None => {
                tracing::debug!("Cache MISS for layout {}", layout_hash);
                let result = json_to_typst(&request.content, &request.data, &request.page)?;
                get_cache().insert(layout_hash.clone(), result.clone());
                result
            }
        };

        // Resolve every QR code against the REAL request data, every single
        // request — this step is deliberately outside the cache. A QR
        // code's pixel pattern is data, not layout; caching it here would
        // mean every request after the first cache hit gets the first
        // request's QR code (e.g. every subsequent invoice would embed the
        // first invoice's payment QR — a wrong-amount/wrong-reference bug,
        // not a cosmetic one).
        for (asset_path, svg_bytes) in qr::resolve_all(&qr_requests, &request.data)? {
            assets.insert(asset_path, svg_bytes);
        }
        for node in &request.content {
            match node {
                crate::domain::Node::Chart {
                    chart_type,
                    title,
                    data,
                    width: _,
                    height: _,
                    colors,
                } => {
                    let title_str = title.as_deref().unwrap_or("Chart");
                    let svg_bytes = crate::engines::graphics::chart::render_chart_svg(
                        chart_type, title_str, data, colors,
                    )?;
                    let asset_path = format!("__chart_{}.svg", assets.len());
                    assets.insert(asset_path, Bytes::new(svg_bytes));
                }
                crate::domain::Node::Shape {
                    kind,
                    path_data,
                    width,
                    height,
                    fill,
                    stroke,
                    stroke_width,
                    rotate,
                } => {
                    if kind.to_lowercase() == "path" {
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
                        assets.insert(asset_path, Bytes::new(svg_bytes));
                    }
                }
                _ => {}
            }
        }

        let data_json = serde_json::to_string(&request.data).map_err(|e| e.to_string())?;
        assets.insert("data.json".to_string(), Bytes::new(data_json.into_bytes()));

        let final_markup = format!("#let data = json(\"data.json\")\n{}", layout_markup);

        // 5. Compile to PDF (pass the fully populated assets map)
        let raw_pdf_bytes = compiler::render_pdf(&final_markup, assets)?;

        // 6. Apply Security/Encryption if configured
        let final_bytes = if let Some(sec_config) = &request.security {
            tracing::info!("Applying AES-256 PDF encryption...");
            security::apply_security(raw_pdf_bytes, sec_config)?
        } else {
            raw_pdf_bytes
        };

        Ok(RenderOutput {
            bytes: final_bytes,
            mime_type: "application/pdf".to_string(),
            suggested_filename: format!("{}.pdf", request.template_id),
        })
    }
}
