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

- [Getting Started with the CLI](docs/tutorials/getting-started-cli.md)
- [Getting Started with VS Code](docs/tutorials/getting-started-vscode.md)

## Documentation

See [docs/](docs/) for full documentation following the [Diátaxis framework](https://diataxis.fr/):

- [tutorials/](docs/tutorials/) - Getting started guides
- [how-to/](docs/how-to/) - Task-oriented guides
- [references/](docs/references/) - Command and syntax reference
- [explanations/](docs/explanations/) - Conceptual documentation

## License

[MIT](LICENSE.md) © 2025 Matthew Battagel
