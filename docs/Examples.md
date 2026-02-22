# Usage Examples

## Basic Workflow

```bash
# Initialize and connect
notion init

# Check connection
notion user me

# Search for content
notion search "Project Tracker"

# Get a page
notion page get abc123-def456

# List comments on a page
notion comment list --block-id abc123-def456
```

## Output Formats

```bash
# Pretty-printed (default)
notion user me

# Compact JSON for piping
notion --raw user me

# Explicit format selection
notion --output json user me
```

## Dry-Run Mode

Preview write operations before executing:

```bash
# Preview a page creation
notion --dry-run page create \
  --parent page-123 \
  --properties '{"Name":{"title":[{"text":{"content":"New Page"}}]}}'

# Preview a block deletion
notion --dry-run block delete block-456
```

## Scripting with jq

```bash
# Extract all page IDs from search results
notion --raw search "Notes" | jq -r '.results[].id'

# Count users of a specific type
notion --raw user list --page-size 100 \
  | jq '[.results[] | select(.type == "person")] | length'

# Create a page and capture its ID
page_id=$(notion --raw page create \
  --parent db-id \
  --properties '{"Name":{"title":[{"text":{"content":"Auto-created"}}]}}' \
  | jq -r '.id')
echo "Created page: $page_id"
```

## Pagination

```bash
# First page of results
notion user list --page-size 10

# Next page using cursor
notion user list --page-size 10 --start-cursor "cursor-from-previous-response"
```

### Full Pagination Script

```bash
#!/bin/bash
cursor=""
page=1

while true; do
  echo "=== Page $page ==="

  if [ -z "$cursor" ]; then
    result=$(notion --raw user list --page-size 10)
  else
    result=$(notion --raw user list --page-size 10 --start-cursor "$cursor")
  fi

  echo "$result" | jq '.results | length'

  has_more=$(echo "$result" | jq '.has_more')
  if [ "$has_more" != "true" ]; then
    break
  fi

  cursor=$(echo "$result" | jq -r '.next_cursor')
  page=$((page + 1))
done
```

## Database Operations

```bash
# Get database schema
notion db get database-id

# Query a data source with filters
notion ds query ds-id \
  --filter '{"property":"Status","select":{"equals":"Done"}}' \
  --sorts '[{"property":"Created","direction":"descending"}]' \
  --page-size 50
```

## Content Editing

```bash
# List blocks in a page
notion block children page-id

# Append a heading block
notion block append page-id --children '[
  {
    "object": "block",
    "type": "heading_1",
    "heading_1": {
      "rich_text": [{"type": "text", "text": {"content": "New Section"}}]
    }
  }
]'

# Append a paragraph after a specific block
notion block append page-id \
  --children '[{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"type":"text","text":{"content":"Inserted text"}}]}}]' \
  --after block-id

# Update a block
notion block update block-id \
  --data '{"paragraph":{"rich_text":[{"type":"text","text":{"content":"Updated text"}}]}}'

# Archive a block
notion block update block-id --data '{}' --archived true

# Delete a block
notion block delete block-id
```

## Comments

```bash
# List comments on a page
notion comment list --block-id page-id

# Add a comment
notion comment create --page-id page-id --text "Looks good!"
```

## Batch Operations

```bash
# Archive all pages matching a search
for page in $(notion --raw search "Old Project" --filter page | jq -r '.results[].id'); do
  notion page update "$page" --properties '{}' --archived true
  echo "Archived $page"
done
```
