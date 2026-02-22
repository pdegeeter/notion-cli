# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                        # Debug build
cargo test -- --test-threads=1     # Run all tests (sequential for env var tests)
cargo test config::tests           # Run tests for a specific module
cargo test test_page_get           # Run a single test by name
cargo install --path .             # Install binary globally as `notion`
```

Tests **must** run with `--test-threads=1` because config tests mutate environment variables.

## Architecture

Rust async CLI (`tokio` + `clap` derive) that calls the Notion REST API directly via `reqwest`.

**Flow:** CLI parsing (`main.rs`) &rarr; `NotionClient` (`client.rs`) &rarr; Notion API &rarr; `output::print_result()`

### Core modules

- **`main.rs`** - Clap `#[derive(Parser)]` with nested subcommands enum. Routes parsed commands to `commands::*` functions. Non-API commands (`init`, `completions`, `manpage`) are handled before client construction.
- **`client.rs`** - `NotionClient` wraps reqwest with Bearer auth, `Notion-Version` header, retry on 429 with exponential backoff, and dry-run mode for write operations.
- **`config.rs`** - Token resolution: env var `NOTION_API_TOKEN` takes priority over `~/.config/notion-cli/config.toml`.
- **`output.rs`** - Three formats (`Pretty`/`Json`/`Raw`) + colored helpers (`print_success`, `print_error`, `print_info`).

### Command module pattern

Every command function follows the same signature:

```rust
pub async fn action(client: &NotionClient, params..., format: &OutputFormat) -> Result<()> {
    // Build path/query/body
    // Call client.get/post/patch/delete
    // print_result(&response, format)
}
```

To add a new command: add variant to the relevant enum in `main.rs`, add the handler function in `commands/`, wire it in the `match` block.

### Key constants

- API base: `https://api.notion.com`
- API version header: `2025-09-03`
- Max retries on 429: 3, initial backoff: 500ms

## Testing patterns

- Unit tests live in `#[cfg(test)] mod tests` within each source file
- HTTP tests use `mockito` async server (`Server::new_async().await`)
- Config tests use `tempfile` for filesystem isolation
- CLI parsing tests use `Cli::parse_from()` / `Cli::try_parse_from()`
