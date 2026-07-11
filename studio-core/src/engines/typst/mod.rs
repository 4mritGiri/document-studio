// src/engines/typst/mod.rs

pub mod compiler;
pub mod fonts;
pub mod world;

use crate::converter::builder::json_to_typst;
use crate::domain::DocumentRequest;
use crate::engines::typst::compiler::render_pdf;
use crate::engines::{DocumentEngine, RenderOutput};

pub struct TypstEngine;

impl DocumentEngine for TypstEngine {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String> {
        // 1. Use your existing logic to generate Typst markup and assets
        let (markup, assets) = json_to_typst(&request.content, &request.data, &request.page)?;

        // 2. Compile to PDF
        let pdf_bytes = render_pdf(&markup, assets)?;

        Ok(RenderOutput {
            bytes: pdf_bytes,
            mime_type: "application/pdf".to_string(),
            suggested_filename: format!("{}.pdf", request.template_id),
        })
    }
}
