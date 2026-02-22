use anyhow::Result;

use crate::client::NotionClient;
use crate::output::{OutputFormat, print_result};

pub async fn me(client: &NotionClient, format: &OutputFormat) -> Result<()> {
    let result = client.get("/v1/users/me", &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn get(client: &NotionClient, user_id: &str, format: &OutputFormat) -> Result<()> {
    let path = format!("/v1/users/{}", user_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn list(
    client: &NotionClient,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut query: Vec<(&str, &str)> = Vec::new();
    let ps_str;
    if let Some(ps) = page_size {
        ps_str = ps.to_string();
        query.push(("page_size", &ps_str));
    }
    if let Some(cursor) = start_cursor {
        query.push(("start_cursor", cursor));
    }

    let result = client.get("/v1/users", &query).await?;
    print_result(&result, format)?;
    Ok(())
}

#[cfg(test)]
#[path = "user_tests.rs"]
mod tests;
