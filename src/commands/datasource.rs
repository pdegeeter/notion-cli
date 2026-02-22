use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::client::NotionClient;
use crate::output::{OutputFormat, print_result};

pub async fn get(client: &NotionClient, ds_id: &str, format: &OutputFormat) -> Result<()> {
    let path = format!("/v1/data_sources/{}", ds_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn create(
    client: &NotionClient,
    parent_id: &str,
    title: &str,
    properties_json: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut body = json!({
        "parent": { "page_id": parent_id },
        "title": [{ "type": "text", "text": { "content": title } }],
    });

    if let Some(pj) = properties_json {
        let props: Value = serde_json::from_str(pj).context("Invalid JSON for properties")?;
        body["properties"] = props;
    }

    let result = client.post("/v1/data_sources", Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn update(
    client: &NotionClient,
    ds_id: &str,
    data_json: &str,
    format: &OutputFormat,
) -> Result<()> {
    let body: Value = serde_json::from_str(data_json).context("Invalid JSON for data source")?;
    let path = format!("/v1/data_sources/{}", ds_id);
    let result = client.patch(&path, &body).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn query(
    client: &NotionClient,
    ds_id: &str,
    filter_json: Option<&str>,
    sorts_json: Option<&str>,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut body = json!({});

    if let Some(fj) = filter_json {
        let filter: Value = serde_json::from_str(fj).context("Invalid JSON for filter")?;
        body["filter"] = filter;
    }

    if let Some(sj) = sorts_json {
        let sorts: Value = serde_json::from_str(sj).context("Invalid JSON for sorts")?;
        body["sorts"] = sorts;
    }

    if let Some(ps) = page_size {
        body["page_size"] = json!(ps);
    }

    if let Some(cursor) = start_cursor {
        body["start_cursor"] = json!(cursor);
    }

    let path = format!("/v1/data_sources/{}/query", ds_id);
    let result = client.post(&path, Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn templates(client: &NotionClient, ds_id: &str, format: &OutputFormat) -> Result<()> {
    let path = format!("/v1/data_sources/{}/templates", ds_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

#[cfg(test)]
#[path = "datasource_tests.rs"]
mod tests;
