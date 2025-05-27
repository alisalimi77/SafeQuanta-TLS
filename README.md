# SafeQuanta TLS Proxy

A quantum-safe TLS proxy implementation that provides post-quantum cryptography support for secure communications.

## Features

- Quantum-safe key exchange using Kyber (768 and 1024 variants)
- Quantum-safe signatures using Dilithium3
- TLS 1.3 support with post-quantum cipher suites
- High-performance async I/O with Tokio
- Metrics collection and monitoring
- Configurable connection limits and timeouts

## Requirements

- Rust 1.70 or later
- OpenSSL development libraries
- A valid TLS certificate and private key

## Installation

1. Clone the repository:
```bash
git clone https://github.com/alisalimi77/SafeQuanta-TLS.git
cd SafeQuanta-TLS
```

2. Build the project:
```bash
cargo build --release
```

3. Create a configuration file:
```bash
cp config/default.yaml config/local.yaml
# Edit config/local.yaml with your settings
```

4. Run the proxy:
```bash
./target/release/safequanta-tls
```

## Configuration

The proxy is configured using a YAML file. Here's an example configuration:

```yaml
server:
  listen_addr: "0.0.0.0:8443"
  max_connections: 1000
  timeout: 30s

tls:
  cert_path: "certs/server.crt"
  key_path: "certs/server.key"
  kem_algorithm: "kyber768"  # or "kyber1024"
  signature_algorithm: "dilithium3"  # or "rsa3072"

metrics:
  enabled: true
  prometheus_port: 9090

proxy:
  target_addr: "127.0.0.1:443"
  target_host: "example.com"
  max_connections: 100
```

## Usage

1. Start the proxy with your configuration:
```bash
./target/release/safequanta-tls --config config/local.yaml
```

2. Configure your client to use the proxy:
```bash
curl --proxy https://localhost:8443 https://example.com
```

## Security

This project implements post-quantum cryptography algorithms that are resistant to attacks from both classical and quantum computers:

- Kyber: A lattice-based key encapsulation mechanism (KEM)
- Dilithium: A lattice-based digital signature scheme
- RSA-3072: A classical signature algorithm for compatibility

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Code Style

```bash
cargo fmt
cargo clippy
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [NIST PQC Project](https://csrc.nist.gov/projects/post-quantum-cryptography)
- [Rustls](https://github.com/rustls/rustls)
- [Tokio](https://github.com/tokio-rs/tokio) 