# Database Commands

## `notion database get <id>`

Retrieve database metadata (schema, title, properties).

**Endpoint:** `GET /v1/databases/{id}`

| Argument | Required | Description |
| -------- | -------- | ----------- |
| `<id>`   | yes      | Database ID |

```bash
notion database get <db-id>
notion --raw database get <db-id> | jq '.properties | keys'
notion --raw database get <db-id> | jq '.title[0].plain_text'
```
