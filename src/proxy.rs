use crate::config::ProxyConfig;
use crate::crypto::CryptoProvider;
use crate::error::{Result, SafeQuantaError};
use crate::metrics::Metrics;
use crate::tls::TlsManager;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Proxy server implementation
pub struct ProxyServer {
    config: Arc<ProxyConfig>,
    tls_manager: Arc<TlsManager>,
    crypto_provider: Arc<CryptoProvider>,
    metrics: Arc<Metrics>,
    connection_limit: Arc<Semaphore>,
}

impl ProxyServer {
    /// Create a new proxy server
    pub fn new(
        config: Arc<ProxyConfig>,
        tls_manager: Arc<TlsManager>,
        crypto_provider: Arc<CryptoProvider>,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            config: config.clone(),
            tls_manager,
            crypto_provider,
            metrics,
            connection_limit: Arc::new(Semaphore::new(config.max_connections)),
        }
    }

    /// Start the proxy server
    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.config.listen_addr).await?;
        log::info!("Proxy server listening on {}", self.config.listen_addr);

        loop {
            // Accept new connection
            let (client_stream, client_addr) = listener.accept().await?;
            log::debug!("New connection from {}", client_addr);

            // Clone necessary components for the connection handler
            let tls_manager = self.tls_manager.clone();
            let crypto_provider = self.crypto_provider.clone();
            let metrics = self.metrics.clone();
            let connection_limit = self.connection_limit.clone();
            let config = self.config.clone();

            // Spawn connection handler
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(
                    client_stream,
                    client_addr,
                    tls_manager,
                    crypto_provider,
                    metrics,
                    connection_limit,
                    config,
                )
                .await
                {
                    log::error!("Connection error: {}", e);
                }
            });
        }
    }

    /// Handle a single client connection
    async fn handle_connection(
        client_stream: TcpStream,
        client_addr: std::net::SocketAddr,
        tls_manager: Arc<TlsManager>,
        crypto_provider: Arc<CryptoProvider>,
        metrics: Arc<Metrics>,
        connection_limit: Arc<Semaphore>,
        config: Arc<ProxyConfig>,
    ) -> Result<()> {
        // Acquire connection permit
        let _permit = connection_limit.acquire().await?;

        // Accept TLS connection
        let client_tls = tls_manager.accept(client_stream).await?;

        // Connect to target server
        let target_stream = TcpStream::connect(&config.target_addr).await?;
        let target_tls = tls_manager.connect(&config.target_host).await?;

        // Start proxying data
        let (client_reader, client_writer) = tokio::io::split(client_tls);
        let (target_reader, target_writer) = tokio::io::split(target_tls);

        // Spawn bidirectional data transfer
        let client_to_target = Self::proxy_data(
            client_reader,
            target_writer,
            "client -> target",
            metrics.clone(),
        );
        let target_to_client = Self::proxy_data(
            target_reader,
            client_writer,
            "target -> client",
            metrics.clone(),
        );

        // Wait for either direction to complete
        tokio::select! {
            result = client_to_target => {
                if let Err(e) = result {
                    log::error!("Client to target error: {}", e);
                }
            }
            result = target_to_client => {
                if let Err(e) = result {
                    log::error!("Target to client error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Proxy data between two streams
    async fn proxy_data<R, W>(
        mut reader: R,
        mut writer: W,
        direction: &str,
        metrics: Arc<Metrics>,
    ) -> Result<()>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        let mut buffer = vec![0u8; 8192];
        let mut total_bytes = 0;

        loop {
            let n = reader.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            writer.write_all(&buffer[..n]).await?;
            total_bytes += n;

            // Record metrics
            metrics.record_bytes_transferred(n);
        }

        log::debug!("{}: transferred {} bytes", direction, total_bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{KemAlgorithm, SignatureAlgorithm};
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::time::Duration;

    async fn setup_test_proxy() -> (ProxyServer, SocketAddr, SocketAddr) {
        let proxy_config = Arc::new(ProxyConfig {
            listen_addr: "127.0.0.1:0".parse().unwrap(),
            target_addr: "127.0.0.1:0".parse().unwrap(),
            target_host: "localhost".to_string(),
            max_connections: 10,
        });

        let tls_config = Arc::new(crate::config::TlsConfig {
            cert_path: "tests/fixtures/test.crt".to_string(),
            key_path: "tests/fixtures/test.key".to_string(),
            server_addr: "127.0.0.1:0".parse().unwrap(),
            kem_algorithm: KemAlgorithm::Kyber768,
            signature_algorithm: SignatureAlgorithm::Dilithium3,
        });

        let metrics = Arc::new(Metrics::new());
        let crypto_provider = Arc::new(CryptoProvider::new(
            tls_config.kem_algorithm,
            tls_config.signature_algorithm,
            &tls_config.cert_path,
            &tls_config.key_path,
        ).unwrap());

        let tls_manager = Arc::new(TlsManager::new(
            tls_config,
            crypto_provider.clone(),
            metrics.clone(),
        ).unwrap());

        let proxy_server = ProxyServer::new(
            proxy_config.clone(),
            tls_manager,
            crypto_provider,
            metrics,
        );

        let proxy_listener = TcpListener::bind(proxy_config.listen_addr).await.unwrap();
        let proxy_addr = proxy_listener.local_addr().unwrap();

        let target_listener = TcpListener::bind(proxy_config.target_addr).await.unwrap();
        let target_addr = target_listener.local_addr().unwrap();

        (proxy_server, proxy_addr, target_addr)
    }

    #[tokio::test]
    async fn test_proxy_data_transfer() {
        let (proxy_server, proxy_addr, target_addr) = setup_test_proxy().await;

        // Start target server
        let target_server = tokio::spawn(async move {
            let listener = TcpListener::bind(target_addr).await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();
            let mut tls_stream = proxy_server.tls_manager.accept(stream).await.unwrap();
            
            let mut buf = [0u8; 1024];
            let n = tls_stream.read(&mut buf).await.unwrap();
            assert_eq!(&buf[..n], b"hello");
            
            tls_stream.write_all(b"world").await.unwrap();
        });

        // Connect client to proxy
        let client_stream = TcpStream::connect(proxy_addr).await.unwrap();
        let mut client_tls = proxy_server.tls_manager.connect("localhost").await.unwrap();
        
        client_tls.write_all(b"hello").await.unwrap();
        
        let mut buf = [0u8; 1024];
        let n = client_tls.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"world");

        target_server.await.unwrap();
    }

    #[tokio::test]
    async fn test_connection_limit() {
        let (proxy_server, proxy_addr, _) = setup_test_proxy().await;
        let mut handles = vec![];

        // Try to establish more connections than the limit
        for _ in 0..proxy_server.config.max_connections + 1 {
            let handle = tokio::spawn(async move {
                let stream = TcpStream::connect(proxy_addr).await;
                stream
            });
            handles.push(handle);
        }

        // Wait for all connection attempts
        let results = futures::future::join_all(handles).await;
        
        // Count successful connections
        let successful = results.iter()
            .filter(|r| r.as_ref().unwrap().is_ok())
            .count();

        assert_eq!(successful, proxy_server.config.max_connections);
    }
} 