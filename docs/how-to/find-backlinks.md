# How to Find Backlinks

Find all topics that link to a specific topic.

## Why Backlinks Matter

Backlinks help you:

- Discover connections between topics
- See how ideas relate to each other
- Find topics that reference a concept

## From the CLI

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

### JSON Output

For scripting or integration:

```bash
hypha backlinks "my-topic" --json
```

```json
[
  {
    "path": "/path/to/notes/project-plan.md",
    "title": "Project Plan",
    "description": "Q1 2026 project roadmap"
  }
]
```

### Match by Filename

You can use the filename instead of the title:

```bash
hypha backlinks my-topic.md
```

## From VS Code

### Right-Click Menu

1. Find the topic in the Hypha sidebar
2. Right-click the topic
3. Select **Find Backlinks**

The sidebar filters to show only topics that link to your selected topic.

### Clear the Filter

Click **Clear Search** in the sidebar header (or press `Cmd+K C` / `Ctrl+K C`).

## How It Works

Hypha scans all topics for Markdown links pointing to the target file:

```markdown
<!-- These links in other-topic.md create backlinks to my-topic -->
See [My Topic](my-topic.md) for details.
Check the [setup guide](my-topic.md#setup).
```

Both standard links and links with anchors are detected.

## Example Workflow

1. You're reading `authentication.md`
2. Run `hypha backlinks authentication` or right-click → Find Backlinks
3. See that `api-gateway.md` and `user-service.md` reference it
4. Navigate to those topics to understand the broader context

## Reference

See [CLI Reference: backlinks](../references/cli.md#hypha-backlinks) for all options.
