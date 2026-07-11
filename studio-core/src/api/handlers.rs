// src/api/handlers.rs

use crate::api::error::error_response;
use crate::config::{AppState, RENDER_TIMEOUT};
use crate::domain::{DocumentRequest, OutputFormat};
use crate::engines::DocumentEngine;
use axum::{
    body::Bytes as AxumBytes,
    extract::{Json, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn generate_document(
    State(state): State<AppState>,
    Json(payload): Json<DocumentRequest>,
) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();

    let permit = match state.render_semaphore.clone().try_acquire_owned() {
        Ok(p) => p,
        Err(_) => return error_response(StatusCode::SERVICE_UNAVAILABLE, "Server at capacity"),
    };

    // 1. Select the engine based on the requested format
    let engine: Arc<dyn DocumentEngine> = match payload.format {
        OutputFormat::Pdf => state.typst_engine.clone(),
        OutputFormat::Html => state.html_engine.clone(),
        OutputFormat::Docx => {
            return error_response(StatusCode::NOT_IMPLEMENTED, "DOCX format not yet supported")
        }
    };

    // 2. Render using the unified trait
    let result = tokio::time::timeout(
        RENDER_TIMEOUT,
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            engine.render(&payload)
        }),
    )
    .await;

    // 3. Handle the unified RenderOutput
    match result {
        Err(_) => error_response(StatusCode::GATEWAY_TIMEOUT, "Generation timed out"),
        Ok(Err(e)) => {
            tracing::error!("Task panic: {}", e);
            error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
        }
        Ok(Ok(Err(e))) => {
            tracing::warn!("Render failed: {}", e);
            error_response(StatusCode::UNPROCESSABLE_ENTITY, &e)
        }
        Ok(Ok(Ok(output))) => {
            tracing::info!(request_id = %request_id, size = output.bytes.len(), format = %output.mime_type, "Success");

            let disposition = format!("inline; filename=\"{}\"", output.suggested_filename);

            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, output.mime_type),
                    (header::CONTENT_DISPOSITION, disposition),
                ],
                AxumBytes::from(output.bytes),
            )
                .into_response()
        }
    }
}
