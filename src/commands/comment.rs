use anyhow::Result;
use serde_json::json;

use crate::client::NotionClient;
use crate::output::{print_result, OutputFormat};

pub async fn list(
    client: &NotionClient,
    block_id: &str,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut query: Vec<(&str, &str)> = vec![("block_id", block_id)];
    let ps_str;
    if let Some(ps) = page_size {
        ps_str = ps.to_string();
        query.push(("page_size", &ps_str));
    }
    if let Some(cursor) = start_cursor {
        query.push(("start_cursor", cursor));
    }

    let result = client.get("/v1/comments", &query).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn create(
    client: &NotionClient,
    page_id: &str,
    text: &str,
    format: &OutputFormat,
) -> Result<()> {
    let body = json!({
        "parent": { "page_id": page_id },
        "rich_text": [{
            "type": "text",
            "text": { "content": text }
        }]
    });

    let result = client.post("/v1/comments", Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

#[cfg(test)]
#[path = "comment_tests.rs"]
mod tests;
