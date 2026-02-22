# Contributing

## Getting Started

1. Clone the repository
2. Run `cargo build` to compile
3. Run `cargo test -- --test-threads=1` to verify everything works

## Adding a New Command

1. **Define the CLI variant** in `src/main.rs`: add an entry to the relevant `*Commands` enum (or create a new subcommand enum for a new resource type).
2. **Create the handler** in `src/commands/`. Follow the existing pattern:
   ```rust
   pub async fn action(client: &NotionClient, params..., format: &OutputFormat) -> Result<()>
   ```
3. **Wire it up** in the `match` block in `main()`.
4. **Add CLI parsing tests** in the `#[cfg(test)]` section of `main.rs`.
5. **Add HTTP tests** using mockito in your command module or in `client.rs`.

## Testing

Tests must run sequentially because some tests mutate environment variables:

```bash
cargo test -- --test-threads=1          # All tests
cargo test test_name                    # Single test
cargo test module::tests                # Module tests
```

All HTTP interactions are tested with [mockito](https://docs.rs/mockito) to avoid real API calls.

### Coverage

Line coverage must stay at or above **90%**. Run coverage locally with:

```bash
cargo llvm-cov --fail-under-lines 90 --text -- --test-threads=1
```

The CI will fail if coverage drops below this threshold.

### Lint & Format

Code must pass clippy with no warnings and be properly formatted:

```bash
cargo clippy -- -D warnings         # Lint (warnings are errors)
cargo fmt --check                   # Check formatting
cargo fmt                           # Auto-fix formatting
```

## Code Conventions

- Use `anyhow::Result` for error handling, with `.context()` for meaningful error messages.
- All API-calling functions are `async`.
- JSON payloads are built with `serde_json::json!()`.
- User-facing JSON strings passed as CLI arguments are parsed with `serde_json::from_str()` and validated with `.context()`.
- Write operations (POST, PATCH, DELETE) must respect the `--dry-run` flag (handled automatically by `NotionClient`).

## Commit Messages

Use a short summary line describing what was done. Include scope when relevant (e.g., "Add block duplicate command").
