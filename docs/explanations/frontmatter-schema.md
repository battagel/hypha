# Frontmatter Schema

Hypha topics use YAML frontmatter for metadata. All fields are user-defined and queryable.

## Format

Frontmatter is a YAML block at the start of a markdown file:

```markdown
---
status: active
tags: [work, important]
priority: high
---

# My Topic

Content here...
```

## Field Types

Any valid YAML is supported:

```yaml
# Strings
status: active
client: acme-corp

# Numbers
priority: 1

# Booleans
published: true

# Arrays
tags:
  - work
  - important

# Dates (stored as strings)
due: 2026-03-15
```

## Querying Fields

All fields are searchable:

```bash
hypha search "status:active"
hypha search "tags:work"
hypha search "priority:1"
hypha search "client:acme"
```

Matching is case-insensitive and partial:

```bash
hypha search "status:act"    # matches "active"
hypha search "due:2026-03"   # matches dates in March
```

For arrays, a match on any element succeeds.

## Field Statistics

See which fields you're using:

```bash
$ hypha info -v

Topics: 149
Fields: status (45), tags (89), client (23)

Field Details:
========================================

status (45 topics):
  active (20)
  archived (15)
  draft (10)
```

## Templates

Define default fields in `.template.md`:

```markdown
---
status: draft
tags: []
---

# {title}

## Overview

## Notes
```

New topics created with `hypha new` start with these fields.

## Title and Description

The **title** comes from the first `# heading`, not frontmatter.

The **description** is the first paragraph after the title:

```markdown
---
status: active
---

# Authentication Service

Handles JWT tokens and session management for the API.

## Details
...
```

The description appears in tooltips in the VS Code sidebar.
