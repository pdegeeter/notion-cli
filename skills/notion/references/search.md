# Search

## `notion search <query>`

Search pages and databases by title.

**Endpoint:** `POST /v1/search`

| Argument / Option | Required | Description                             |
| ----------------- | -------- | --------------------------------------- |
| `<query>`         | yes      | Text to search for                      |
| `--filter`, `-f`  | no       | Filter by type: `page` or `data_source` |

Supports `--page-size` and `--start-cursor` for pagination.

### Examples

```bash
# Search all objects
notion search "Project plan"

# Search only pages
notion search "Meeting notes" --filter page

# Search data sources
notion search "Inventory" --filter data_source

# Raw JSON output for scripting
notion --raw search "Budget" | jq '.results[].id'
```
