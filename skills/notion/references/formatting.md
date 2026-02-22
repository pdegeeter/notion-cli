# Formatting Notion Pages

This reference covers how to format page content using the `notion` CLI: block types for `--children` arguments and rich text for text content.

## Table of Contents

- [Rich Text](#rich-text)
- [Text Blocks](#text-blocks)
- [List Blocks](#list-blocks)
- [Media Blocks](#media-blocks)
- [Structural Blocks](#structural-blocks)
- [Colors](#colors)
- [Limits](#limits)

---

## Rich Text

Rich text is a JSON array used in all text-containing blocks. Each element can have its own formatting.

### Plain text

```json
[{"type": "text", "text": {"content": "Hello world"}}]
```

### Bold, italic, strikethrough, underline, code

```json
[
  {"type": "text", "text": {"content": "bold"}, "annotations": {"bold": true}},
  {"type": "text", "text": {"content": " and "}},
  {"type": "text", "text": {"content": "italic"}, "annotations": {"italic": true}},
  {"type": "text", "text": {"content": " and "}},
  {"type": "text", "text": {"content": "code"}, "annotations": {"code": true}}
]
```

Available annotations: `bold`, `italic`, `strikethrough`, `underline`, `code` (all booleans), `color` (string).

### Link

```json
[{"type": "text", "text": {"content": "click here", "link": {"url": "https://example.com"}}}]
```

### Colored text

```json
[{"type": "text", "text": {"content": "warning"}, "annotations": {"color": "red", "bold": true}}]
```

### Highlighted text

```json
[{"type": "text", "text": {"content": "important"}, "annotations": {"color": "yellow_background"}}]
```

### Mention a user

```json
[{"type": "mention", "mention": {"type": "user", "user": {"id": "user-uuid"}}}]
```

### Mention a page

```json
[{"type": "mention", "mention": {"type": "page", "page": {"id": "page-uuid"}}}]
```

### Mention a date

```json
[{"type": "mention", "mention": {"type": "date", "date": {"start": "2025-01-15"}}}]
```

### Inline equation (LaTeX)

```json
[{"type": "equation", "equation": {"expression": "E = mc^2"}}]
```

---

## Text Blocks

### Paragraph

```bash
notion block append <page-id> --children '[
  {"object":"block","type":"paragraph","paragraph":{
    "rich_text":[{"type":"text","text":{"content":"A simple paragraph."}}]
  }}
]'
```

### Headings (heading_1, heading_2, heading_3)

```bash
notion block append <page-id> --children '[
  {"object":"block","type":"heading_1","heading_1":{
    "rich_text":[{"type":"text","text":{"content":"Main Title"}}]
  }},
  {"object":"block","type":"heading_2","heading_2":{
    "rich_text":[{"type":"text","text":{"content":"Subtitle"}}]
  }},
  {"object":"block","type":"heading_3","heading_3":{
    "rich_text":[{"type":"text","text":{"content":"Section"}}]
  }}
]'
```

Toggle heading (collapsible — supports children):

```json
{"object":"block","type":"heading_2","heading_2":{
  "rich_text":[{"type":"text","text":{"content":"Click to expand"}}],
  "is_toggleable": true
}}
```

### Quote

```json
{"object":"block","type":"quote","quote":{
  "rich_text":[{"type":"text","text":{"content":"To be or not to be."}}]
}}
```

### Callout

```json
{"object":"block","type":"callout","callout":{
  "rich_text":[{"type":"text","text":{"content":"Important note"}}],
  "icon":{"type":"emoji","emoji":"⚠️"},
  "color":"yellow_background"
}}
```

### Code block

```json
{"object":"block","type":"code","code":{
  "rich_text":[{"type":"text","text":{"content":"fn main() {\n    println!(\"Hello\");\n}"}}],
  "language":"rust"
}}
```

Common languages: `bash`, `c`, `cpp`, `css`, `go`, `html`, `java`, `javascript`, `json`, `kotlin`, `markdown`, `python`, `ruby`, `rust`, `sql`, `swift`, `typescript`, `yaml`

### Equation block (LaTeX)

```json
{"object":"block","type":"equation","equation":{"expression":"\\int_0^\\infty e^{-x} dx = 1"}}
```

---

## List Blocks

### Bulleted list

```bash
notion block append <page-id> --children '[
  {"object":"block","type":"bulleted_list_item","bulleted_list_item":{
    "rich_text":[{"type":"text","text":{"content":"First item"}}]
  }},
  {"object":"block","type":"bulleted_list_item","bulleted_list_item":{
    "rich_text":[{"type":"text","text":{"content":"Second item"}}]
  }}
]'
```

### Numbered list

```json
[
  {"object":"block","type":"numbered_list_item","numbered_list_item":{
    "rich_text":[{"type":"text","text":{"content":"Step one"}}]
  }},
  {"object":"block","type":"numbered_list_item","numbered_list_item":{
    "rich_text":[{"type":"text","text":{"content":"Step two"}}]
  }}
]
```

### To-do list

```json
[
  {"object":"block","type":"to_do","to_do":{
    "rich_text":[{"type":"text","text":{"content":"Buy milk"}}],
    "checked":false
  }},
  {"object":"block","type":"to_do","to_do":{
    "rich_text":[{"type":"text","text":{"content":"Write docs"}}],
    "checked":true
  }}
]
```

### Toggle

```json
{"object":"block","type":"toggle","toggle":{
  "rich_text":[{"type":"text","text":{"content":"Click to reveal"}}]
}}
```

Toggle content is added as children (nested blocks).

---

## Media Blocks

### Image (external URL)

```json
{"object":"block","type":"image","image":{
  "type":"external",
  "external":{"url":"https://example.com/photo.png"}
}}
```

### Video (external URL or YouTube)

```json
{"object":"block","type":"video","video":{
  "type":"external",
  "external":{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}
}}
```

### File attachment

```json
{"object":"block","type":"file","file":{
  "type":"external",
  "external":{"url":"https://example.com/report.pdf"},
  "name":"Report.pdf"
}}
```

### Bookmark

```json
{"object":"block","type":"bookmark","bookmark":{
  "url":"https://github.com"
}}
```

### Embed

```json
{"object":"block","type":"embed","embed":{
  "url":"https://codepen.io/pen/123"
}}
```

---

## Structural Blocks

### Divider

```json
{"object":"block","type":"divider","divider":{}}
```

### Table of contents

```json
{"object":"block","type":"table_of_contents","table_of_contents":{"color":"default"}}
```

### Breadcrumb

```json
{"object":"block","type":"breadcrumb","breadcrumb":{}}
```

### Table

```bash
notion block append <page-id> --children '[
  {"object":"block","type":"table","table":{
    "table_width":3,
    "has_column_header":true,
    "has_row_header":false,
    "children":[
      {"object":"block","type":"table_row","table_row":{
        "cells":[
          [{"type":"text","text":{"content":"Name"}}],
          [{"type":"text","text":{"content":"Role"}}],
          [{"type":"text","text":{"content":"Status"}}]
        ]
      }},
      {"object":"block","type":"table_row","table_row":{
        "cells":[
          [{"type":"text","text":{"content":"Alice"}}],
          [{"type":"text","text":{"content":"Engineer"}}],
          [{"type":"text","text":{"content":"Active"}}]
        ]
      }}
    ]
  }}
]'
```

`table_width` must match the number of cells per row. Can only be set at creation.

### Column layout

```json
{"object":"block","type":"column_list","column_list":{
  "children":[
    {"object":"block","type":"column","column":{
      "children":[
        {"object":"block","type":"paragraph","paragraph":{
          "rich_text":[{"type":"text","text":{"content":"Left column"}}]
        }}
      ]
    }},
    {"object":"block","type":"column","column":{
      "children":[
        {"object":"block","type":"paragraph","paragraph":{
          "rich_text":[{"type":"text","text":{"content":"Right column"}}]
        }}
      ]
    }}
  ]
}}
```

Must have at least 2 columns.

---

## Colors

Available for blocks (`color` field) and rich text annotations:

**Text colors:** `default`, `gray`, `brown`, `orange`, `yellow`, `green`, `blue`, `purple`, `pink`, `red`

**Background colors:** `gray_background`, `brown_background`, `orange_background`, `yellow_background`, `green_background`, `blue_background`, `purple_background`, `pink_background`, `red_background`

---

## Limits

| Type                | Limit              |
| ------------------- | ------------------ |
| Rich text content   | 2000 chars/element |
| Rich text array     | 100 elements       |
| Blocks per request  | 100                |
| Nesting depth       | 2 levels           |
| Equation expression | 1000 chars         |
| Payload size        | 500 KB             |
