# Installation

## From Source

Requires [Rust](https://rustup.rs/) (edition 2021 or later).

```bash
git clone https://github.com/pdegeeter/notion-cli.git
cd notion-cli
cargo install --path .
```

The binary is installed as `notion` and available globally.

## Verify Installation

```bash
notion --version
```

## Shell Completions

Generate completions for your shell and add them to your config:

### Zsh

```bash
# Add to ~/.zshrc
eval "$(notion completions zsh)"
```

### Bash

```bash
# Add to ~/.bashrc
eval "$(notion completions bash)"
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
notion completions fish | source
```

### PowerShell

```powershell
notion completions powershell | Out-String | Invoke-Expression
```

## Man Page

Generate and install the man page:

```bash
notion manpage | gzip > /usr/local/share/man/man1/notion.1.gz
man notion
```
