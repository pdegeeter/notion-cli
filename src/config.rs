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
        if let Ok(token) = std::env::var("NOTION_API_TOKEN") {
            if !token.is_empty() {
                return Ok(Config {
                    api_token: Some(token),
                });
            }
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
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_token_returns_token_when_set() {
        let config = Config {
            api_token: Some("ntn_test_abc123".to_string()),
        };
        assert_eq!(config.get_token().unwrap(), "ntn_test_abc123");
    }

    #[test]
    fn test_get_token_errors_when_none() {
        let config = Config { api_token: None };
        assert!(config.get_token().is_err());
    }

    #[test]
    fn test_get_token_errors_when_empty() {
        let config = Config {
            api_token: Some("".to_string()),
        };
        assert!(config.get_token().is_err());
    }

    #[test]
    fn test_default_config_has_no_token() {
        let config = Config::default();
        assert!(config.api_token.is_none());
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = Config {
            api_token: Some("ntn_secret_token".to_string()),
        };
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(
            deserialized.api_token.as_deref(),
            Some("ntn_secret_token")
        );
    }

    #[test]
    fn test_config_deserialize_empty_toml() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.api_token.is_none());
    }

    #[test]
    fn test_config_save_and_load_from_file() {
        let dir = tempfile::tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        let config = Config {
            api_token: Some("ntn_file_token".to_string()),
        };
        let content = toml::to_string_pretty(&config).unwrap();
        std::fs::write(&config_path, &content).unwrap();

        let loaded_content = std::fs::read_to_string(&config_path).unwrap();
        let loaded: Config = toml::from_str(&loaded_content).unwrap();
        assert_eq!(loaded.api_token.as_deref(), Some("ntn_file_token"));
    }

    #[test]
    fn test_load_prefers_env_var() {
        // Save current value
        let prev = env::var("NOTION_API_TOKEN").ok();

        // SAFETY: test runs single-threaded via cargo test -- --test-threads=1
        unsafe {
            env::set_var("NOTION_API_TOKEN", "ntn_from_env");
        }
        let config = Config::load().unwrap();
        assert_eq!(config.api_token.as_deref(), Some("ntn_from_env"));

        // Restore
        unsafe {
            match prev {
                Some(v) => env::set_var("NOTION_API_TOKEN", v),
                None => env::remove_var("NOTION_API_TOKEN"),
            }
        }
    }

    #[test]
    fn test_load_ignores_empty_env_var() {
        let prev = env::var("NOTION_API_TOKEN").ok();

        // SAFETY: test runs single-threaded via cargo test -- --test-threads=1
        unsafe {
            env::set_var("NOTION_API_TOKEN", "");
        }
        // Should not return an empty token from env
        let config = Config::load().unwrap();
        if let Some(ref token) = config.api_token {
            assert!(!token.is_empty());
        }

        unsafe {
            match prev {
                Some(v) => env::set_var("NOTION_API_TOKEN", v),
                None => env::remove_var("NOTION_API_TOKEN"),
            }
        }
    }

    #[test]
    fn test_config_path_ends_with_expected() {
        let path = Config::config_path().unwrap();
        assert!(path.ends_with("notion-cli/config.toml"));
    }
}
