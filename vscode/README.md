# Hypha

Hypha brings fast, flexible note management to VS Code. Define your own frontmatter schema, query any field, and keep your notes as portable markdown files.

## Features

- **In-editor notes** — Stay focused with less context switching
- **Your schema** — Define custom frontmatter fields and query them all
- **Keyboard shortcuts** — Built-in shortcuts for core workflows
- **Plain markdown** — Portable files, no lock-in
- **Safe refiling** — Rename topics and update all links automatically

## Getting Started

1. Click the **Hypha icon** in the Activity Bar
2. Set your **notes directory** when prompted
3. Create a new note and start writing!

> **Note:** The extension includes pre-compiled CLI binaries for all platforms. If you prefer your own CLI installation, set `hypha.binaryPath`.

## Notes

Notes are markdown files that can include frontmatter, a YAML section at the top of the file. Frontmatter fields power Hypha's query language so you can navigate notes more effectively.

```text
---
status: in-progress
tags: [work, meeting]
priority: high
project: hypha
---

# Project Meeting Notes

A short description describing the note.

## Discussed roadmap
1. Remove lorem ipsum
```

Query: `status:in-progress priority:high` — instantly filter to active high-priority items.

## Learn More

For more information, see [Hypha on GitHub](https://github.com/battagel/hypha).
