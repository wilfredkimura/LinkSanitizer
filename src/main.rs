use link_sanitizer::service::LinkSanitizerService;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize console logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("=======================================================");
    println!("     Rivium + Apalis LinkSanitizer (Desktop Runner)     ");
    println!("=======================================================\n");

    let db_path = "target/link_sanitizer.db";
    let mut service = LinkSanitizerService::init(db_path).await?;

    // Enqueue a few test URLs with various trackers
    let sample_urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ&si=tracking_id_12345",
        "https://news.ycombinator.com/item?id=123456&utm_source=twitter&utm_medium=social&utm_campaign=launch",
        "https://github.com/rivium-rs/rivium?fbclid=IwAR2V8b6wO6k0hJ9Q",
    ];

    for url in sample_urls {
        println!(">> Submitting URL to Apalis SQLite queue: {url}");
        service.enqueue(url).await?;
    }

    println!("\n>> Starting Rivium background worker to process queue...");
    println!(">> Press Ctrl+C to exit.\n");

    // Start background processing
    service.run_worker().await;

    Ok(())
}
