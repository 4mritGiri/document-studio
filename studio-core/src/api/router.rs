// src/api/router.rs

use crate::api::error::error_response;
use crate::api::handlers::{generate_document, health};
use crate::config::AppState;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware,
    middleware::Next,
    response::Response,
    routing::{get, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/generate", post(generate_document))
        .route("/health", get(health))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_api_key,
        ))
        .with_state(state)
}

async fn require_api_key(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let provided = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if provided.is_empty() || provided != state.api_key.as_str() {
        // FIX: Removed .into_response() because error_response now returns Response
        return error_response(StatusCode::UNAUTHORIZED, "Invalid API Key");
    }

    next.run(req).await
}
