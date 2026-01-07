# Frontmatter Reference

Hypha topics use YAML frontmatter for metadata. All fields are optional and user-defined.

## Format

Frontmatter is a YAML block at the start of a markdown file, delimited by `---`:

```markdown
---
tags:
  - work
  - important
status: active
---

# My Topic

Your content here...
```

## Title

The title is extracted from the **first `# heading`** in the document, not from frontmatter.
If no heading is found, the filename (without `.md`) is used as the title.

```markdown
---
status: active
---

# Q1 2026 Planning

Content starts here...
```

## Description

The **description** is the first non-empty line after the title heading:

```markdown
# Project Alpha

A comprehensive overview of the Alpha project roadmap and milestones.

## Details
...
```

The description is:

- **Indexed** in the JSON output for AI tools and integrations
- **Shown on hover** in the VS Code extension sidebar
- **Displayed inline** next to the topic title in the topic list

If no description line is found (e.g., heading is immediately followed by another heading), the field is `null` in the JSON output.

## Custom Fields

Add any fields you need—they're automatically queryable via `hypha search`:

```yaml
---
tags:
  - work
  - important
status: active
priority: high
project: acme-corp
due: 2026-03-15
attendees:
  - Alice
  - Bob
---
```

```bash
hypha search "tags:work"
hypha search "status:active"
hypha search "priority:high"
hypha search "project:acme"
hypha search "due:2026-03"
```

## Common Field Patterns

Here are some fields other users find useful (but none are required):

| Field     | Example Values             | Purpose                    |
|-----------|----------------------------|----------------------------|
| `tags`    | `[work, important]`        | Categorization labels      |
| `status`  | `draft`, `active`, `done`  | Lifecycle tracking         |
| `priority`| `high`, `medium`, `low`    | Importance ranking         |
| `project` | `hypha`, `acme-corp`       | Project association        |
| `due`     | `2026-03-15`               | Deadlines                  |
| `related` | `[other-topic.md]`         | Links to related topics    |

## Array Syntax

Arrays (like `tags`) can be written multiple ways:

```yaml
# Multi-line
tags:
  - work
  - important

# Inline
tags: [work, important]

# Comma-separated string (also supported)
tags: work, important
```

## Field Types

| YAML Type | Query Behavior                        |
|-----------|---------------------------------------|
| String    | Partial match, case-insensitive       |
| Number    | Converted to string, then matched     |
| Boolean   | Matches `true` or `false`             |
| Array     | Matches if any element matches        |

## Example Topic

```markdown
---
status: active
priority: high
due: 2026-01-15
---

# Q1 2026 Planning

## Objectives

1. Launch new feature
2. Hire two engineers
3. Reduce tech debt by 20%

## Timeline

- Jan 5: Kickoff meeting
- Jan 15: Plan finalized
- Mar 31: Q1 complete

## Notes

See [2026 goals](goals/2026.md) for context.
```

## Customizing the Template

Edit `.template.md` in your notes directory to change the default frontmatter for new topics:

```markdown
---
status: draft
---

# {title}

Short description for summary/AI purposes

## Overview

## Notes
```

The `{title}` placeholder is replaced with the topic title when creating new notes.
