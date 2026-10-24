use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::sanitizer::{sanitize_text, SanitizeResult};

/// The job payload queued by Android or CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizeLinkJob {
    pub id: String,
    pub raw_url: String,
    pub submitted_at: u64,
}

impl SanitizeLinkJob {
    pub fn new(raw_url: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("job-{}", now),
            raw_url: raw_url.into(),
            submitted_at: now,
        }
    }
}

/// The Apalis background worker task handler
pub async fn process_sanitize_job(job: SanitizeLinkJob) -> Result<SanitizeResult, Box<dyn std::error::Error + Send + Sync>> {
    info!("[Apalis Worker] Starting job {}: {}", job.id, job.raw_url);

    // Simulate work / async latency if needed
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let result = sanitize_text(&job.raw_url);

    info!(
        "[Apalis Worker] Job {} complete! Sanitized: {} (Removed {} trackers)",
        job.id, result.sanitized_url, result.removed_params_count
    );

    Ok(result)
}
