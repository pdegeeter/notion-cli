# Comment Commands

## `notion comment list`

List comments on a block or page.

**Endpoint:** `GET /v1/comments?block_id={id}`

| Option            | Required | Description         |
| ----------------- | -------- | ------------------- |
| `--block-id <id>` | yes      | Block ID or Page ID |

Supports `--page-size` and `--start-cursor` for pagination.

```bash
notion comment list --block-id <page-id>
notion --raw comment list --block-id <page-id> | jq '.results[].rich_text[0].plain_text'
```

## `notion comment create`

Create a comment on a page.

**Endpoint:** `POST /v1/comments`

| Option            | Required | Description          |
| ----------------- | -------- | -------------------- |
| `--page-id <id>`  | yes      | Page ID              |
| `--text <string>` | yes      | Comment text content |

Supports `--dry-run`.

```bash
notion comment create --page-id <page-id> --text "Looks good!"
notion --dry-run comment create --page-id <page-id> --text "Test comment"
```
