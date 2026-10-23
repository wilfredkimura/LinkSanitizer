use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::Path;
use std::str::FromStr;
use tracing::info;

use crate::job::{process_sanitize_job, SanitizeLinkJob};

/// Configuration and state for the LinkSanitizer backend service
pub struct LinkSanitizerService {
    pub db_path: String,
    pub storage: SqliteStorage<SanitizeLinkJob>,
}

impl LinkSanitizerService {
    /// Initialize SQLite database and setup Apalis storage table
    pub async fn init(database_path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing SQLite storage at: {}", database_path);

        // Ensure parent directory exists
        if let Some(parent) = Path::new(database_path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let opts = SqliteConnectOptions::from_str(&format!("sqlite://{database_path}"))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?;

        // Sets up SQLite tables required by Apalis for persistence
        SqliteStorage::setup(&pool).await?;

        let storage = SqliteStorage::new(pool);

        Ok(Self {
            db_path: database_path.to_string(),
            storage,
        })
    }

    /// Enqueue a new link sanitization job into persistent SQLite storage
    pub async fn enqueue(&mut self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let job = SanitizeLinkJob::new(url);
        let job_id = job.id.clone();
        self.storage.push(job).await?;
        info!("Enqueued job {} for url: {}", job_id, url);
        Ok(job_id)
    }

    /// Start the Apalis worker loop (keeps running under Rivium)
    pub async fn run_worker(self) {
        info!("Starting Apalis LinkSanitizer worker...");

        WorkerBuilder::new("link-sanitizer-worker")
            .backend(self.storage)
            .build_fn(process_sanitize_job)
            .run()
            .await;
    }
}
