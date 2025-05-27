mod config;
mod crypto;
mod error;
mod metrics;
mod proxy;
mod tls;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .pretty()
        .init();

    info!("Starting SafeQuanta-TLS proxy...");

    // Load configuration
    let config = config::Config::load()?;
    info!("Configuration loaded successfully");

    // Initialize metrics
    metrics::init(&config.metrics)?;
    info!("Metrics initialized");

    // Start the proxy
    proxy::start(config).await?;

    Ok(())
} 