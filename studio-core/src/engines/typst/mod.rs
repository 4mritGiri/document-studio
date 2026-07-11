// src/engines/typst/mod.rs

pub mod compiler;
pub mod fonts;
pub mod world;

use crate::converter::builder::json_to_typst;
use crate::domain::DocumentRequest;
use crate::engines::{DocumentEngine, RenderOutput};
use moka::sync::Cache;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::OnceLock;
use typst::foundations::Bytes;

type CachedLayout = (String, HashMap<String, Bytes>);

// Global LRU Cache for compiled layouts (holds up to 100 templates)
static LAYOUT_CACHE: OnceLock<Cache<String, CachedLayout>> = OnceLock::new();

fn get_cache() -> &'static Cache<String, CachedLayout> {
    LAYOUT_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(100) // Cache 100 unique layouts
            .time_to_live(std::time::Duration::from_secs(3600)) // Expire after 1 hour
            .build()
    })
}

pub struct TypstEngine;

impl DocumentEngine for TypstEngine {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String> {
        // 1. Hash the layout (content + page settings) to get a cache key
        let layout_key =
            serde_json::to_string(&(&request.content, &request.page)).map_err(|e| e.to_string())?;

        let mut hasher = Sha256::new();
        hasher.update(layout_key.as_bytes());
        let layout_hash = format!("{:x}", hasher.finalize());

        // 2. Check cache for the layout markup AND the assets
        let (layout_markup, mut assets) = match get_cache().get(&layout_hash) {
            Some(cached) => {
                tracing::debug!("Cache HIT for layout {}", layout_hash);
                cached // Returns both the markup string and the HashMap of images
            }
            None => {
                tracing::debug!("Cache MISS for layout {}", layout_hash);
                // Generate the layout and collect assets (images, etc.)
                let result = json_to_typst(&request.content, &request.data, &request.page)?;

                // FIX: Cache the entire tuple (markup + assets)
                get_cache().insert(layout_hash.clone(), result.clone());
                result
            }
        };

        // 3. Inject the actual data as a virtual JSON file for Typst
        let data_json = serde_json::to_string(&request.data).map_err(|e| e.to_string())?;
        assets.insert("data.json".to_string(), Bytes::new(data_json.into_bytes()));

        // 4. Prepend the data loader to the cached layout
        let final_markup = format!("#let data = json(\"data.json\")\n{}", layout_markup);

        // 5. Compile to PDF (pass the fully populated assets map)
        let pdf_bytes = compiler::render_pdf(&final_markup, assets)?;

        Ok(RenderOutput {
            bytes: pdf_bytes,
            mime_type: "application/pdf".to_string(),
            suggested_filename: format!("{}.pdf", request.template_id),
        })
    }
}
