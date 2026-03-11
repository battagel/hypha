# Changelog

All notable changes to Hypha will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0]

### Added

- **VS Code**: Published to the VS Code Marketplace (`battagel.hypha-vscode`)
- **VS Code**: Extension now bundles platform-specific CLI binaries — no separate CLI installation required
- **VS Code**: Provide option to create new topic when there are no search results

### Changed

- **VS Code**: Updated extension icon to 320px variant
- **VS Code**: Extension README rewritten as a marketplace-facing page
- **VS Code**: Marketplace installation instructions added to getting started tutorial
- **VS Code**: Architecture documentation moved to `vscode/docs/development.md`
- **CI**: Release workflow now builds CLI binaries first and bundles them into the `.vsix`
- **CI**: `.vscodeignore` updated to exclude dev files (`src/`, `docs/`, `scripts/`, `node_modules/`, etc.) from packaged extension

## [1.1.0]

### Added

- **CLI**: `hypha search` command for regex body content search across all notes
- **VS Code**: Search Content command (`Cmd+K S`) with live QuickPick results
- **VS Code**: Toolbar button for Search Content in Hypha panel

### Changed

- **CLI**: Renamed `search` command to `find` (searches by title/metadata)
- **CLI**: New `search` command searches body content (replaces grep concept)
- **VS Code**: Keybindings aligned - Find Topic (`Cmd+K F`), Filter View (`Cmd+K Shift+F`), Search Content (`Cmd+K S`)
- **CLI**: Search results use ordered JSON output (indexmap)

### Fixed

- **VS Code**: QuickPick now properly displays search results by matching on detail content

## [1.0.0]

### Added

- **CLI**: Core commands - `new`, `list`, `search`, `delete`, `rename`, `backlinks`, `lint`, `info`
- **CLI**: Frontmatter schema validation with customizable fields
- **CLI**: Query syntax for filtering topics by metadata
- **CLI**: JSON output support for all commands
- **CLI**: Sort options (alpha, modified, created)
- **VS Code**: Topic tree view with filtering and sorting
- **VS Code**: Quick Find command (`Cmd+K F`) for topic navigation
- **VS Code**: Filter View for narrowing topic list
- **VS Code**: New Topic, Delete, Rename commands
- **VS Code**: Backlinks discovery
- **VS Code**: Copy Link and Copy Path commands
- **VS Code**: Markdown preview integration
- **Docs**: Comprehensive documentation following Diátaxis framework

---

<!-- Template for new releases:

## [X.Y.Z]

### Added
- New features

### Changed
- Changes to existing functionality

### Fixed
- Bug fixes

### Removed
- Removed features

[X.Y.Z]: https://github.com/battagel/hypha/compare/vX.Y.Z-1...vX.Y.Z
-->
