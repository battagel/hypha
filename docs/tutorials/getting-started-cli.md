# Getting Started with the Hypha CLI

A step-by-step guide to installing and using the Hypha CLI.

## Prerequisites

- A terminal (zsh, bash, etc.)

## Installation

### Download Pre-built Binary (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/battagel/hypha/releases/latest):

- **Linux**: `hypha-v*-linux-x86_64`
- **macOS (Intel)**: `hypha-v*-darwin-x86_64`
- **macOS (Apple Silicon)**: `hypha-v*-darwin-aarch64`
- **Windows**: `hypha-v*-windows-x86_64.exe`

Make the binary executable and add it to your PATH:

```bash
# Linux/macOS (user-level, no sudo required)
chmod +x hypha-v*-*
mkdir -p ~/.local/bin
mv hypha-v*-* ~/.local/bin/hypha

# Add to PATH if not already (add to ~/.zshrc or ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"

# System-wide (requires sudo)
# sudo mv hypha-v*-* /usr/local/bin/hypha

# Windows (PowerShell)
# Create a bin directory in your user profile
# New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"
# Move-Item hypha-v*-windows-x86_64.exe "$env:USERPROFILE\bin\hypha.exe"
# Then add %USERPROFILE%\bin to your user PATH in System Properties
```

Verify installation:

```bash
hypha --version
```

### Build from Source (Alternative)

If you prefer to build from source, you'll need [Rust toolchain](https://rustup.rs/) (1.70 or later):

```bash
git clone https://github.com/battagel/hypha.git
cd hypha/cli
cargo install --path .
```

## Initial Setup

Run any command and Hypha will prompt you to configure your notes directory:

```bash
hypha list
```

```text
Welcome to Hypha! Let's get you set up.

Where would you like to store your notes? [/Users/you/hypha]:
```

Press Enter to accept the default, or type a custom path. Hypha creates a config file at `~/.hypha` with your settings.

## Creating Your First Topic

Create a new topic:

```bash
hypha new "My First Note"
```

This creates `my-first-note.md` and opens it in your default editor (`$EDITOR`). The file starts with frontmatter from your template:

```markdown
---
title: My First Note
---

# My First Note
```

Add some content:

```markdown
---
title: My First Note
status: active
tags:
  - tutorial
---

# My First Note

This is my first note in Hypha! It demonstrates frontmatter metadata.

## Next Steps

- Create more topics
- Learn about search queries
```

## Listing and Searching Topics

List all topics:

```bash
hypha list
```

Search by frontmatter fields:

```bash
hypha search "status:active"
hypha search "tags:tutorial"
```

Search by title text:

```bash
hypha search "first note"
```

Combine filters and text:

```bash
hypha search "status:active tutorial"
```

## Linking Between Topics

Create another topic:

```bash
hypha new "Project Ideas"
```

Add a link to your first note using standard Markdown syntax:

```markdown
# Project Ideas

See [My First Note](my-first-note.md) for context.
```

## Finding Backlinks

See which topics link to a given topic:

```bash
hypha backlinks "my-first-note"
```

Output:

```text
project-ideas.md
  Project Ideas
```

## Linting for Issues

Check your notes for problems (missing titles, empty content, broken links):

```bash
hypha lint
```

If there are issues:

```text
Found issues in 1 file(s):

/path/to/notes/broken-note.md:
  Line 5, Col 3: Broken link: nonexistent.md
```

## Viewing Statistics

See an overview of your notes and field usage:

```bash
hypha info
```

```text
Config: /Users/you/.hypha
Root:   /Users/you/notes

Topics: 2
Fields: status (1), tags (1)
```

Add `-v` for detailed field values:

```bash
hypha info -v
```

## Renaming Topics

Rename a topic and automatically update all links pointing to it:

```bash
hypha rename "My First Note" "Getting Started Notes"
```

This updates:

- The filename (`my-first-note.md` → `getting-started-notes.md`)
- The title in frontmatter
- All links in other topics that referenced the old name

## Deleting Topics

Remove a topic:

```bash
hypha delete "project-ideas"
```

## Next Steps

- Read the [CLI Reference](../references/cli.md) for all commands and options
- Learn [Query Syntax](../references/query-syntax.md) for advanced searches
- Understand [Frontmatter](../references/frontmatter.md) for metadata schema
- Try the [VS Code Extension](getting-started-vscode.md) for a graphical interface
