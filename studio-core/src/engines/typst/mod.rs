// src/engines/typst/mod.rs

pub mod compiler;
pub mod fonts;
pub mod security;
pub mod world;

use crate::converter::builder::json_to_typst;
use crate::converter::nodes::qr::{self, QrRequest};
use crate::converter::nodes::table::{self, TableRequest};
use crate::domain::DocumentRequest;
use crate::engines::{DocumentEngine, RenderOutput};
use moka::sync::Cache;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::OnceLock;
use typst::foundations::Bytes;

// The cached value is purely structural: markup + static assets (logos,
// letterhead shapes) + a list of QR *requests* + a list of Table
// *requests* (never resolved values — see QrRequest's and TableRequest's
// doc comments for why). Both QR payloads and table loop-rows/footer
// aggregations are resolved against live request data on every call to
// `render`, strictly after the cache lookup.
type CachedLayout = (
    String,
    HashMap<String, Bytes>,
    Vec<QrRequest>,
    Vec<TableRequest>,
);

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

        let (layout_markup, mut assets, qr_requests, table_requests) =
            match get_cache().get(&layout_hash) {
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

        // Resolve every QR code against the REAL request data, every
        // single request — deliberately outside the cache. See
        // qr::QrRequest's doc comment.
        for (asset_path, svg_bytes) in qr::resolve_all(&qr_requests, &request.data)? {
            assets.insert(asset_path, svg_bytes);
        }

        // Resolve every deferred table fragment (loop-generated rows,
        // footer aggregations) against the REAL request data, every
        // single request — deliberately outside the cache. This is the
        // fix for a real, reproduced bug: previously these were resolved
        // while building the (cached) layout, so a cache hit on a shared
        // template would silently replay the FIRST request's row values —
        // e.g. one customer's invoice line items and totals appearing on
        // a different customer's document. See table::TableRequest's doc
        // comment for the full explanation.
        let mut layout_markup = layout_markup;
        for (token, rendered) in table::resolve_deferred(&table_requests, &request.data) {
            layout_markup = layout_markup.replacen(&token, &rendered, 1);
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

        // Compile to PDF (pass the fully populated assets map)
        let raw_pdf_bytes = compiler::render_pdf(&final_markup, assets)?;

        // Apply Security/Encryption if configured
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
