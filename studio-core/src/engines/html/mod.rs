// src/engines/html/mod.rs

#[path = "converter.rs"]
pub mod converter;

use crate::domain::DocumentRequest;
use crate::engines::{DocumentEngine, RenderOutput};

pub struct HtmlEngine;

impl DocumentEngine for HtmlEngine {
    fn render(&self, request: &DocumentRequest) -> Result<RenderOutput, String> {
        let html_body = converter::json_to_html(&request.content, &request.data)?;

        // template_id is client-controlled and lands in <title> — it needs
        // the same escaping as any other untrusted text.
        let safe_title = converter::html_escape(&request.template_id);
        let full_html = format!(
            "<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>{}</title></head><body>{}</body></html>",
            safe_title, html_body
        );

        Ok(RenderOutput {
            bytes: full_html.into_bytes(),
            mime_type: "text/html".to_string(),
            // Also client-controlled, and ends up in a Content-Disposition
            // header — sanitize rather than pass through raw.
            suggested_filename: format!(
                "{}.html",
                converter::sanitize_filename(&request.template_id)
            ),
        })
    }
}
