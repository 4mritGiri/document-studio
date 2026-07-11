// src/api/v1/auth.rs

use crate::api::error::error_response;
use crate::config::AppState;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use serde::Serialize;

pub async fn health() -> &'static str {
    "ok"
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub authenticated: bool,
}

pub async fn verify_api_key(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<VerifyResponse>, axum::response::Response> {
    let provided = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided != state.api_key.as_str() {
        return Err(error_response(StatusCode::UNAUTHORIZED, "Invalid API Key"));
    }

    Ok(Json(VerifyResponse {
        authenticated: true,
    }))
}
