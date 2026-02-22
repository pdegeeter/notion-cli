# Data Source Commands

## `notion ds get <id>`

Retrieve a data source.

**Endpoint:** `GET /v1/data_sources/{id}`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | Data Source ID |

```bash
notion ds get <ds-id>
notion --raw ds get <ds-id> | jq '.title'
```

## `notion ds create`

Create a data source.

**Endpoint:** `POST /v1/data_sources`

| Option                | Required | Description             |
| --------------------- | -------- | ----------------------- |
| `--parent <id>`       | yes      | Parent page ID          |
| `--title <string>`    | yes      | Data source title       |
| `--properties <json>` | no       | Property schema as JSON |

Supports `--dry-run`.

```bash
# Create a simple data source
notion ds create --parent <page-id> --title "Tasks"

# Create with property schema
notion ds create --parent <page-id> --title "Inventory" \
  --properties '{"Name":{"title":{}},"Count":{"number":{}},"Status":{"select":{"options":[{"name":"In Stock"},{"name":"Out"}]}}}'

# Preview
notion --dry-run ds create --parent <page-id> --title "Test"
```

## `notion ds update <id>`

Update a data source.

**Endpoint:** `PATCH /v1/data_sources/{id}`

| Argument / Option | Required | Description         |
| ----------------- | -------- | ------------------- |
| `<id>`            | yes      | Data Source ID      |
| `--data <json>`   | yes      | Update data as JSON |

Supports `--dry-run`.

```bash
notion ds update <ds-id> \
  --data '{"title":[{"type":"text","text":{"content":"Renamed DS"}}]}'
```

## `notion ds query <id>`

Query a data source.

**Endpoint:** `POST /v1/data_sources/{id}/query`

| Argument / Option | Required | Description           |
| ----------------- | -------- | --------------------- |
| `<id>`            | yes      | Data Source ID        |
| `--filter <json>` | no       | Filter object as JSON |
| `--sorts <json>`  | no       | Sort array as JSON    |

Supports `--page-size`, `--start-cursor`, and `--dry-run`.

```bash
# Query all records
notion ds query <ds-id>

# Query with filter
notion ds query <ds-id> \
  --filter '{"property":"Status","select":{"equals":"Done"}}'

# Query with sort
notion ds query <ds-id> \
  --sorts '[{"property":"Created","direction":"descending"}]'

# Combined filter + sort + pagination
notion ds query <ds-id> \
  --filter '{"property":"Priority","select":{"equals":"High"}}' \
  --sorts '[{"property":"Due Date","direction":"ascending"}]' \
  --page-size 50

# Raw output for scripting
notion --raw ds query <ds-id> | jq '.results[].properties.Name.title[0].plain_text'
```

## `notion ds templates <id>`

List data source templates.

**Endpoint:** `GET /v1/data_sources/{id}/templates`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | Data Source ID |

```bash
notion ds templates <ds-id>
notion --raw ds templates <ds-id> | jq '.results[].id'
```
