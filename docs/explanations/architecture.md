# Architecture Overview

Hypha follows a simple architecture where the CLI is the source of truth
and editor integrations are thin wrappers.

## System Diagram

```mermaid
flowchart TB
    subgraph Editors["Editor Integrations"]
        VSC[VS Code Extension]
    end

    subgraph CLI["hypha CLI (Rust)"]
        CMD[Commands]
        CORE[Core Modules]
    end

    subgraph Storage["File System"]
        MD[("Markdown Files<br>*.md")]
    end

    VSC -->|"Shells out<br>(JSON output)"| CMD
    CMD --> CORE
    CORE -->|"Read/write"| MD
```

## CLI Module Structure

```mermaid
flowchart LR
    subgraph Commands["commands/"]
        NEW[new]
        LIST[list]
        SEARCH[search]
        OPEN[open]
        DELETE[delete]
        LINT[lint]
        BACKLINKS[backlinks]
        RENAME[rename]
        INFO[info]
    end

    subgraph Core["core/"]
        INDEX[index]
        TOPIC[topic]
        MARKDOWN[markdown]
        QUERY[query]
        FRONT[frontmatter]
        TEMPLATE[template]
    end

    NEW --> TOPIC
    NEW --> TEMPLATE
    LIST --> INDEX
    SEARCH --> INDEX
    SEARCH --> QUERY
    LINT --> INDEX
    BACKLINKS --> INDEX
    RENAME --> INDEX
    INFO --> INDEX

    INDEX --> TOPIC
    TOPIC --> MARKDOWN
    TOPIC --> FRONT
    MARKDOWN -->|"pulldown-cmark"| MD[Markdown AST]
```

## Component Responsibilities

### CLI (`cli/`)

The Rust binary handles all core logic:

- **commands/** - One file per CLI command (new, list, search, etc.)
- **core/** - Shared modules for indexing, parsing, and querying
- **main.rs** - Entry point with clap argument parsing

### VS Code Extension (`vscode/`)

Thin wrappers that:

1. Spawn the `hypha` binary with arguments
2. Parse the output
3. Present results in native UI

No business logic, just presentation.

## Data Flow

### Creating a Topic

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant FS as File System

    User->>CLI: hypha new "My Topic"
    CLI->>CLI: Generate slug (my-topic)
    CLI->>CLI: Build frontmatter template
    CLI->>FS: Write my-topic.md
    CLI->>User: Created: my-topic.md
```

### Searching Topics

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant FS as File System

    User->>CLI: hypha search "status:active meeting"
    CLI->>FS: Walk directory, read *.md files
    CLI->>CLI: Parse frontmatter for each file
    CLI->>CLI: Build in-memory index
    CLI->>CLI: Parse query into filters + terms
    CLI->>CLI: Match topics against query
    CLI->>User: Display matching topics
```

### Parsing a Topic

```mermaid
flowchart LR
    subgraph Input
        FILE[".md file"]
    end

    subgraph Parsing
        FRONT["frontmatter::parse()"]
        MARKDOWN["markdown::parse()"]
    end

    subgraph Output
        TOPIC["Topic struct"]
    end

    FILE --> FRONT
    FRONT -->|"metadata\n(YAML)"| TOPIC
    FRONT -->|"body\n(markdown)"| MARKDOWN
    MARKDOWN -->|"title, description,\nlinks with line numbers"| TOPIC
```

The parser extracts:

- **Title**: First H1 heading
- **Description**: First paragraph after title
- **Links**: All local file links with line numbers (for lint)

## Design Principles

### 1. Files as Source of Truth

- All data lives in plain markdown files
- No database, no sync service, no lock-in
- Files are portable and version-controllable

### 2. CLI as Core Engine

- Single implementation of indexing/search logic
- Editor integrations are thin and interchangeable
- Easy to script and automate

### 3. Index on Demand

- No persistent index or daemon
- Re-index on every command (~50-200ms for typical usage)
- Simplicity over premature optimization

### 4. Extensible Frontmatter

- Only `title` in default template
- All fields are user-defined and queryable
- No schema enforcement—flexibility over rigidity

## Performance Characteristics

| Operation        | Typical Time | Notes             |
|------------------|--------------|-------------------|
| Index 100 files  | ~50ms        | Cold start        |
| Index 1000 files | ~200ms       | Still instant     |
| Search           | <10ms        | After indexing    |
| Create topic     | <5ms         | Single file write |

For 5000+ files, consider adding a daemon mode (not yet implemented).

## Future Considerations

- **Watch mode**: Daemon for large note collections
- **Web UI**: Browser-based interface using same CLI
- **Sync**: Optional sync layer (git-based or custom)
