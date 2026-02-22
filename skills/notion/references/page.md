# Page Commands

## `notion page get <id>`

Retrieve a page.

**Endpoint:** `GET /v1/pages/{id}`

| Argument / Option            | Required | Description                             |
| ---------------------------- | -------- | --------------------------------------- |
| `<id>`                       | yes      | Page ID                                 |
| `--filter-properties <list>` | no       | Comma-separated property IDs to include |

```bash
notion page get abc123
notion page get abc123 --filter-properties title,status
notion --raw page get abc123 | jq '.properties'
```

## `notion page create`

Create a new page.

**Endpoint:** `POST /v1/pages`

| Option                | Required | Description                                |
| --------------------- | -------- | ------------------------------------------ |
| `--parent <id>`       | yes      | Parent page or database ID                 |
| `--properties <json>` | yes      | Page properties as JSON                    |
| `--children <json>`   | no       | Child blocks as JSON                       |
| `--database-parent`   | no       | Flag: parent is a database (default: page) |

Supports `--dry-run`.

```bash
# Create a simple page under a parent page
notion page create --parent <page-id> \
  --properties '{"Name":{"title":[{"text":{"content":"My Page"}}]}}'

# Create a page in a database
notion page create --parent <db-id> --database-parent \
  --properties '{"Name":{"title":[{"text":{"content":"Task 1"}}]},"Status":{"select":{"name":"To Do"}}}'

# Create a page with content blocks
notion page create --parent <page-id> \
  --properties '{"Name":{"title":[{"text":{"content":"With Content"}}]}}' \
  --children '[{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"type":"text","text":{"content":"First paragraph"}}]}}]'

# Preview without creating
notion --dry-run page create --parent <page-id> \
  --properties '{"Name":{"title":[{"text":{"content":"Test"}}]}}'
```

## `notion page update <id>`

Update page properties.

**Endpoint:** `PATCH /v1/pages/{id}`

| Argument / Option     | Required | Description                        |
| --------------------- | -------- | ---------------------------------- |
| `<id>`                | yes      | Page ID                            |
| `--properties <json>` | yes      | New properties as JSON             |
| `--archived <bool>`   | no       | Archive (`true`) or unarchive page |

Supports `--dry-run`.

```bash
# Update a property
notion page update abc123 \
  --properties '{"Status":{"select":{"name":"Done"}}}'

# Archive a page
notion page update abc123 --properties '{}' --archived true
```

## `notion page move <id>`

Move a page to a different parent.

**Endpoint:** `POST /v1/pages/{id}/move`

| Argument / Option        | Required | Description                                        |
| ------------------------ | -------- | -------------------------------------------------- |
| `<id>`                   | yes      | Page ID to move                                    |
| `--to <id>`              | yes\*    | Destination parent ID (\*not needed for workspace) |
| `--parent-type <string>` | no       | `page` (default), `database`, or `workspace`       |

Supports `--dry-run`.

```bash
# Move to another page
notion page move abc123 --to def456

# Move to a database
notion page move abc123 --to db-id --parent-type database

# Move to workspace root
notion page move abc123 --parent-type workspace
```

## `notion page property <page_id> <property_id>`

Get a page property value.

**Endpoint:** `GET /v1/pages/{page_id}/properties/{property_id}`

| Argument        | Required | Description |
| --------------- | -------- | ----------- |
| `<page_id>`     | yes      | Page ID     |
| `<property_id>` | yes      | Property ID |

Supports `--page-size` and `--start-cursor` for pagination.

```bash
notion page property abc123 title
notion --raw page property abc123 status | jq '.select.name'
```
