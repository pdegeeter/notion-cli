# Notion CLI

A command-line interface for the [Notion API](https://developers.notion.com/), built in Rust.

## Installation

```bash
cargo install --path .
```

The binary is installed as `notion`.

## Setup

```bash
notion init
```

This will prompt for your Notion API token and test the connection. The token is stored in `~/.config/notion-cli/config.toml`.

Alternatively, set the `NOTION_API_TOKEN` environment variable (takes priority over the config file).

## Usage

```bash
notion <command> [options]
```

### Commands

| Command                                              | Description                                        |
| ---------------------------------------------------- | -------------------------------------------------- |
| `notion init`                                        | Setup API token and test connection                |
| `notion search <query>`                              | Search pages and databases by title                |
| `notion user me`                                     | Get the current bot user                           |
| `notion user get <id>`                               | Get a user by ID                                   |
| `notion user list`                                   | List all users                                     |
| `notion page get <id>`                               | Retrieve a page                                    |
| `notion page create`                                 | Create a new page                                  |
| `notion page update <id>`                            | Update page properties                             |
| `notion page move <id>`                              | Move a page to a different parent                  |
| `notion page property <page_id> <property_id>`       | Get a page property value                          |
| `notion block get <id>`                              | Retrieve a block                                   |
| `notion block children <id>`                         | List block children                                |
| `notion block append <id>`                           | Append children to a block                         |
| `notion block update <id>`                           | Update a block                                     |
| `notion block delete <id>`                           | Delete a block                                     |
| `notion comment list --block-id <id>`                | List comments                                      |
| `notion comment create --page-id <id> --text <text>` | Create a comment                                   |
| `notion db get <id>`                                 | Retrieve database metadata                         |
| `notion ds get <id>`                                 | Retrieve a data source                             |
| `notion ds create`                                   | Create a data source                               |
| `notion ds update <id>`                              | Update a data source                               |
| `notion ds query <id>`                               | Query a data source                                |
| `notion ds templates <id>`                           | List data source templates                         |
| `notion completions <shell>`                         | Generate shell completions (bash, zsh, fish, etc.) |
| `notion manpage`                                     | Generate man page                                  |

### Global Options

| Option                       | Description                                                   |
| ---------------------------- | ------------------------------------------------------------- |
| `--output pretty\|json\|raw` | Output format (default: `pretty`)                             |
| `--raw`                      | Shorthand for `--output raw`                                  |
| `--dry-run`                  | Show the request without executing it (write operations only) |
| `--page-size <n>`            | Number of items per page (max 100)                            |
| `--start-cursor <cursor>`    | Pagination cursor                                             |

### Examples

```bash
# Search for pages
notion search "Meeting notes"
notion search "Project" --filter page

# Get a page with specific properties only
notion page get abc123 --filter-properties title,status

# Create a page under a parent page
notion page create --parent <page-id> --properties '{"Name":{"title":[{"text":{"content":"New Page"}}]}}'

# Preview a delete without executing it
notion --dry-run block delete <block-id>

# Get raw JSON output for scripting
notion --raw user me | jq '.name'
```

### Shell Completions

```bash
# Zsh (add to ~/.zshrc)
eval "$(notion completions zsh)"

# Bash (add to ~/.bashrc)
eval "$(notion completions bash)"

# Fish
notion completions fish | source
```

## Development

```bash
cargo build                       # Build
cargo test -- --test-threads=1    # Run tests (sequential required)
cargo run -- search "test"        # Run locally
```

## License

MIT
