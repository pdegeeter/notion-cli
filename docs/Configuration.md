# Configuration

## Setting Up a Notion Integration

1. Go to <https://www.notion.so/my-integrations>
2. Click **Create new integration**
3. Name it (e.g. `notion-cli`)
4. Select your workspace
5. Grant capabilities: Read content, Update content, Insert content, Delete content
6. Copy the secret token (`ntn_...`)

## Connecting Pages to Your Integration

For each page or database you want to access:

1. Open it in Notion
2. Click the `⋯` menu
3. Select **Add connections**
4. Choose your integration

## Authentication

### Interactive Setup (Recommended)

```bash
notion init
```

This will:
- Prompt for your API token
- Test the connection to Notion
- Save the token to `~/.config/notion-cli/config.toml`

### Environment Variable

Set `NOTION_API_TOKEN` to override the config file:

```bash
export NOTION_API_TOKEN="ntn_your_token_here"
notion user me
```

### Config File

The config file is located at `~/.config/notion-cli/config.toml`:

```toml
api_token = "ntn_your_token_here"
```

## Token Priority

1. `NOTION_API_TOKEN` environment variable (highest priority)
2. `~/.config/notion-cli/config.toml`
3. Interactive prompt via `notion init`
