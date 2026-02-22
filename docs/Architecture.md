# Architecture

## Overview

Notion CLI is an async Rust application built with [tokio](https://tokio.rs/) and [clap](https://docs.rs/clap/latest/clap/) that calls the Notion REST API via [reqwest](https://docs.rs/reqwest/latest/reqwest/).

```
CLI parsing (main.rs)
        │
        ├── init / completions / manpage   → handled directly (no API)
        │
        └── all other commands
                │
                ├── Load config → resolve token
                ├── Create NotionClient
                ├── Route to command handler (commands/*)
                ├── Build API request (path, query, body)
                ├── NotionClient.{get|post|patch|delete}()
                │       └── retry on 429 with exponential backoff
                └── output::print_result() in chosen format
```

## Core Modules

### `main.rs`

Clap `#[derive(Parser)]` with nested subcommand enums. Parses CLI arguments and routes to the appropriate command handler. Non-API commands (`init`, `completions`, `manpage`) are handled before client construction.

### `client.rs` — `NotionClient`

Wraps `reqwest::Client` with:

- **Bearer auth** and `Notion-Version` header on every request
- **Retry on 429** with exponential backoff (max 3 retries, starting at 500ms)
- **Dry-run mode** for write operations — prints the request without sending it

```
NotionClient
├── get(path, query)        → GET
├── post(path, body)        → POST
├── patch(path, body)       → PATCH
├── delete(path)            → DELETE
└── send_with_retry(...)    → retry logic for all methods
```

### `config.rs`

Token resolution with priority:

1. `NOTION_API_TOKEN` environment variable
2. `~/.config/notion-cli/config.toml`

### `output.rs`

Three output formats via the `OutputFormat` enum:

| Format | Description |
|--------|-------------|
| `Pretty` | Indented JSON (default) |
| `Json` | Indented JSON |
| `Raw` | Compact single-line JSON |

Also provides colored helpers: `print_success`, `print_error`, `print_info`.

## Command Module Pattern

Every command handler follows the same signature:

```rust
pub async fn action(
    client: &NotionClient,
    params...,
    format: &OutputFormat,
) -> Result<()> {
    // Build path/query/body
    // Call client.get/post/patch/delete
    // print_result(&response, format)
}
```

## API Constants

| Constant | Value |
|----------|-------|
| API base URL | `https://api.notion.com` |
| API version header | `2025-09-03` |
| Max retries on 429 | 3 |
| Initial backoff | 500ms |

## Project Structure

```
src/
├── main.rs              # CLI parsing, command routing
├── client.rs            # HTTP client, retry logic
├── config.rs            # Token and config management
├── output.rs            # Output formatting
└── commands/
    ├── mod.rs           # Module declarations
    ├── init.rs          # Interactive setup
    ├── search.rs        # Search pages/databases
    ├── user.rs          # User operations
    ├── page.rs          # Page CRUD + move + property
    ├── block.rs         # Block CRUD + children + append
    ├── comment.rs       # Comment list/create
    ├── database.rs      # Database metadata
    └── datasource.rs    # Data source operations
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing (derive) |
| `clap_complete` | Shell completion generation |
| `clap_mangen` | Man page generation |
| `reqwest` | HTTP client (rustls-tls) |
| `tokio` | Async runtime |
| `serde` / `serde_json` | JSON serialization |
| `toml` | Config file parsing |
| `dirs` | XDG config directory resolution |
| `colored` | Terminal colors |
| `dialoguer` | Interactive prompts |
| `anyhow` | Error handling |

### Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `mockito` | HTTP mock server for tests |
| `tempfile` | Temporary files for config tests |

## Testing

Tests run sequentially because config tests mutate environment variables:

```bash
cargo test -- --test-threads=1
```

- **Unit tests** live in `#[cfg(test)] mod tests` within each source file
- **HTTP tests** use `mockito` async server
- **Config tests** use `tempfile` for filesystem isolation
- **CLI parsing tests** use `Cli::parse_from()` / `Cli::try_parse_from()`
