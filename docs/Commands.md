# Commands Reference

## Global Options

These options apply to all API commands:

| Option | Description | Default |
|--------|-------------|---------|
| `--output <format>` | Output format: `pretty`, `json`, or `raw` | `pretty` |
| `--raw` | Shorthand for `--output raw` | `false` |
| `--dry-run` | Preview write operations without executing | `false` |
| `--page-size <n>` | Items per page for pagination (max 100) | — |
| `--start-cursor <cursor>` | Pagination cursor | — |

---

## `notion init`

Initialize configuration and test connection.

```bash
notion init
```

---

## `notion search`

Search for pages and databases by title.

```bash
notion search <query> [--filter <type>]
```

| Argument / Option | Description |
|-------------------|-------------|
| `<query>` | Search text (required) |
| `--filter`, `-f` | Filter by type: `page` or `data_source` |

```bash
notion search "Meeting Notes"
notion search "Project" --filter page
```

---

## `notion user`

### `notion user me`

Get the current bot user.

```bash
notion user me
```

### `notion user get <id>`

Get a user by ID.

```bash
notion user get <user-id>
```

### `notion user list`

List all users. Supports pagination.

```bash
notion user list
notion user list --page-size 50
```

---

## `notion page`

### `notion page get <id>`

Get page details.

```bash
notion page get <page-id>
notion page get <page-id> --filter-properties title,status
```

| Option | Description |
|--------|-------------|
| `--filter-properties` | Comma-separated list of property names to include |

### `notion page create`

Create a new page.

```bash
notion page create --parent <id> --properties '<json>'
```

| Option | Description |
|--------|-------------|
| `--parent` | Parent page or database ID (required) |
| `--properties` | Properties as JSON (required) |
| `--children` | Child blocks as JSON |
| `--database-parent` | Flag: parent is a database (default: page) |

### `notion page update <id>`

Update a page's properties.

```bash
notion page update <page-id> --properties '<json>'
notion page update <page-id> --archived true
```

| Option | Description |
|--------|-------------|
| `--properties` | Properties to update as JSON (required) |
| `--archived` | Archive or unarchive the page |

### `notion page move <id>`

Move a page to a different parent.

```bash
notion page move <page-id> --parent-type page --to <parent-id>
notion page move <page-id> --parent-type database --to <db-id>
notion page move <page-id> --parent-type workspace
```

| Option | Description |
|--------|-------------|
| `--parent-type` | `page`, `database`, or `workspace` (default: `page`) |
| `--to` | Destination parent ID (required unless workspace) |

### `notion page property <page-id> <property-id>`

Get a specific property value. Supports pagination.

```bash
notion page property <page-id> <property-id>
```

---

## `notion block`

### `notion block get <id>`

Get block details.

```bash
notion block get <block-id>
```

### `notion block children <id>`

List child blocks. Supports pagination.

```bash
notion block children <block-id>
notion block children <block-id> --page-size 50
```

### `notion block append <id>`

Append child blocks.

```bash
notion block append <block-id> --children '<json>'
notion block append <block-id> --children '<json>' --after <block-id>
```

| Option | Description |
|--------|-------------|
| `--children` | Array of blocks as JSON (required) |
| `--after` | Insert after this block ID |

### `notion block update <id>`

Update a block.

```bash
notion block update <block-id> --data '<json>'
notion block update <block-id> --data '{}' --archived true
```

| Option | Description |
|--------|-------------|
| `--data` | Block data as JSON (required) |
| `--archived` | Archive or unarchive the block |

### `notion block delete <id>`

Delete a block.

```bash
notion block delete <block-id>
```

---

## `notion comment`

### `notion comment list`

List comments on a block or page. Supports pagination.

```bash
notion comment list --block-id <id>
```

### `notion comment create`

Create a comment on a page.

```bash
notion comment create --page-id <id> --text "Your comment"
```

| Option | Description |
|--------|-------------|
| `--page-id` | Page ID (required) |
| `--text` | Comment text (required) |

---

## `notion database`

### `notion database get <id>`

Get database metadata and schema.

```bash
notion database get <database-id>
```

---

## `notion datasource` (Data Sources)

### `notion datasource get <id>`

Get a data source.

```bash
notion datasource get <ds-id>
```

### `notion datasource create`

Create a new data source.

```bash
notion datasource create --parent <page-id> --title "My Data Source"
notion datasource create --parent <page-id> --title "My DB" --properties '<json>'
```

| Option | Description |
|--------|-------------|
| `--parent` | Parent page ID (required) |
| `--title` | Title (required) |
| `--properties` | Property schema as JSON |

### `notion datasource update <id>`

Update a data source.

```bash
notion datasource update <ds-id> --data '<json>'
```

### `notion datasource query <id>`

Query a data source with filters and sorting.

```bash
notion datasource query <ds-id>
notion datasource query <ds-id> --filter '<json>' --sorts '<json>'
```

| Option | Description |
|--------|-------------|
| `--filter` | Filter as JSON |
| `--sorts` | Sort array as JSON |

### `notion datasource templates <id>`

List templates for a data source.

```bash
notion datasource templates <ds-id>
```

---

## `notion completions <shell>`

Generate shell completions. See [Installation](Installation.md#shell-completions).

```bash
notion completions bash|zsh|fish|powershell|elvish
```

## `notion manpage`

Generate a Unix man page. See [Installation](Installation.md#man-page).

```bash
notion manpage
```
