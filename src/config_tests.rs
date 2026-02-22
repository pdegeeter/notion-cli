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
