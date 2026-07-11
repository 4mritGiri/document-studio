// src/engine/compiler.rs

use super::world::SimpleWorld;
use std::collections::HashMap;
use typst::foundations::Bytes;
use typst::syntax::Source;
use typst_pdf::{pdf, PdfOptions};

pub fn render_pdf(typst_markup: &str, assets: HashMap<String, Bytes>) -> Result<Vec<u8>, String> {
    let source = Source::detached(typst_markup.to_string());
    let world = SimpleWorld::new(source, assets);
    let result = typst::compile(&world);
    if !result.warnings.is_empty() {
        tracing::debug!("Typst warnings: {:?}", result.warnings);
    }
    let doc = result
        .output
        .map_err(|e| format!("Compilation error: {:?}", e))?;
    pdf(&doc, &PdfOptions::default()).map_err(|e| format!("PDF export error: {:?}", e))
}
