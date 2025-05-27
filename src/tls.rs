use crate::config::TlsConfig;
use crate::crypto::CryptoProvider;
use crate::error::{Result, SafeQuantaError};
use crate::metrics::Metrics;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_rustls::rustls::{
    Certificate, PrivateKey, ServerConfig, ServerName,
};
use tokio_rustls::TlsAcceptor;

/// TLS connection manager
pub struct TlsManager {
    config: Arc<TlsConfig>,
    crypto_provider: Arc<CryptoProvider>,
    metrics: Arc<Metrics>,
    acceptor: TlsAcceptor,
}

impl TlsManager {
    /// Create a new TLS manager
    pub fn new(
        config: Arc<TlsConfig>,
        crypto_provider: Arc<CryptoProvider>,
        metrics: Arc<Metrics>,
    ) -> Result<Self> {
        // Load TLS certificate and private key
        let cert = Certificate(std::fs::read(&config.cert_path)?);
        let key = PrivateKey(std::fs::read(&config.key_path)?);

        // Configure TLS server
        let mut server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![cert], key)?;

        // Enable quantum-safe cipher suites
        server_config.cipher_suites = vec![
            // TODO: Add quantum-safe cipher suites
            // This will be implemented when we add the actual crypto implementations
        ];

        Ok(Self {
            config,
            crypto_provider,
            metrics,
            acceptor: TlsAcceptor::from(Arc::new(server_config)),
        })
    }

    /// Accept a new TLS connection
    pub async fn accept(&self, stream: TcpStream) -> Result<impl AsyncRead + AsyncWrite> {
        let start_time = std::time::Instant::now();
        
        // Accept TLS connection
        let tls_stream = self.acceptor.accept(stream).await?;
        
        // Record metrics
        self.metrics.record_tls_handshake_time(start_time.elapsed());
        self.metrics.increment_tls_connections();

        Ok(tls_stream)
    }

    /// Create a new TLS client connection
    pub async fn connect(&self, server_name: &str) -> Result<impl AsyncRead + AsyncWrite> {
        let start_time = std::time::Instant::now();
        
        // Create TCP connection
        let stream = TcpStream::connect(&self.config.server_addr).await?;
        
        // Perform TLS handshake
        let tls_stream = tokio_rustls::TlsConnector::from(self.acceptor.config().clone())
            .connect(ServerName::try_from(server_name)?, stream)
            .await?;
        
        // Record metrics
        self.metrics.record_tls_handshake_time(start_time.elapsed());
        self.metrics.increment_tls_connections();

        Ok(tls_stream)
    }

    /// Perform a quantum-safe key exchange during TLS handshake
    async fn perform_quantum_safe_key_exchange(&self) -> Result<Vec<u8>> {
        // TODO: Implement quantum-safe key exchange during TLS handshake
        Err(SafeQuantaError::Tls("Quantum-safe key exchange not implemented yet".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{KemAlgorithm, SignatureAlgorithm};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    async fn setup_test_tls_manager() -> (TlsManager, SocketAddr) {
        let config = Arc::new(TlsConfig {
            cert_path: "tests/fixtures/test.crt".to_string(),
            key_path: "tests/fixtures/test.key".to_string(),
            server_addr: "127.0.0.1:0".parse().unwrap(),
            kem_algorithm: KemAlgorithm::Kyber768,
            signature_algorithm: SignatureAlgorithm::Dilithium3,
        });

        let metrics = Arc::new(Metrics::new());
        let crypto_provider = Arc::new(CryptoProvider::new(
            config.kem_algorithm,
            config.signature_algorithm,
            &config.cert_path,
            &config.key_path,
        ).unwrap());

        let tls_manager = TlsManager::new(config, crypto_provider, metrics).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        (tls_manager, addr)
    }

    #[tokio::test]
    async fn test_tls_manager_creation() {
        let (tls_manager, _) = setup_test_tls_manager().await;
        assert!(tls_manager.acceptor.config().cipher_suites.is_empty());
    }

    #[tokio::test]
    async fn test_tls_handshake() {
        let (tls_manager, addr) = setup_test_tls_manager().await;
        
        // Start server
        let server = tokio::spawn(async move {
            let listener = TcpListener::bind(addr).await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            let mut tls_stream = tls_manager.accept(stream).await.unwrap();
            
            let mut buf = [0u8; 1024];
            let n = tls_stream.read(&mut buf).await.unwrap();
            assert_eq!(&buf[..n], b"hello");
            
            tls_stream.write_all(b"world").await.unwrap();
        });

        // Connect client
        let stream = TcpStream::connect(addr).await.unwrap();
        let mut tls_stream = tls_manager.connect("localhost").await.unwrap();
        
        tls_stream.write_all(b"hello").await.unwrap();
        
        let mut buf = [0u8; 1024];
        let n = tls_stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"world");

        server.await.unwrap();
    }
} 