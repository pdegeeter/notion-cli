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

### Lint & Format

```bash
cargo clippy -- -D warnings         # Lint (warnings are errors)
cargo fmt --check                   # Check formatting
cargo fmt                           # Auto-fix formatting
```

All clippy warnings **must** be resolved. The CI will fail on any warning.

### Coverage

```bash
cargo llvm-cov --fail-under-lines 90 --text -- --test-threads=1    # Text report
cargo llvm-cov --fail-under-lines 90 --html -- --test-threads=1    # HTML report (target/llvm-cov/html/)
cargo llvm-cov --fail-under-lines 90 --lcov --output-path lcov.info -- --test-threads=1  # LCOV format
```

Line coverage **must** stay at or above **90%**. The CI will fail if coverage drops below this threshold.

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

## Commit conventions

Use [Conventional Commits](https://www.conventionalcommits.org/). Format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `ci`, `chore`, `perf`, `build`

Examples:

- `feat(page): add duplicate command`
- `fix(client): handle 502 responses`
- `docs(readme): update installation instructions`
- `refactor(output): extract table formatter`

## Maintaining documentation

When adding or modifying a CLI command, update the following files:

1. **`README.md`** - Update the commands table and examples if applicable.
2. **`CONTRIBUTING.md`** - Update if the contribution workflow or conventions change.
3. **`skills/notion/`** - Update the Claude Code skill:
   - Add/update the relevant reference file in `skills/notion/references/` (one file per command group).
   - Update `skills/notion/SKILL.md` if a new command group is added (add it to the command groups table).
