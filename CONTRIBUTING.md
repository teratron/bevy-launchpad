# Contributing to Bevy Launchpad

Thank you for your interest in contributing! We welcome contributions from everyone.

## Code of Conduct

Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/teratron/bevy-launchpad/issues)
2. If not, create a new issue with:
   - Clear title
   - Steps to reproduce
   - Expected vs actual behavior
   - Bevy Launchpad version
   - Operating system

### Suggesting Features

1. Check [Issues](https://github.com/teratron/bevy-launchpad/issues) and [Roadmap](ROADMAP.md)
2. Create a feature request with:
   - Clear use case
   - Proposed API (if applicable)
   - Examples of usage

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Run fmt: `cargo fmt --all`
6. Run clippy: `cargo clippy --workspace -- -D warnings`
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Code Style

- Follow Rust API guidelines
- Document public APIs with rustdoc
- Add tests for new features
- Keep examples up to date

## Development Setup

```bash
git clone https://github.com/teratron/bevy-launchpad
cd bevy-launchpad
cargo test --workspace
cargo run --example minimal_2d
```

## Questions?

Feel free to ask in [Discussions](https://github.com/teratron/bevy-launchpad/discussions)!
