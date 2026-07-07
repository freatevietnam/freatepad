# Contributing to FreatePad

Thank you for your interest in contributing to FreatePad!

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a feature branch (`git checkout -b feature/my-feature`)
4. Make your changes
5. Run tests (`cargo test --all`)
6. Run clippy (`cargo clippy`)
7. Commit your changes
8. Push to your fork and open a Pull Request

## Development

### Prerequisites

- Rust 1.75+
- System dependencies for egui (see README)

### Building

```bash
cargo build
```

### Testing

```bash
cargo test --all
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Code Style

- Follow standard Rust conventions
- Keep functions focused and small
- Use meaningful variable names
- Add documentation for public items
- No unnecessary comments unless requested

## Pull Requests

- Keep PRs focused on a single change
- Include a clear description of what changed and why
- Ensure all tests pass
- Ensure clippy has no warnings

## Issues

- Use the provided issue templates
- Include steps to reproduce for bugs
- Specify your OS and Rust version
