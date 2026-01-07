# Linking Between Topics

Hypha uses standard Markdown links to connect topics.

## Link Syntax

```markdown
See [My Topic](my-topic.md) for details.
```

This creates a clickable link to `my-topic.md` in the same directory.

## Relative Paths

Links are resolved relative to the current file:

```markdown
# From notes/project.md

[Setup Guide](setup-guide.md)           # Same directory
[Auth Docs](api/authentication.md)       # Subdirectory
[Overview](../overview.md)               # Parent directory
```

## Backlinks

Find all topics that link to a given topic:

```bash
hypha backlinks "my-topic"
```

Output:

```text
project-plan.md
  Project Plan
meeting-notes.md
  Meeting Notes
```

The VS Code extension shows backlinks via right-click → **Find Backlinks**.

## Link Validation

The linter checks that linked files exist:

```bash
$ hypha lint
my-topic.md:
  Line 5, Col 3: Broken link: nonexistent.md
```

Warnings appear in the VS Code Problems panel with clickable locations.

## Copying Links

In the VS Code extension, right-click a topic and select:

- **Copy Markdown Link** → `[Topic Title](filename.md)`
- **Copy Relative Path** → `filename.md`

## Renaming Topics

When you rename a topic, Hypha updates all links pointing to it:

```bash
hypha rename "Old Title" "New Title"
```

This updates:

- The filename
- The title in frontmatter
- All `[text](old-title.md)` references in other files
