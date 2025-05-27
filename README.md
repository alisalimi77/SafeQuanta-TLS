# SafeQuanta TLS Proxy

A quantum-safe TLS proxy implementation that provides post-quantum cryptography (PQC) support for secure communications resistant to both classical and quantum computer attacks. This project aims to demonstrate the integration of PQC algorithms into the TLS 1.3 protocol.

## Features

- Quantum-safe key exchange using Kyber (768 and 1024 variants)
- Quantum-safe signatures using Dilithium3 and classical RSA-3072 for compatibility
- TLS 1.3 support with post-quantum cipher suites
- High-performance asynchronous I/O using the Tokio runtime
- Metrics collection and monitoring (Prometheus format)
- Configurable connection limits and timeouts for both server and proxy connections
- Flexible configuration using YAML files

## Requirements

To build and run this project, you need:

- **Rust:** Version 1.70 or later. Install using `rustup`: [https://rustup.rs/](https://rustup.rs/).
- **OpenSSL:** Development libraries. On Windows, it's recommended to use `vcpkg`. Install vcpkg and then run `vcpkg install openssl:x64-windows`. Ensure vcpkg is integrated with your Rust setup (refer to vcpkg documentation).
- **C/C++ Compiler:**
    - On Windows, if you encounter build issues related to assembly files (common with `pqcrypto` crates), using a GNU toolchain like **MinGW-w64** is often necessary.
      1.  Download the `x86_64-*-posix-seh-ucrt` build from a reliable source (e.g., [https://github.com/niXman/mingw-builds-binaries/releases](https://github.com/niXman/mingw-builds-binaries/releases)). Look for a file like `x86_64-15.1.0-release-posix-seh-ucrt-rt_v12-rev0.7z`.
      2.  Extract the downloaded archive to a directory, for example, `C:\\mingw64`.
      3.  Add the `bin` directory (e.g., `C:\\mingw64\\bin`) to your system's PATH environment variable.
      4.  Set your default Rust toolchain to the GNU target: `rustup default stable-x86_64-pc-windows-gnu`.
    - On Linux/macOS, ensure you have a standard C/C++ build environment (like `build-essential` on Debian/Ubuntu or Xcode Command Line Tools on macOS).
- **A valid TLS certificate and private key:** For the proxy to present to clients.
- **A YAML configuration file:** To specify proxy settings.

## Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/alisalimi77/SafeQuanta-TLS.git
    cd SafeQuanta-TLS
    ```

2.  **Install Rust:** If you haven't already, install Rust using rustup.

3.  **Install OpenSSL:**
    - **Windows (using vcpkg):**
      ```bash
      # Navigate to your vcpkg directory
      cd D:\\vcpkg
      vcpkg install openssl:x64-windows
      # Set environment variables for Rust build
      $env:OPENSSL_DIR = "D:\\vcpkg\\installed\\x64-windows"
      $env:OPENSSL_STATIC = "1" # If linking statically
      ```
      You might need to set these environment variables in your shell session before building or add them permanently to your system.
    - **Linux/macOS:** Use your system's package manager (e.g., `sudo apt-get install libssl-dev` on Debian/Ubuntu, `brew install openssl@3` on macOS).

4.  **Install MinGW-w64 (Windows, if needed):** Follow the steps in the "Requirements" section if you encounter compiler errors.

5.  **Build the project:**
    ```bash
    cargo build --release
    ```
    This will build the optimized release version of the proxy executable.

6.  **Create a configuration file:** The project uses `config/default.yaml` as a template. Copy it and edit for your specific needs.
    ```bash
    cp config/default.yaml config/local.yaml
    # Open config/local.yaml in your editor and modify settings
    ```

## Configuration

The proxy is configured using a YAML file. A typical configuration includes:

```yaml
server:
  listen_addr: "0.0.0.0:8443" # Address and port the proxy listens on (e.g., for clients)
  max_connections: 1000       # Maximum concurrent connections to the proxy listener
  timeout: 30s                # Connection timeout for proxy listener

tls:
  cert_path: "certs/server.crt" # Path to the TLS certificate file
  key_path: "certs/server.key"  # Path to the TLS private key file
  kem_algorithm: "kyber768"     # Post-quantum KEM algorithm: "kyber768" or "kyber1024"
  signature_algorithm: "dilithium3" # Post-quantum/classical signature algorithm: "dilithium3" or "rsa3072"

metrics:
  enabled: true             # Enable or disable metrics endpoint
  prometheus_port: 9090     # Port for the Prometheus metrics endpoint

proxy:
  target_addr: "127.0.0.1:443" # Address and port of the target server (e.g., the actual web server)
  target_host: "example.com" # Host header to use when connecting to the target server
  max_connections: 100      # Maximum concurrent connections from the proxy to the target server
  timeout: 30s               # Connection timeout for target connections
```

Edit `config/local.yaml` to match your desired settings, including paths to your certificate and key files and the target server details.

## Usage

1.  **Ensure you have your TLS certificate and key files ready** (e.g., in a `certs/` directory).
2.  **Run the proxy** with your configuration file:
    ```bash
    ./target/release/safequanta-tls --config config/local.yaml
    ```
    (On Windows, the executable will be `./target/release/safequanta-tls.exe`)

3.  **Configure your client** to use the proxy address and port. For example, using `curl`:
    ```bash
    curl --proxy https://localhost:8443 https://example.com
    ```
    Replace `localhost:8443` with the actual listen address and port you configured, and `https://example.com` with the target host and path.

## Security

This project implements post-quantum cryptography algorithms that are designed to be resistant to attacks from both classical and quantum computers, in addition to classical algorithms for compatibility:

-   **Kyber:** A lattice-based Key Encapsulation Mechanism (KEM) selected by NIST for standardization. Used for establishing shared secrets.
-   **Dilithium:** A lattice-based digital signature scheme also selected by NIST. Used for authentication.
-   **RSA-3072:** A widely used classical digital signature algorithm included for backward compatibility.

## Development

### Building

Build the project using Cargo. For a debug build (faster compile times, includes debug info):
```bash
cargo build
```
For an optimized release build:
```bash
cargo build --release
```

### Testing

Run the automated tests to ensure everything is working correctly:
```bash
cargo test
```

### Code Style and Linting

Ensure your code adheres to the project's style and passes lint checks:
```bash
cargo fmt      # Formats the code according to standard Rust style
cargo clippy   # Runs linter checks to catch common mistakes and improve code quality
```

## Project Structure

-   `src/main.rs`: Entry point of the application.
-   `src/proxy.rs`: Contains the core proxy logic.
-   `src/tls.rs`: Handles TLS setup and configuration.
-   `src/crypto.rs`: Deals with cryptography-related operations, including PQC.
-   `src/config.rs`: Handles loading and parsing the configuration file.
-   `src/error.rs`: Defines custom error types.
-   `src/metrics.rs`: Implements metrics collection.
-   `config/default.yaml`: Default configuration file template.
-   `tests/`: Contains integration tests.

## Contributing

Contributions are welcome! Please follow these steps:

1.  Fork the repository on GitHub.
2.  Create a new branch for your feature or bugfix (`git checkout -b feature/your-feature-name`).
3.  Commit your changes, ensuring your commit messages are clear and descriptive.
4.  Push your branch to your fork (`git push origin feature/your-feature-name`).
5.  Open a Pull Request against the main repository, describing your changes.

Please ensure your code follows the project's style (`cargo fmt`) and passes lint checks (`cargo clippy`) and tests (`cargo test`) before submitting a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

-   [NIST Post-Quantum Cryptography Project](https://csrc.nist.gov/projects/post-quantum-cryptography) for driving the standardization of PQC algorithms.
-   The developers of the Rust crates used, including `rustls`, `tokio`, `pqcrypto-*`, and others.

## Troubleshooting

*   **"command not found: cargo" or "command not found: rustup":** Ensure that the Cargo bin directory (`%USERPROFILE%\.cargo\bin` on Windows, `~/.cargo/bin` on Linux/macOS) is added to your system's PATH environment variable. You might need to restart your terminal or computer after installing Rust.
*   **Linker errors on Windows (e.g., cannot open input file '*.o'):** This often indicates an issue with the C/C++ compiler not correctly handling assembly files used by some dependencies (like `pqcrypto`). Switching your Rust toolchain to use the GNU target and ensuring MinGW-w64 is correctly installed and in your PATH usually resolves this. Follow the MinGW-w64 installation steps in the "Requirements" section.
*   **OpenSSL linking errors:** Ensure OpenSSL is correctly installed for your target and the appropriate environment variables (`OPENSSL_DIR`, `OPENSSL_STATIC`) are set before building with Cargo, especially when using vcpkg.

Please open an issue on GitHub if you encounter other problems. 