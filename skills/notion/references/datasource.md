# Data Source Commands

## `notion datasource get <id>`

Retrieve a data source.

**Endpoint:** `GET /v1/data_sources/{id}`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | Data Source ID |

```bash
notion datasource get <ds-id>
notion --raw datasource get <ds-id> | jq '.title'
```

## `notion datasource create`

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
notion datasource create --parent <page-id> --title "Tasks"

# Create with property schema
notion datasource create --parent <page-id> --title "Inventory" \
  --properties '{"Name":{"title":{}},"Count":{"number":{}},"Status":{"select":{"options":[{"name":"In Stock"},{"name":"Out"}]}}}'

# Preview
notion --dry-run datasource create --parent <page-id> --title "Test"
```

## `notion datasource update <id>`

Update a data source.

**Endpoint:** `PATCH /v1/data_sources/{id}`

| Argument / Option | Required | Description         |
| ----------------- | -------- | ------------------- |
| `<id>`            | yes      | Data Source ID      |
| `--data <json>`   | yes      | Update data as JSON |

Supports `--dry-run`.

```bash
notion datasource update <ds-id> \
  --data '{"title":[{"type":"text","text":{"content":"Renamed DS"}}]}'
```

## `notion datasource query <id>`

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
notion datasource query <ds-id>

# Query with filter
notion datasource query <ds-id> \
  --filter '{"property":"Status","select":{"equals":"Done"}}'

# Query with sort
notion datasource query <ds-id> \
  --sorts '[{"property":"Created","direction":"descending"}]'

# Combined filter + sort + pagination
notion datasource query <ds-id> \
  --filter '{"property":"Priority","select":{"equals":"High"}}' \
  --sorts '[{"property":"Due Date","direction":"ascending"}]' \
  --page-size 50

# Raw output for scripting
notion --raw datasource query <ds-id> | jq '.results[].properties.Name.title[0].plain_text'
```

## `notion datasource templates <id>`

List data source templates.

**Endpoint:** `GET /v1/data_sources/{id}/templates`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | Data Source ID |

```bash
notion datasource templates <ds-id>
notion --raw datasource templates <ds-id> | jq '.results[].id'
```
