# Contributing to PredictiveRolls

Thank you for your interest in contributing to PredictiveRolls! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/PredictiveRolls.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites
- Rust 1.70 or later
- Cargo
- A supported GPU (for Vulkan backend)

### Building the Project
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Formatting
We use rustfmt for consistent code formatting:
```bash
cargo fmt --all
```

### Linting
We use clippy for linting:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Code Style

- Follow the Rust API Guidelines
- Use `rustfmt` with the provided configuration
- Ensure all clippy warnings are addressed
- Write clear, descriptive commit messages
- Add comments for complex logic
- Update documentation when changing public APIs

## Submitting Changes

1. Ensure all tests pass
2. Run `cargo fmt` and `cargo clippy`
3. Commit your changes with a clear message
4. Push to your fork
5. Create a Pull Request with a detailed description

## Code Review Process

- All submissions require review
- Address reviewer feedback promptly
- Maintain a respectful and constructive tone

## Reporting Issues

- Use GitHub Issues to report bugs
- Provide clear reproduction steps
- Include system information and error messages

## Security

- Do not commit API keys or credentials
- Report security vulnerabilities privately

## Questions?

Feel free to open an issue for questions or discussion!
