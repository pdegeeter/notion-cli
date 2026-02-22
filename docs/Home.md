# Notion CLI

A fast, feature-rich CLI for the [Notion API](https://developers.notion.com/) — manage pages, databases, blocks, users, comments and more from your terminal.

## Features

- **Full API coverage** — pages, blocks, databases, data sources, comments, users, search
- **Three output formats** — pretty, JSON, and raw (pipe-friendly)
- **Dry-run mode** — preview write operations before executing them
- **Automatic retry** — exponential backoff on 429 rate limits
- **Shell completions** — Bash, Zsh, Fish, PowerShell, Elvish
- **Man page generation** — built-in `manpage` command

## Quick Start

```bash
# Install from source
cargo install --path .

# Set up your Notion token
notion init

# Search for pages
notion search "Meeting Notes"

# Get a page
notion page get <page-id>

# List users
notion user list
```

## Documentation

- [Installation](Installation.md)
- [Configuration](Configuration.md)
- [Commands Reference](Commands.md)
- [Usage Examples](Examples.md)
- [Architecture](Architecture.md)

## License

MIT
