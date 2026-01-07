# Getting Started with the Hypha VS Code Extension

A step-by-step guide to installing and using the Hypha extension for VS Code.

## Prerequisites

- [VS Code](https://code.visualstudio.com/) 1.85 or later
- [Hypha CLI](getting-started-cli.md) installed and configured

The extension requires the CLI to be installed and in your `PATH`. Install the CLI first if you haven't already.

## Installation

### From VSIX (Packaged)

```bash
cd vscode
npm install
npx vsce package
code --install-extension hypha-vscode-1.0.1-alpha.vsix
```

### From Source

1. Clone the repository:

   ```bash
   git clone https://github.com/battagel/hypha.git
   cd hypha/vscode
   ```

2. Install dependencies and compile:

   ```bash
   npm install
   npm run compile
   ```

3. Open VS Code in the extension directory:

   ```bash
   code .
   ```

4. Press `F5` to launch a new VS Code window with the extension loaded.

## Initial Configuration

1. Open the Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
2. Run **Hypha: Settings**
3. Set the **Root Directory** to your notes folder

Alternatively, the extension will detect your CLI configuration automatically if you've already run `hypha list` in the terminal.

## The Hypha Sidebar

The extension adds a **Hypha** icon to the Activity Bar. Click it to open the Topics panel.

### Topics List

Each topic displays:

- **Title** from the first `# heading`
- **Badges** showing configured frontmatter fields (customizable in settings)
- **Warning icon** if the topic has lint issues

Hover over a topic to see its description and full frontmatter in a tooltip.

### Sorting

Click the **overflow menu** (⋯) in the sidebar header and select **Sort By** to change the sort order:

- **Alphabetical** — Sort by title (A-Z)
- **Recently Modified** — Most recently edited first
- **Recently Created** — Newest topics first

The current sort order is shown in the title bar when not alphabetical.

## Creating Topics

**Keyboard shortcut:** `Cmd+K N` (macOS) / `Ctrl+K N` (Windows/Linux)

Or click the **+** icon in the sidebar header.

Enter a title and the topic opens in the editor with your template.

## Searching Topics

### Filter Search

**Keyboard shortcut:** `Cmd+K Shift+F` / `Ctrl+K Shift+F`

Enter a search query using the same syntax as the CLI:

```text
status:active
tags:work meeting
priority:high project:hypha
```

The sidebar filters to show only matching topics.

**Clear search:** `Cmd+K C` / `Ctrl+K C`

### Content Search

**Keyboard shortcut:** `Cmd+K Shift+S` / `Ctrl+K Shift+S`

Opens VS Code's Search panel scoped to your notes directory. Use this to find topics by content rather than frontmatter.

## Quick Find

**Keyboard shortcut:** `Cmd+K F` / `Ctrl+K F`

Opens a quick pick with fuzzy search across all topic titles. Select a topic to open it.

## Finding Backlinks

Right-click a topic in the sidebar and select **Find Backlinks** to see all topics that link to it.

The sidebar shows a filtered list with a header indicating the backlink target.

## Context Menu Actions

Right-click a topic in the sidebar to access:

| Action | Description |
| ------ | ----------- |
| **Find Backlinks** | Show topics linking to this topic |
| **Copy Markdown Link** | Copy `[Title](filename.md)` to clipboard |
| **Copy Relative Path** | Copy `filename.md` to clipboard |
| **Rename Topic** | Rename and update all references |
| **Delete Topic** | Remove the topic file |

## Linting

**Keyboard shortcut:** `Cmd+K L` / `Ctrl+K L`

Runs lint checks on all topics. Issues appear in:

- The **Problems** panel with clickable locations
- **Warning icons** on affected topics in the sidebar

Issues detected:

- Missing title (no `# heading`)
- Empty content
- Broken links (references to non-existent files)

## Statistics

**Keyboard shortcut:** `Cmd+K I` / `Ctrl+K I`

Shows topic count and field usage in an information message.

## Auto-Sync with Editor

When you open a markdown file, the corresponding topic is automatically highlighted in the sidebar. This helps you stay oriented in large note collections.

## Keyboard Shortcuts Reference

| Shortcut | Command |
| -------- | ------- |
| `Cmd+K O` | Focus Hypha sidebar |
| `Cmd+K N` | Create new topic |
| `Cmd+K F` | Quick find topic |
| `Cmd+K Shift+F` | Search topics |
| `Cmd+K Shift+S` | Search in content |
| `Cmd+K C` | Clear search |
| `Cmd+K R` | Refresh topic list |
| `Cmd+K L` | Lint topics |
| `Cmd+K I` | Show info/statistics |

*On Windows/Linux, use `Ctrl` instead of `Cmd`.*

## Customizing Settings

### Display Fields

Configure which frontmatter fields appear as badges in the sidebar:

1. Open Settings (`Cmd+,` / `Ctrl+,`)
2. Search for "Hypha"
3. Edit **Display Fields** to list fields like `status`, `priority`, `tags`
4. Adjust **Max Badges** to control how many fields show

### Sort Order

Change how topics are sorted in the sidebar using the overflow menu (⋯) → **Sort By**:

- `Alphabetical` — Alphabetically by title (default)
- `Recently Modified` — Most recently modified first
- `Recently Created` — Most recently created first

The current sort order is shown in the panel title when not alphabetical.

## Next Steps

- Learn [Query Syntax](../references/query-syntax.md) for advanced searches
- Understand [Frontmatter](../references/frontmatter.md) for metadata options
- Read about [Architecture](../explanations/architecture.md) to understand how the extension works
