# Contributing to png2lvgl

Thank you for your interest in contributing! This document provides guidelines for contributing to png2lvgl.

## Code of Conduct

Be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/png2lvgl.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests and checks (see below)
6. Commit and push
7. Open a Pull Request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `chore:` Maintenance tasks
- `refactor:` Code refactoring
- `test:` Test additions or changes
- `ci:` CI/CD changes

Examples:
```
feat: add support for RGB888 format
fix: handle empty PNG files gracefully
docs: update installation instructions
```

Commits are validated by commitlint via husky hooks.

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write clear, descriptive variable names
- Add comments for complex logic
- Keep functions focused and small

## Testing

- Add tests for new features
- Ensure all tests pass: `cargo test`
- Test edge cases and error conditions

## Pull Request Process

1. Update CHANGELOG.md with your changes
2. Ensure CI passes (tests, clippy, fmt)
3. Request review from maintainers
4. Address review feedback
5. Squash commits if requested

## Questions?

Open an issue for questions or discussions.
