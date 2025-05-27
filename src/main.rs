mod config;
mod crypto;
mod error;
mod metrics;
mod proxy;
mod tls;

use crate::config::Config;
use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::metrics::Metrics;
use crate::proxy::ProxyServer;
use crate::tls::TlsManager;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    log::info!("Starting SafeQuanta TLS Proxy...");

    // Load configuration
    let config = Arc::new(Config::load()?);
    log::info!("Configuration loaded successfully");

    // Initialize metrics
    let metrics = Arc::new(Metrics::new());
    log::info!("Metrics initialized");

    // Initialize crypto provider
    let crypto_provider = Arc::new(CryptoProvider::new(
        config.tls.kem_algorithm,
        config.tls.signature_algorithm,
        &config.tls.cert_path,
        &config.tls.key_path,
    )?);
    log::info!("Crypto provider initialized");

    // Initialize TLS manager
    let tls_manager = Arc::new(TlsManager::new(
        Arc::new(config.tls.clone()),
        crypto_provider.clone(),
        metrics.clone(),
    )?);
    log::info!("TLS manager initialized");

    // Create and start proxy server
    let proxy_server = ProxyServer::new(
        Arc::new(config.proxy.clone()),
        tls_manager,
        crypto_provider,
        metrics,
    );
    log::info!("Proxy server created");

    // Start the server
    proxy_server.start().await?;

    Ok(())
} 