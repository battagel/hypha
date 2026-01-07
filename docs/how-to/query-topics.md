# How to Query Topics

Search and filter your topics by frontmatter fields and title text.

## Basic Search

### By Title Text

```bash
hypha search "meeting notes"
```

Matches topics with "meeting" and "notes" in the title.

### By Frontmatter Field

```bash
hypha search "status:active"
```

Matches topics where the `status` field contains "active".

### Combined

```bash
hypha search "status:active meeting"
```

Matches topics that are both active AND have "meeting" in the title.

## Multiple Filters

All filters use AND logic:

```bash
hypha search "status:active priority:high"
```

Matches topics that are active AND high priority.

## Searching Arrays

For array fields like `tags`:

```yaml
---
tags: [work, important, q1]
---
```

Search any element:

```bash
hypha search "tags:work"
hypha search "tags:important"
```

## Partial Matching

Searches are case-insensitive and match partial values:

```bash
hypha search "status:act"      # matches "active"
hypha search "tags:import"     # matches "important"
hypha search "due:2026-03"     # matches any date in March 2026
```

## From VS Code

### Search Command

1. Press `Cmd+K Shift+F` (macOS) or `Ctrl+K Shift+F` (Windows/Linux)
2. Enter your query
3. The sidebar filters to matching topics

### Quick Find

For fuzzy title search only:

1. Press `Cmd+K F` / `Ctrl+K F`
2. Type part of the title
3. Select from matching topics

### Clear Search

Press `Cmd+K C` / `Ctrl+K C` or click the clear icon.

## Example Queries

| Query | Matches |
| ----- | ------- |
| `status:active` | All active topics |
| `tags:work priority:high` | High-priority work items |
| `client:acme` | Topics for Acme client |
| `due:2026-01` | Topics due in January 2026 |
| `meeting Q1` | Topics with "meeting" and "Q1" in title |
| `status:draft rust` | Draft topics about Rust |

## JSON Output

For scripting:

```bash
hypha search "status:active" --json
```

## Common Patterns

### Find All Drafts

```bash
hypha search "status:draft"
```

### Find Overdue Items

```bash
hypha search "due:2025"  # Anything due in 2025 (past)
```

### Find by Project

```bash
hypha search "project:hypha"
```

### Find Untagged Topics

Currently not supported. All topics are returned by `hypha list`, then filter externally.

## Reference

- [CLI Reference: search](../references/cli.md#hypha-search) — Command options
- [Query Syntax Reference](../references/query-syntax.md) — Full syntax details
