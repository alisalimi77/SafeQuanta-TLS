use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub metrics: MetricsConfig,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub kem_algorithm: KemAlgorithm,
    pub signature_algorithm: SignatureAlgorithm,
    pub fallback_config: FallbackConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KemAlgorithm {
    Kyber768,
    Kyber1024,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Dilithium3,
    Rsa3072,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FallbackConfig {
    pub enabled: bool,
    pub strategy: FallbackStrategy,
    pub non_pqc_port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FallbackStrategy {
    Reject,
    Redirect,
    ClassicTls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyConfig {
    pub mode: ProxyMode,
    pub upstream: String,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProxyMode {
    Layer4,
    Layer7,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/default.yaml".to_string());

        let config = config::Config::builder()
            .add_source(config::File::with_name(&config_path))
            .add_source(config::Environment::with_prefix("SAFEQUANTA"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
} 