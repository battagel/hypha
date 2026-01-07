# CLI Reference

Complete reference for all `hypha` commands.

## Global Options

| Option          | Short | Description                          |
|-----------------|-------|--------------------------------------|
| `--root <PATH>` | `-r`  | Override root directory              |
| `--help`        | `-h`  | Print help information               |
| `--version`     | `-V`  | Print version                        |

## Commands

### `hypha new`

Create a new topic.

```bash
hypha new <TITLE> [OPTIONS]
```

**Arguments:**

| Argument  | Description              |
|-----------|--------------------------|
| `<TITLE>` | Title of the new topic   |

**Options:**

| Option      | Short | Description                           |
|-------------|-------|---------------------------------------|
| `--no-edit` | `-n`  | Don't open the file in editor         |

**Examples:**

```bash
hypha new "My Topic"
hypha new "Project Ideas" --no-edit
```

**See also:** [How to Create a Topic](../how-to/create-topic.md)

---

### `hypha list`

List all topics.

```bash
hypha list [OPTIONS]
```

**Options:**

| Option           | Short | Description                                |
|------------------|-------|--------------------------------------------|
| `--sort <ORDER>` | `-s`  | Sort order: `alpha`, `modified`, `created` |
| `--json`         |       | Output as JSON                             |

**Examples:**

```bash
hypha list
hypha list --sort modified
hypha list --sort created --json
```

---

### `hypha search`

Search topics by query.

```bash
hypha search <QUERY> [OPTIONS]
```

**Arguments:**

| Argument  | Description                                        |
|-----------|----------------------------------------------------|
| `<QUERY>` | Search query (see [Query Syntax](query-syntax.md)) |

**Options:**

| Option           | Short | Description                                |
|------------------|-------|--------------------------------------------|
| `--sort <ORDER>` | `-s`  | Sort order: `alpha`, `modified`, `created` |
| `--json`         |       | Output as JSON                             |

**Examples:**

```bash
hypha search "meeting notes"
hypha search "tags:work status:active"
hypha search "priority:high"
```

**See also:** [How to Query Topics](../how-to/query-topics.md)

---

### `hypha open`

Open a topic in your default editor.

```bash
hypha open <TOPIC>
```

**Arguments:**

| Argument  | Description                     |
|-----------|---------------------------------|
| `<TOPIC>` | Topic title, slug, or filename  |

Uses the `$EDITOR` environment variable (defaults to `vim`).

**Examples:**

```bash
hypha open "My Topic"
hypha open my-topic
hypha open my-topic.md
---

### `hypha delete`

Delete a topic.

```bash
hypha delete <TOPIC>
```

**Arguments:**

| Argument  | Description                     |
|-----------|---------------------------------|
| `<TOPIC>` | Topic title, slug, or filename  |

**Examples:**

```bash
hypha delete "My Topic"
hypha delete my-topic
hypha delete my-topic.md
```

---

### `hypha lint`

Lint topics for issues (missing title, empty content).

```bash
hypha lint [OPTIONS]
```

**Options:**

| Option   | Description          |
|----------|----------------------|
| `--json` | Output as JSON       |

Scans all topics and reports any with warnings:

- Missing title (no `# Heading`)
- Empty content

**Output:**

```text
Found issues in 2 file(s):

/path/to/note.md:
  - Empty content
  - Missing title (no # heading)

/path/to/other.md:
  - Missing title (no # heading)
```

Exit code is `1` if issues are found.

---

### `hypha backlinks`

Show topics that link to a specific topic.

```bash
hypha backlinks <TOPIC> [OPTIONS]
```

**Arguments:**

| Argument  | Description                     |
|-----------|---------------------------------|
| `<TOPIC>` | Topic title or filename         |

**Options:**

| Option   | Description          |
|----------|----------------------|
| `--json` | Output as JSON       |

**Examples:**

```bash
hypha backlinks "My Topic"
hypha backlinks my-topic.md
```

**See also:** [How to Find Backlinks](../how-to/find-backlinks.md)

---

### `hypha rename`

Rename a topic and update all links.

```bash
hypha rename <FROM> <TO>
```

**Arguments:**

| Argument | Description                     |
|----------|---------------------------------|
| `<FROM>` | Current topic title or filename |
| `<TO>`   | New title                       |

Updates the topic's filename, frontmatter title, and all references in other topics.

**Examples:**

```bash
hypha rename "Old Title" "New Title"
hypha rename old-title.md "New Title"
```

---

### `hypha info`

Show setup info, topic count, and field usage.

```bash
hypha info [OPTIONS]
```

**Options:**

| Option       | Short | Description                |
|--------------|-------|----------------------------|
| `--verbose`  | `-v`  | Show detailed field values |

**Output:**

```text
Config: /Users/you/.hypha
Root:   /Users/you/notes

Topics: 42
Fields: status (35), tags (28), priority (12)
```

With `--verbose`, shows which values are used for each field:

```text
Field Details:
========================================

status (35 topics):
  active (20)
  archived (10)
  draft (5)

tags (28 topics):
  work (15)
  personal (8)
  ideas (5)
```

## Environment Variables

| Variable | Description             | Default |
|----------|-------------------------|---------|
| `EDITOR` | Editor for `hypha open` | `vim`   |

## Exit Codes

| Code | Meaning                                 |
|------|-----------------------------------------|
| `0`  | Success                                 |
| `1`  | Error (issues found, topic not found)   |
