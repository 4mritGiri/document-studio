// src/config/mod.rs

use crate::engines::{html::HtmlEngine, typst::TypstEngine};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

// --- Production tunables ---
pub const MAX_BODY_BYTES: usize = 25 * 1024 * 1024;
pub const MAX_CONCURRENT_RENDERS: usize = 4;
pub const RENDER_TIMEOUT: Duration = Duration::from_secs(30);

pub const MAX_IMAGE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_NODE_DEPTH: usize = 32;
pub const MAX_TABLE_LOOP_ROWS: usize = 5_000;
pub const HTTP_FETCH_TIMEOUT: Duration = Duration::from_secs(5);

pub const MAX_QR_PAYLOAD_BYTES: usize = 2_000;

#[derive(Clone)]
pub struct AppState {
    pub typst_engine: Arc<TypstEngine>,
    pub html_engine: Arc<HtmlEngine>,
    pub render_semaphore: Arc<Semaphore>,
    pub api_key: Arc<String>,
}
