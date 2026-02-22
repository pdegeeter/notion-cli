---
name: notion-cli
description: >
  Use the `notion` CLI to interact with the Notion API from the command line.
  Trigger when the user asks to search, create, read, update, or delete Notion
  pages, blocks, databases, data sources, comments, or users.
  Also trigger for Notion workspace setup, shell completions, or any task
  involving the Notion API.
---

# Notion CLI Skill

Run `notion <command> [options]` to interact with the Notion API.

## Prerequisites

### Install the binary

If `notion` is not found in `$PATH`, install it from source:

```bash
cargo install --path /path/to/notion-cli
```

Or from the GitHub releases:

```bash
# macOS (Apple Silicon)
curl -sL https://github.com/pdegeeter/notion-cli/releases/latest/download/notion-aarch64-apple-darwin.tar.gz | tar xz
sudo mv notion /usr/local/bin/

# Linux (x86_64)
curl -sL https://github.com/pdegeeter/notion-cli/releases/latest/download/notion-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv notion /usr/local/bin/
```

### Configure authentication

The CLI must be configured with a valid Notion API token:

- Run `notion init` for interactive setup, OR
- Set `NOTION_API_TOKEN` environment variable

## Global Options

| Option                       | Description                                  |
| ---------------------------- | -------------------------------------------- |
| `--output pretty\|json\|raw` | Output format (default: `pretty`)            |
| `--raw`                      | Shorthand for `--output raw`                 |
| `--dry-run`                  | Show request without executing (writes only) |
| `--page-size <n>`            | Items per page (max 100)                     |
| `--start-cursor <cursor>`    | Pagination cursor                            |

## Command Groups

| Group      | Reference                                            |
| ---------- | ---------------------------------------------------- |
| search     | [references/search.md](references/search.md)         |
| user       | [references/user.md](references/user.md)             |
| page       | [references/page.md](references/page.md)             |
| block      | [references/block.md](references/block.md)           |
| comment    | [references/comment.md](references/comment.md)       |
| db         | [references/database.md](references/database.md)     |
| ds         | [references/datasource.md](references/datasource.md) |
| formatting | [references/formatting.md](references/formatting.md) |

## Examples

```bash
# Search for pages containing "Meeting"
notion search "Meeting" --filter page

# Get a page as raw JSON for scripting
notion --raw page get abc123 | jq '.properties.Name'

# Create a page under a parent
notion page create --parent <page-id> \
  --properties '{"Name":{"title":[{"text":{"content":"New Page"}}]}}'

# Append a paragraph block to a page
notion block append <page-id> \
  --children '[{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"type":"text","text":{"content":"Hello"}}]}}]'

# Query a data source with filter and sort
notion ds query <ds-id> \
  --filter '{"property":"Status","equals":"Done"}' \
  --sorts '[{"property":"Created","direction":"descending"}]'

# Preview a delete without executing
notion --dry-run block delete <block-id>

# Create a comment on a page
notion comment create --page-id <page-id> --text "Review complete"

# List all users as JSON
notion --raw user list | jq '.'
```

## Tips

- Use `--dry-run` before any destructive or write operation to preview the request.
- Use `--raw` with `jq` for scripting and piping.
- Pagination: pass `--page-size 100` and use `--start-cursor` from the response's `next_cursor` field.
- All IDs can be UUIDs with or without dashes.
