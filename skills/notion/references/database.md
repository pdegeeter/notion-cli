# Database Commands

## `notion db get <id>`

Retrieve database metadata (schema, title, properties).

**Endpoint:** `GET /v1/databases/{id}`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | Database ID |

```bash
notion db get <db-id>
notion --raw db get <db-id> | jq '.properties | keys'
notion --raw db get <db-id> | jq '.title[0].plain_text'
```
