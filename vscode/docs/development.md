# VS Code Extension Development

Guide for developing the Hypha VS Code extension.

## Prerequisites

- [Node.js](https://nodejs.org/) (18+)
- [VS Code](https://code.visualstudio.com/)
- `hypha` CLI installed (see [CLI development](../../cli/docs/development.md))

## Setup

Install dependencies:

```bash
cd vscode
npm install
```

## Development Workflow

### Compile TypeScript

```bash
npm run compile
```

### Watch Mode (Auto-Compile)

```bash
npm run watch
```

### Linting

```bash
npm run lint
```

### Launch Extension Development Host

1. Open the `vscode/` folder in VS Code
2. Press `F5` (or Run → Start Debugging)
3. A new VS Code window opens with the extension loaded

Or from command line:

```bash
code --extensionDevelopmentPath=$PWD
```

### Test the Extension

In the Extension Development Host:

1. **Sidebar**: Look for the Hypha icon in the Activity Bar (left side)
2. **Commands**: Open Command Palette (`Cmd+Shift+P`) and type "Hypha"
3. **Search**: Click the search icon in the Hypha sidebar title bar

Available commands:

- `Hypha: New Topic` — Create a new topic
- `Hypha: Quick Find Topic` — Fuzzy search and open
- `Hypha: Search Topics` — Filter topics with query
- `Hypha: Refresh` — Refresh the topic list
- `Hypha: Lint Topics` — Check for issues
- `Hypha: Show Info` — Display setup info and statistics
- `Hypha: Settings` — Configure Hypha

### View Extension Logs

In the Extension Development Host:

1. Open Output panel (`Cmd+Shift+U`)
2. Select "Log (Extension Host)" from dropdown
3. Look for "Hypha extension" messages

## Configuration

The extension reads these settings:

| Setting              | Description                                      | Default           |
|----------------------|--------------------------------------------------|-------------------|
| `hypha.binaryPath`   | Path to hypha binary                             | (uses PATH)       |
| `hypha.rootDir`      | Root directory for notes                         | (uses CLI config) |
| `hypha.displayFields`| Frontmatter fields to show as badges             | `[]`              |
| `hypha.maxBadges`    | Maximum badges before showing +N                 | `3`               |

## Packaging

Build a `.vsix` package:

```bash
npm install -g @vscode/vsce
vsce package
```

Install locally:

```bash
code --install-extension hypha-vscode-0.1.0.vsix
```
