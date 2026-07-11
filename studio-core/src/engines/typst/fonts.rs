// src/engine/fonts.rs

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
        let mut font_book = FontBook::new();
        let mut fonts = Vec::new();
        for face in font_db.faces() {
            if let FontSource::File(path) = &face.source {
                if let Ok(font_data) = std::fs::read(path) {
                    if let Some(font) = Font::new(Bytes::new(font_data), face.index) {
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
