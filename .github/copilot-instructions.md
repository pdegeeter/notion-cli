# Copilot Instructions

## Project Overview

Rust async CLI tool (`notion-cli`) that interacts with the Notion REST API. Built with `tokio`, `clap` (derive), and `reqwest`.

**Binary name:** `notion`

## Architecture

**Flow:** CLI parsing (`main.rs`) → `NotionClient` (`client.rs`) → Notion API → `output::print_result()`

### Core modules

- **`main.rs`** – Clap `#[derive(Parser)]` with nested subcommand enums. Routes parsed commands to `commands::*` handler functions. Non-API commands (`init`, `completions`, `manpage`) are handled before client construction.
- **`client.rs`** – `NotionClient` wraps `reqwest` with Bearer auth, `Notion-Version` header, retry on 429 with exponential backoff, and dry-run mode for write operations.
- **`config.rs`** – Token resolution: env var `NOTION_API_TOKEN` takes priority over `~/.config/notion-cli/config.toml`.
- **`output.rs`** – Three output formats (`Pretty`/`Json`/`Raw`) + colored helpers (`print_success`, `print_error`, `print_info`).
- **`commands/`** – One file per Notion resource (`page.rs`, `database.rs`, `block.rs`, `comment.rs`, `user.rs`, `search.rs`, `datasource.rs`, `init.rs`).

### Key constants

- API base: `https://api.notion.com`
- API version header: `2025-09-03`
- Max retries on 429: 3, initial backoff: 500ms

## Command module pattern

Every command handler follows this signature:

```rust
pub async fn action(client: &NotionClient, params..., format: &OutputFormat) -> Result<()> {
    // Build path/query/body
    // Call client.get/post/patch/delete
    // print_result(&response, format)
}
```

To add a new command: add a variant to the relevant enum in `main.rs`, add the handler function in `commands/`, and wire it in the `match` block.

## Code Style

- **No single-line `if` statements.** Always use braces with the body on a separate line:

  ```rust
  // bad
  if test { return true; }

  // good
  if test {
      return true;
  }
  ```

- Prefer idiomatic Rust: use `?` for error propagation, avoid unnecessary `.clone()` and `.unwrap()` in non-test code.
- Use `anyhow::Result` for error handling in command functions.
- Keep functions focused: one responsibility per function.

## Testing

- Tests **must** run with `--test-threads=1` because config tests mutate environment variables.
- Unit tests live in `#[cfg(test)] mod tests` within each source file.
- HTTP tests use `mockito` async server (`Server::new_async().await`).
- Config tests use `tempfile` for filesystem isolation.
- CLI parsing tests use `Cli::parse_from()` / `Cli::try_parse_from()`.

## Dependencies

| Crate                  | Purpose                 |
| ---------------------- | ----------------------- |
| `clap` (derive)        | CLI argument parsing    |
| `reqwest` (rustls-tls) | HTTP client             |
| `tokio`                | Async runtime           |
| `serde` / `serde_json` | JSON serialization      |
| `toml`                 | Config file parsing     |
| `colored`              | Terminal output styling |
| `dialoguer`            | Interactive prompts     |
| `anyhow`               | Error handling          |
| `mockito` (dev)        | HTTP mocking            |
| `tempfile` (dev)       | Temp file/dir for tests |

## Review Checklist

When reviewing pull requests, verify:

1. **Error handling** – No `.unwrap()` outside of tests. Use `?` or explicit error messages with `anyhow::bail!`/`anyhow::Context`.
2. **API calls** – All Notion API calls go through `NotionClient` methods, never raw `reqwest` calls.
3. **Output** – All user-facing output uses `output::print_result()` or the colored helpers, never raw `println!`.
4. **Tests** – New commands and behaviors have corresponding tests. HTTP tests use `mockito`.
5. **Code style** – No single-line `if` statements. Braces required on all control flow.
6. **Dry-run** – Write operations (POST/PATCH/DELETE) must respect the client's dry-run mode.
7. **No secrets** – API tokens must never be hardcoded. Token resolution goes through `config.rs`.
