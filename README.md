# Hypha

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A CLI and VS Code extension for managing markdown notes with user-defined metadata. Your notes, your schema, your workflow.

## Why Hypha?

Most note-taking tools force you into their structure. Hypha takes a different approach: **you define the metadata that matters to you**.

- **Flexible schema** — Add any frontmatter fields; all are queryable
- **Plain files** — Your notes stay as portable markdown
- **Fast** — Rust CLI indexes hundreds of files in milliseconds
- **Editor agnostic** — CLI works anywhere; VS Code extension for GUI

```yaml
---
status: in-progress
tags: [rust, cli]
energy: high
client: acme-corp
---
```

```bash
hypha search "status:in-progress client:acme"
```

## Quick Start

### CLI

**Install from source:**
```bash
git clone https://github.com/battagel/hypha.git
cd hypha/cli
cargo install --path .
```

**Or download a binary** from [releases](https://github.com/battagel/hypha/releases).

**Get started:**
- [Getting Started with the CLI](docs/tutorials/getting-started-cli.md)

### VS Code Extension

1. Install the [Hypha CLI](#cli) first
2. Download the `.vsix` from [releases](https://github.com/battagel/hypha/releases)
3. Install: `code --install-extension hypha-vscode-1.0.1-alpha.vsix`

**Get started:**
- [Getting Started with VS Code](docs/tutorials/getting-started-vscode.md)

## Documentation

See [docs/](docs/README.md) for full documentation:

- **Tutorials**: [CLI](docs/tutorials/getting-started-cli.md) | [VS Code](docs/tutorials/getting-started-vscode.md)
- **References**: [CLI Commands](docs/references/cli.md) | [Query Syntax](docs/references/query-syntax.md) | [Frontmatter](docs/references/frontmatter.md)
- **Explanations**: [Architecture](docs/explanations/architecture.md) | [Linking](docs/explanations/linking.md) | [Frontmatter](docs/explanations/frontmatter-schema.md)

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

[MIT](LICENSE.md) © 2025 Matthew Battagel
