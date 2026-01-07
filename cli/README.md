# Hypha CLI

A fast CLI for managing markdown notes with rich metadata.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary at ./target/release/hypha
```

## Usage

```bash
# Create a new topic
hypha new "My Topic"
hypha new "Project Plan" --no-edit

# List all topics
hypha list

# Search topics
hypha search "meeting notes"
hypha search "status:active priority:high"

# Open a topic in $EDITOR
hypha open "my-topic"
hypha open my-topic.md

# Delete a topic
hypha delete "my-topic"

# Lint for issues (missing title, empty content)
hypha lint
hypha lint --json  # JSON output

# Show backlinks to a topic
hypha backlinks "my-topic"

# Rename a topic and update all links
hypha rename "old-title" "new-title"

# Show setup info and statistics
hypha info

# Specify a different root directory
hypha --root ~/notes list
```

## Query Syntax

Searches support key-value filters and free-text terms:

```bash
# Key-value filters (match frontmatter fields)
hypha search "status:active"
hypha search "status:active priority:high"

# Free-text (matches title)
hypha search "quarterly review"

# Combined
hypha search "status:active quarterly review"
```

Any frontmatter field is queryable (e.g., `priority:high`, `due:2026`, `tags:work`).

## Frontmatter Schema

Topics use YAML frontmatter. Only `title` is in the default template—add any fields you need:

```yaml
---
title: My Topic
status: active
priority: high
due: 2026-03-01
tags:
  - work
  - important
---

Your content here...
```

All fields are optional and queryable. Customize `.template.md` in your notes directory to change the default frontmatter for new topics.

## Environment Variables

| Variable | Description             | Default |
|----------|-------------------------|---------|
| `EDITOR` | Editor for `hypha open` | `vim`   |
