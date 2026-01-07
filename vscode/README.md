# Hypha VS Code Extension

VS Code integration for Hypha markdown notes.

## Prerequisites

- Hypha CLI installed and in your `PATH`
- VS Code 1.85 or later

## Features

### Sidebar

- **Topic list** with configurable frontmatter badges and sorting
- **Search** with field filters (`status:active`, `tags:work`)
- **Quick Find** for fuzzy title search
- **Backlinks** view showing topics that link to selected topic
- **Warning icons** for topics with lint issues
- **Auto-sync** — Highlights current file in tree

### Commands

| Command          | Keybinding       | Description                      |
| ---------------- | ---------------- | -------------------------------- |
| Focus Sidebar    | `Cmd+K O`        | Focus the Hypha topics view      |
| New Topic        | `Cmd+K N`        | Create a new topic               |
| Quick Find       | `Cmd+K F`        | Fuzzy search topic titles        |
| Search           | `Cmd+K Shift+F`  | Search with filters              |
| Search in Content| `Cmd+K Shift+S`  | Full-text search in note content |
| Clear Search     | `Cmd+K C`        | Clear search filter              |
| Refresh          | `Cmd+K R`        | Refresh topic list               |
| Lint             | `Cmd+K L`        | Check for issues                 |
| Info             | `Cmd+K I`        | Show statistics                  |

*On Windows/Linux, use `Ctrl` instead of `Cmd`.*

### Context Menu

Right-click a topic for:

- **Find Backlinks** — Show topics linking to this one
- **Copy Markdown Link** — Copy `[Title](filename.md)`
- **Copy Relative Path** — Copy `filename.md`
- **Rename Topic** — Rename and update all references
- **Delete Topic** — Remove the topic file

### Diagnostics

Lint warnings appear in the Problems panel with clickable locations for:

- Missing titles
- Empty content
- Broken links

## Configuration

| Setting               | Description                                     | Default |
| --------------------- | ----------------------------------------------- | ------- |
| `hypha.displayFields` | Frontmatter fields to show as badges            | `[]`    |
| `hypha.maxBadges`     | Maximum badges per topic                        | `3`     |
| `hypha.binaryPath`    | Path to hypha binary                            | `hypha` |
| `hypha.rootDir`       | Notes directory (auto-detected from CLI config) | —       |
| `hypha.sortOrder`     | Sort order: `alpha`, `modified`, `created`      | `alpha` |

## Development

```bash
npm install
npm run watch    # Compile on change
npm run lint     # Check for issues
```

Press `F5` to launch Extension Development Host.

## Architecture

The extension is a thin wrapper around the Hypha CLI:

1. Spawns `hypha` commands with JSON output flags
2. Parses JSON responses
3. Presents results in VS Code UI

No business logic — all functionality comes from the CLI.

See [Architecture](../docs/explanations/architecture.md) for details.
