use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};

use crate::client::NotionClient;
use crate::config::Config;
use crate::output::{print_error, print_info, print_success};

pub async fn run() -> Result<()> {
    print_info("Notion CLI initialization");

    let mut config = Config::load()?;

    if let Some(ref token) = config.api_token
        && !token.is_empty()
    {
        print_info(&format!(
            "Existing token found ({}...)",
            &token[..token.len().min(8)]
        ));

        let keep = Confirm::new()
            .with_prompt("Keep existing token?")
            .default(true)
            .interact()?;

        if !keep {
            config.api_token = None;
        }
    }

    if config.api_token.is_none() {
        print_info("You can create an integration at https://www.notion.so/my-integrations");
        let token: String = Input::new()
            .with_prompt("Enter your Notion API token")
            .interact_text()?;

        config.api_token = Some(token);
    }

    // Test the connection
    print_info("Testing connection...");
    let token = config.get_token()?;
    let client = NotionClient::new(token)?;

    verify_connection(&client).await?;

    // Save config
    config.save()?;
    let config_path = Config::config_path()?;
    print_success(&format!("Config saved to {}", config_path.display()));

    Ok(())
}

pub async fn verify_connection(client: &NotionClient) -> Result<()> {
    match client.get("/v1/users/me", &[]).await {
        Ok(user) => {
            let name = user["name"].as_str().unwrap_or("Unknown");
            let bot_type = user["type"].as_str().unwrap_or("unknown");
            let workspace = user["bot"]["workspace_name"]
                .as_str()
                .unwrap_or("Unknown workspace");

            print_success(&format!("Connected as {} ({})", name.bold(), bot_type));
            print_success(&format!("Workspace: {}", workspace.bold()));

            Ok(())
        }
        Err(e) => {
            print_error(&format!("Connection failed: {}", e));
            print_error("Please check your API token and try again.");
            Err(e)
        }
    }
}

#[cfg(test)]
#[path = "init_tests.rs"]
mod tests;
