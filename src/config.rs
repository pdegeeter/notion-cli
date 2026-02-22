use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_token: Option<String>,
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("notion-cli");
        Ok(dir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        // First check env var
        if let Ok(token) = std::env::var("NOTION_API_TOKEN")
            && !token.is_empty()
        {
            return Ok(Config {
                api_token: Some(token),
            });
        }

        // Then check config file
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read config file: {}", path.display()))?;
            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
            return Ok(config);
        }

        Ok(Config::default())
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config directory: {}", dir.display()))?;

        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    pub fn get_token(&self) -> Result<&str> {
        self.api_token
            .as_deref()
            .filter(|t| !t.is_empty())
            .context(
                "No API token configured. Run `notion init` or set NOTION_API_TOKEN environment variable.",
            )
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
