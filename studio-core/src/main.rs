// src/main.rs

use axum::extract::DefaultBodyLimit;
use axum::http::Response;
use owo_colors::OwoColorize;
use std::sync::Arc;
use std::time::Duration;
use studio_core::{
    api::router::create_router,
    config::{AppState, MAX_BODY_BYTES, MAX_CONCURRENT_RENDERS},
};
use tokio::sync::Semaphore;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::trace::{DefaultMakeSpan, OnResponse, TraceLayer};
use tracing::Level;
use tracing::Span;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logging();

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

    let app = create_router(state.clone())
        .with_state(state)
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::INFO)
                        .include_headers(false),
                )
                .on_response(PrettyResponseLogger),
        );

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Invalid PORT value");
            std::process::exit(1);
        });

    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap_or_else(|err| {
            tracing::error!(
                error = %err,
                address = %address,
                "Failed to bind server"
            );
            std::process::exit(1);
        });

    print_banner(&address);

    tracing::info!("Document Studio Engine started successfully");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "studio_core=info,tower_http=info".into()),
        )
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(true)
        .with_timer(LocalTime::rfc_3339())
        .compact()
        .init();
}

#[derive(Clone, Copy)]
struct PrettyResponseLogger;

impl<B> OnResponse<B> for PrettyResponseLogger {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        let status = response.status();

        if status.is_success() {
            tracing::info!(
                status = %status,
                latency_ms = latency.as_millis(),
                "request completed"
            );
        } else if status.is_client_error() {
            tracing::warn!(
                status = %status,
                latency_ms = latency.as_millis(),
                "client error"
            );
        } else {
            tracing::error!(
                status = %status,
                latency_ms = latency.as_millis(),
                "server error"
            );
        }
    }
}

fn print_banner(address: &str) {
    // let url = format!("http://{}", address);
    println!();

    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════════════╗".bright_blue()
    );

    println!(
        "{}",
        "║                     Document Studio Engine                        ║".bright_blue()
    );

    println!(
        "{}",
        "╠════════════════════════════════════════════════════════════════════╣".bright_blue()
    );

    println!(
        "{}",
        "║ Version      │ v1                                                 ║".cyan()
    );

    println!(
        "{}",
        "║ Environment  │ Development                                        ║".cyan()
    );

    println!(
        "{}",
        format!("║ Listening    │ http://{:<34}║", address).cyan()
    );

    println!(
        "{}",
        "║ Health       │ GET  /api/v1/health                                ║".cyan()
    );

    println!(
        "{}",
        "║ Generate     │ POST /api/v1/generate                              ║".cyan()
    );

    println!(
        "{}",
        "║ Auth Verify  │ GET  /api/v1/auth/verify                           ║".cyan()
    );

    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════════════╝".bright_blue()
    );

    println!();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C received");
        }

        _ = terminate => {
            tracing::info!("SIGTERM received");
        }
    }

    tracing::info!("Graceful shutdown started");
}
