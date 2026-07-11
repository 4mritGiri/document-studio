// src/engines/typst/fonts.rs

use fontdb::{Database, Source as FontSource};
use std::sync::{Arc, OnceLock};
use typst::foundations::Bytes;
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;

pub struct FontAssets {
    pub book: LazyHash<FontBook>,
    pub fonts: Vec<Arc<Font>>,
}

static FONT_ASSETS: OnceLock<FontAssets> = OnceLock::new();

pub fn font_assets() -> &'static FontAssets {
    FONT_ASSETS.get_or_init(|| {
        let mut font_db = Database::new();
        font_db.load_system_fonts();

        let custom_font_dir =
            std::env::var("STUDIO_FONTS_DIR").unwrap_or_else(|_| "./fonts".to_string());

        if std::path::Path::new(&custom_font_dir).exists() {
            tracing::info!("Loading custom fonts from: {}", custom_font_dir);

            if let Ok(entries) = std::fs::read_dir(&custom_font_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext = ext.to_string_lossy().to_lowercase();
                            if ext == "ttf" || ext == "otf" || ext == "ttc" {
                                if let Err(e) = font_db.load_font_file(&path) {
                                    tracing::warn!("Failed to load font file {:?}: {}", path, e);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            tracing::warn!("Custom font directory not found: {}", custom_font_dir);
        }

        let mut font_book = FontBook::new();
        let mut fonts = Vec::new();

        for face in font_db.faces() {
            if let FontSource::File(path) = &face.source {
                if let Ok(font_data) = std::fs::read(path) {
                    let bytes = Bytes::new(font_data);
                    if let Some(font) = Font::new(bytes, face.index) {
                        font_book.push(font.info().clone());
                        fonts.push(Arc::new(font));
                    }
                }
            }
        }

        FontAssets {
            book: LazyHash::new(font_book),
            fonts,
        }
    })
}
