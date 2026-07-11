// src/api/middleware.rs

use crate::api::error::error_response;
use crate::config::AppState;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn require_api_key(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let provided = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided != state.api_key.as_str() {
        return error_response(StatusCode::UNAUTHORIZED, "Invalid API Key");
    }

    next.run(req).await
}
