// src/api/v1/router.rs

use crate::api::middleware::require_api_key;
use crate::config::AppState;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::auth::{health, verify_api_key};
use super::document::generate_document;

pub fn router(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/health", get(health))
        .route("/auth/verify", get(verify_api_key));

    let protected = Router::new()
        .route("/generate", post(generate_document))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_api_key,
        ));

    public.merge(protected)
}
