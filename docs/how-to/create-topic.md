# How to Create a Topic

Create new topics from the CLI or VS Code extension.

## From the CLI

```bash
hypha new "My Topic Title"
```

This:

1. Generates a slug from the title (`my-topic-title`)
2. Creates `my-topic-title.md` with your template
3. Opens the file in your `$EDITOR`

### Skip Opening the Editor

```bash
hypha new "My Topic Title" --no-edit
```

### Use a Different Notes Directory

```bash
hypha --root ~/work/notes new "Meeting Notes"
```

## From VS Code

### Keyboard Shortcut

Press `Cmd+K N` (macOS) or `Ctrl+K N` (Windows/Linux).

### From the Sidebar

1. Click the **+** icon in the Hypha sidebar header
2. Enter the topic title when prompted
3. The new topic opens in the editor

## What Gets Created

A new file with frontmatter from your template:

```markdown
---
title: My Topic Title
---

# My Topic Title
```

## Customizing the Template

Edit `.template.md` in your notes directory:

```markdown
---
status: draft
tags: []
---

# {title}

## Overview

## Notes
```

The `{title}` placeholder is replaced with your topic title.

## Next Steps

- Add content and frontmatter fields
- Link to other topics with `[text](other-topic.md)`
- Search your topics with [queries](query-topics.md)

## Reference

See [CLI Reference: new](../references/cli.md#hypha-new) for all options.
