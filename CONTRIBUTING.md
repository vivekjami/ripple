# Contributing to Ripple

Thank you for your interest in contributing to Ripple! This document outlines the process and guidelines.

## How to Contribute

1. **Fork** the repository
2. **Create a branch** for your feature or fix (`git checkout -b feature/my-feature`)
3. **Commit** your changes with clear messages
4. **Push** to your fork
5. **Open a Pull Request** against `main`

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- Follow Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Document all public functions with `///` doc comments
- Include examples in documentation where helpful

## Testing

- Write unit tests for all new functionality
- Run the full test suite before submitting: `cargo test`
- Integration tests go in `tests/integration/`
- Load tests go in `tests/load/`

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if your change affects public APIs
3. Add an entry to `CHANGELOG.md`
4. Request review from a maintainer
5. Address review feedback promptly

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Include reproduction steps for bugs
- Include expected vs. actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
