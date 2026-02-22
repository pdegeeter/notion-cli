# Block Commands

## `notion block get <id>`

Retrieve a block.

**Endpoint:** `GET /v1/blocks/{id}`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | Block ID    |

```bash
notion block get abc123
notion --raw block get abc123 | jq '.type'
```

## `notion block children <id>`

List block children.

**Endpoint:** `GET /v1/blocks/{id}/children`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | Block ID    |

Supports `--page-size` and `--start-cursor` for pagination.

```bash
notion block children abc123
notion block children abc123 --page-size 100
notion --raw block children abc123 | jq '.results[].type'
```

## `notion block append <id>`

Append children to a block.

**Endpoint:** `PATCH /v1/blocks/{id}/children`

| Argument / Option   | Required | Description                |
| ------------------- | -------- | -------------------------- |
| `<id>`              | yes      | Parent block ID            |
| `--children <json>` | yes      | Block array as JSON        |
| `--after <id>`      | no       | Insert after this block ID |

Supports `--dry-run`.

```bash
# Append a paragraph
notion block append <page-id> \
  --children '[{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"type":"text","text":{"content":"Hello world"}}]}}]'

# Append a to-do item after a specific block
notion block append <page-id> \
  --children '[{"object":"block","type":"to_do","to_do":{"rich_text":[{"type":"text","text":{"content":"Buy milk"}}],"checked":false}}]' \
  --after <block-id>

# Append a heading
notion block append <page-id> \
  --children '[{"object":"block","type":"heading_2","heading_2":{"rich_text":[{"type":"text","text":{"content":"Section Title"}}]}}]'
```

## `notion block update <id>`

Update a block.

**Endpoint:** `PATCH /v1/blocks/{id}`

| Argument / Option   | Required | Description                    |
| ------------------- | -------- | ------------------------------ |
| `<id>`              | yes      | Block ID                       |
| `--data <json>`     | yes      | Block data as JSON             |
| `--archived <bool>` | no       | Archive or unarchive the block |

Supports `--dry-run`.

```bash
# Update paragraph text
notion block update <block-id> \
  --data '{"paragraph":{"rich_text":[{"type":"text","text":{"content":"Updated text"}}]}}'

# Archive a block
notion block update <block-id> --data '{}' --archived true
```

## `notion block delete <id>`

Delete a block.

**Endpoint:** `DELETE /v1/blocks/{id}`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | Block ID    |

Supports `--dry-run`.

```bash
notion block delete <block-id>
notion --dry-run block delete <block-id>
```
