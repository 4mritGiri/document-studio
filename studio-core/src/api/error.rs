// src/api/error.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub fn error_response(status: StatusCode, message: &str) -> Response {
    let request_id = Uuid::new_v4().to_string();
    tracing::warn!(request_id = %request_id, status = %status, "{}", message);

    (
        status,
        Json(json!({
            "error": message,
            "request_id": request_id
        })),
    )
        .into_response()
}
