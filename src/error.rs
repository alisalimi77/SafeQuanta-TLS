use thiserror::Error;

#[derive(Error, Debug)]
pub enum SafeQuantaError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OpenSSL error: {0}")]
    OpenSsl(#[from] openssl::error::ErrorStack),

    #[error("Metrics error: {0}")]
    Metrics(String),

    #[error("Proxy error: {0}")]
    Proxy(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Handshake error: {0}")]
    Handshake(String),

    #[error("Fallback error: {0}")]
    Fallback(String),
}

pub type Result<T> = std::result::Result<T, SafeQuantaError>; 