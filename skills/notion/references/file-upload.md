# File Upload Commands

## `notion file-upload create --mode <mode>`

Create a file upload session.

**Endpoint:** `POST /v1/file_uploads`

| Option                  | Required | Description                                           |
| ----------------------- | -------- | ----------------------------------------------------- |
| `--mode <mode>`         | yes      | Upload mode: single_part, multi_part, or external_url |
| `--filename <string>`   | no       | Filename for the upload                               |
| `--content-type <mime>` | no       | MIME content type                                     |
| `--number-of-parts <n>` | no       | Number of parts (multi_part mode)                     |
| `--external-url <url>`  | no       | External URL (external_url mode)                      |

Supports `--dry-run`.

```bash
# Create a single-part upload session
notion file-upload create --mode single_part --filename "report.pdf"

# Create a multi-part upload session
notion file-upload create --mode multi_part --filename "video.mp4" --number-of-parts 3

# Create an external URL upload
notion file-upload create --mode external_url --external-url "https://example.com/image.png"

# Preview
notion --dry-run file-upload create --mode single_part --filename "test.txt"
```

## `notion file-upload send <id> --file <path>`

Send a file to an upload session via multipart/form-data.

**Endpoint:** `POST /v1/file_uploads/{id}/send`

| Argument / Option   | Required | Description                   |
| ------------------- | -------- | ----------------------------- |
| `<id>`              | yes      | File upload ID                |
| `--file <path>`     | yes      | Path to the file to upload    |
| `--part-number <n>` | no       | Part number (multi_part mode) |

Supports `--dry-run`.

```bash
# Send a file
notion file-upload send fu-abc123 --file ./report.pdf

# Send a specific part
notion file-upload send fu-abc123 --file ./part2.bin --part-number 2
```

## `notion file-upload complete <id>`

Complete a file upload session.

**Endpoint:** `POST /v1/file_uploads/{id}/complete`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | File upload ID |

Supports `--dry-run`.

```bash
notion file-upload complete fu-abc123
```

## `notion file-upload get <id>`

Retrieve a file upload.

**Endpoint:** `GET /v1/file_uploads/{id}`

| Argument | Required | Description    |
| -------- | -------- | -------------- |
| `<id>`   | yes      | File upload ID |

```bash
notion file-upload get fu-abc123
notion --raw file-upload get fu-abc123 | jq '.status'
```

## `notion file-upload list`

List file uploads.

**Endpoint:** `GET /v1/file_uploads`

| Option              | Required | Description      |
| ------------------- | -------- | ---------------- |
| `--status <status>` | no       | Filter by status |

Supports `--page-size` and `--start-cursor`.

```bash
# List all file uploads
notion file-upload list

# List completed uploads
notion file-upload list --status upload_completed

# With pagination
notion file-upload list --page-size 10

# Raw output for scripting
notion --raw file-upload list | jq '.results[].id'
```

## `notion file-upload upload <path>`

Upload a file in one step (create + send + complete).

**Endpoint:** Uses `POST /v1/file_uploads`, `POST /v1/file_uploads/{id}/send`, `POST /v1/file_uploads/{id}/complete`

| Argument / Option       | Required | Description       |
| ----------------------- | -------- | ----------------- |
| `<path>`                | yes      | Path to the file  |
| `--content-type <mime>` | no       | MIME content type |

Supports `--dry-run`.

```bash
# Upload a file
notion file-upload upload ./report.pdf

# Upload with explicit content type
notion file-upload upload ./data.csv --content-type text/csv

# Preview
notion --dry-run file-upload upload ./image.png
```
