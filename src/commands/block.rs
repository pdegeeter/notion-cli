use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::client::NotionClient;
use crate::output::{OutputFormat, print_result};

pub async fn get(client: &NotionClient, block_id: &str, format: &OutputFormat) -> Result<()> {
    let path = format!("/v1/blocks/{}", block_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn children(
    client: &NotionClient,
    block_id: &str,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/blocks/{}/children", block_id);
    let mut query: Vec<(&str, &str)> = Vec::new();
    let ps_str;
    if let Some(ps) = page_size {
        ps_str = ps.to_string();
        query.push(("page_size", &ps_str));
    }
    if let Some(cursor) = start_cursor {
        query.push(("start_cursor", cursor));
    }

    let result = client.get(&path, &query).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn append(
    client: &NotionClient,
    block_id: &str,
    children_json: &str,
    after: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let children: Value =
        serde_json::from_str(children_json).context("Invalid JSON for children")?;

    let mut body = json!({ "children": children });
    if let Some(after_id) = after {
        body["after"] = json!(after_id);
    }

    let path = format!("/v1/blocks/{}/children", block_id);
    let result = client.patch(&path, &body).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn update(
    client: &NotionClient,
    block_id: &str,
    data_json: &str,
    archived: Option<bool>,
    format: &OutputFormat,
) -> Result<()> {
    let mut body: Value = serde_json::from_str(data_json).context("Invalid JSON for block data")?;

    if let Some(a) = archived {
        body["archived"] = json!(a);
    }

    let path = format!("/v1/blocks/{}", block_id);
    let result = client.patch(&path, &body).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn delete(client: &NotionClient, block_id: &str, format: &OutputFormat) -> Result<()> {
    let path = format!("/v1/blocks/{}", block_id);
    let result = client.delete(&path).await?;
    print_result(&result, format)?;
    Ok(())
}

#[cfg(test)]
#[path = "block_tests.rs"]
mod tests;
