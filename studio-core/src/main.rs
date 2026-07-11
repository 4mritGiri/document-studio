// src/main.rs

use axum::extract::DefaultBodyLimit;
use std::sync::Arc;
use studio_core::{
    api::router::create_router,
    config::{AppState, MAX_BODY_BYTES, MAX_CONCURRENT_RENDERS},
};
use tokio::sync::Semaphore;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let api_key = std::env::var("DOCUMENT_ENGINE_API_KEY").unwrap_or_else(|_| {
        eprintln!("FATAL: DOCUMENT_ENGINE_API_KEY is not set.");
        std::process::exit(1);
    });

    let state = AppState {
        typst_engine: Arc::new(studio_core::engines::typst::TypstEngine),
        html_engine: Arc::new(studio_core::engines::html::HtmlEngine),
        render_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_RENDERS)),
        api_key: Arc::new(api_key),
    };

    let app = create_router(state)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");
    tracing::info!("🚀 Document Engine listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Ctrl+C");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
    tracing::info!("Shutdown signal received");
}
