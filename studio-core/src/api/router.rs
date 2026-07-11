// src/api/router.rs

use crate::api::v1;
use crate::config::AppState;

use axum::Router;

pub fn create_router(state: AppState) -> Router<AppState> {
    Router::new().nest("/api/v1", v1::router(state))
}
