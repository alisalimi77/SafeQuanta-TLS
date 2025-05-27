# SafeQuanta-TLS 🌟

A Next-Generation TLS Proxy with Post-Quantum Cryptography Support

## Overview

SafeQuanta-TLS is a cutting-edge TLS proxy implementation that provides quantum-safe communications. Built with modern cryptographic protocols and advanced observability features, it offers a robust solution for securing communications in the post-quantum era.

## Key Features

### TLS 1.3 Hybrid KEM Support
- **Default Configuration**: KYBER768 + X25519
- **Advanced Options**: 
  - KYBER1024 for enhanced security
  - DILITHIUM3 for server signatures
  - Configurable KEM selection

### Multi-Layer Reverse Proxy
- **Layer 4 & 7 Support**:
  - TCP traffic handling
  - HTTP/1.1 and HTTP/2 protocols
  - Experimental QUIC/HTTP-3 support

### Smart Fallback Mechanism
- **Legacy Client Handling**:
  - Connection rejection option
  - Non-PQC port redirection
  - Pure X25519 fallback
- **Configurable Policies**:
  - Per-client fallback rules
  - Custom security thresholds

### Advanced Observability
- **Prometheus Integration**:
  - Handshake timing metrics
  - TLS alert rates
  - CPU cycles per cgroup
- **Structured Logging**:
  - JSON format
  - Multiple log levels (Debug/Info/Warn/Error)
  - Detailed connection analytics

### Unified Configuration
- **Flexible Configuration**:
  - YAML/JSON configuration files
  - Environment variables support
  - Dynamic configuration reload
- **Kubernetes Integration**:
  - Native side-car deployment
  - Helm chart support
  - Istio mTLS compatibility

## System Requirements

| Component | Specification |
|-----------|---------------|
| **OS** | Linux x86-64 (Debian/Ubuntu 20.04+, Alpine 3.19, Fedora 40) |
| **Compiler** | Rust ≥ 1.72 (MSRV) • CMake ≥ 3.18 • Perl |
| **Crypto Libraries** | OpenSSL-OQS 3.3-dev or BoringSSL-OQS |
| **Hardware** | AES-NI CPU • 256 MB RAM per 1k connections |

## Quick Start

### Using Helm
```bash
# Add the Helm repository
helm repo add safequanta https://alisalimi77.github.io/SafeQuanta-TLS

# Install SafeQuanta-TLS
helm install safequanta safequanta/safequanta
```

### Manual Installation
```bash
# Clone the repository
git clone https://github.com/alisalimi77/SafeQuanta-TLS.git

# Build the project
cd SafeQuanta-TLS
cargo build --release
```

## Version 0.1.x Limitations

- Single KEM algorithm (KYBER768) and server signature algorithm (rsa3072 or dilithium3)
- Supported browsers: Chrome/Edge 124+ with `chrome://flags/#tls13-kyber` enabled, Firefox Nightly 126
- No Client-Auth PQC support (classic mTLS required)
- QUIC/HTTP-3 in experimental state (not recommended for production)

## Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Security

If you discover any security-related issues, please email security@safequanta-tls.dev instead of using the issue tracker.

## License

[License information will be added]

## Author

Ali Salimi 