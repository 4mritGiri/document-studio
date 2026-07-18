// src/engines/mod.rs

pub mod graphics;
pub mod html;
pub mod typst;
// pub mod docx; // Future

use crate::domain::DocumentRequest;

/// The universal output of any rendering engine.
pub struct RenderOutput {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub suggested_filename: String,
}

/// The contract that all document engines (PDF, HTML, DOCX) must implement.
pub trait DocumentEngine: Send + Sync {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String>;
}
