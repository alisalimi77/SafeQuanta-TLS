# Contributing to SafeQuanta-TLS

Thank you for your interest in contributing to SafeQuanta-TLS! This document provides guidelines and instructions for contributing to the project.

## Prerequisites

- Rust ≥ 1.72
- CMake ≥ 3.18
- Perl (for OpenSSL build)
- Git
- Basic understanding of TLS and cryptography

## Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/SafeQuanta-TLS.git
   cd SafeQuanta-TLS
   ```

2. **Build Dependencies**
   ```bash
   # Install OpenSSL-OQS
   ./scripts/install-openssl-oqs.sh
   
   # Install Rust dependencies
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

## Contribution Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Development**
   - Write your code
   - Add tests
   - Update documentation
   - Run linters and tests

3. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   git push origin feature/your-feature-name
   ```

4. **Create Pull Request**
   - Go to GitHub repository
   - Click "New Pull Request"
   - Fill in the PR template
   - Request review from maintainers

## Code Standards

### Rust Code Style
- Use `rustfmt` for formatting
- Run `cargo clippy` for linting
- Follow Rust API guidelines
- Document public APIs

### Testing
- Write unit tests for new features
- Include integration tests where appropriate
- Maintain test coverage
- Test edge cases and error conditions

### Documentation
- Update README.md for user-facing changes
- Document new configuration options
- Add inline code comments
- Update API documentation

## Security Considerations

- Report security vulnerabilities to security@safequanta-tls.dev
- Do not include sensitive data in commits
- Follow security best practices
- Review cryptographic implementations carefully

## Review Process

1. **Code Review**
   - Address reviewer comments
   - Make requested changes
   - Keep PR up to date with main branch

2. **CI Checks**
   - All tests must pass
   - Code must be properly formatted
   - No security vulnerabilities
   - Documentation must be complete

3. **Final Approval**
   - At least one maintainer approval required
   - All CI checks must pass
   - No unresolved comments

## Getting Help

- Join our [Discord community](https://discord.gg/safequanta)
- Check existing issues and discussions
- Review documentation
- Contact maintainers

## License

By contributing to SafeQuanta-TLS, you agree that your contributions will be licensed under the project's license. 