# CLI Development

Guide for developing the Hypha CLI.

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (1.70+)

## Setup

Clone and build:

```bash
cd cli
cargo build
```

## Development Workflow

### Run Without Installing

```bash
cargo run -- --help
cargo run -- new "Test Topic"
cargo run -- list
cargo run -- search "status:active"
```

Use `--` to separate cargo arguments from hypha arguments.

### Run Against a Specific Directory

```bash
cargo run -- --root /path/to/notes list
```

### Watch Mode (Auto-Rebuild)

Install cargo-watch for auto-rebuilding:

```bash
cargo install cargo-watch
cargo watch -x "run -- list"
```

### Run Tests

```bash
cargo test
```

Run a specific test:

```bash
cargo test query::tests::parse_mixed
```

### Check Without Building

```bash
cargo check
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

Check formatting without applying:

```bash
cargo fmt -- --check
```

## Build Release Binary

```bash
cargo build --release
./target/release/hypha --help
```

## Install Locally

Install to `~/.cargo/bin/`:

```bash
cargo install --path .
```

Reinstall after changes:

```bash
cargo install --path . --force
```
