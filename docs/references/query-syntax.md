# Query Syntax Reference

Hypha's find command supports key-value filters and free-text terms.

## Basic Syntax

```text
hypha find "<filters> <terms>"
```

- **Filters**: `key:value` pairs that match specific frontmatter fields
- **Terms**: Free-text words that match the title

## Examples

```bash
# Single filter
hypha find "status:active"

# Multiple filters (AND logic)
hypha find "status:active priority:high"

# Free-text only
hypha find "meeting notes"

# Combined
hypha find "status:active quarterly review"
```

## Field Filters

Any frontmatter field is queryable:

```yaml
---
title: Q1 Planning
priority: high
project: hypha
due: 2026-03-01
---
```

```bash
hypha find "priority:high"
hypha find "project:hypha"
hypha find "due:2026"
```

## Matching Behavior

### Filters

- **Case-insensitive**: `status:Active` matches `active`
- **Partial match**: `status:act` matches `active`
- **Arrays**: For array fields like `tags: [work, important]`, matches if any element matches
- **Multiple filters**: `status:active priority:high` requires both to match (AND)

### Free-text Terms

- **Case-insensitive**: `Meeting` matches `meeting`
- **Partial match**: `meet` matches `meeting`
- **Multiple terms**: All terms must match (AND)
- **Searches in**: title

## Current Limitations

- No `OR` logic (yet)
- No negation like `-status:draft` (yet)
- No quoted phrases like `"exact match"` (yet)
- No date comparisons like `due:>2026-01-01` (yet)

## Query Grammar (Informal)

```text
query     = (filter | term)*
filter    = key ":" value
key       = [a-zA-Z]+
value     = [^\s]+
term      = [^\s:]+
```

Whitespace separates tokens. A token with `:` is a filter; otherwise a term.

## See Also

- [How to Query Topics](../how-to/query-topics.md) — Practical examples and workflows
- [CLI Reference: find](cli.md#hypha-find) — Command options
