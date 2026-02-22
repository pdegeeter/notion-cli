use anyhow::{Context, Result};
use serde_json::{json, Value};

use crate::client::NotionClient;
use crate::output::{print_result, OutputFormat};

pub async fn get(
    client: &NotionClient,
    page_id: &str,
    filter_properties: &[String],
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/pages/{}", page_id);
    let query: Vec<(&str, &str)> = filter_properties
        .iter()
        .map(|p| ("filter_properties", p.as_str()))
        .collect();
    let result = client.get(&path, &query).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn create(
    client: &NotionClient,
    parent_id: &str,
    properties_json: &str,
    children_json: Option<&str>,
    is_database_parent: bool,
    format: &OutputFormat,
) -> Result<()> {
    let properties: Value =
        serde_json::from_str(properties_json).context("Invalid JSON for properties")?;

    let parent = if is_database_parent {
        json!({ "database_id": parent_id })
    } else {
        json!({ "page_id": parent_id })
    };

    let mut body = json!({
        "parent": parent,
        "properties": properties,
    });

    if let Some(cj) = children_json {
        let children: Value = serde_json::from_str(cj).context("Invalid JSON for children")?;
        body["children"] = children;
    }

    let result = client.post("/v1/pages", Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn update(
    client: &NotionClient,
    page_id: &str,
    properties_json: &str,
    archived: Option<bool>,
    format: &OutputFormat,
) -> Result<()> {
    let properties: Value =
        serde_json::from_str(properties_json).context("Invalid JSON for properties")?;

    let mut body = json!({ "properties": properties });

    if let Some(a) = archived {
        body["archived"] = json!(a);
    }

    let path = format!("/v1/pages/{}", page_id);
    let result = client.patch(&path, &body).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn move_page(
    client: &NotionClient,
    page_id: &str,
    parent_type: &str,
    parent_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let parent = match parent_type {
        "page" => json!({ "type": "page_id", "page_id": parent_id }),
        "database" => json!({ "type": "database_id", "database_id": parent_id }),
        "workspace" => json!({ "type": "workspace" }),
        _ => anyhow::bail!("Invalid parent type: {}. Use 'page', 'database', or 'workspace'", parent_type),
    };

    let body = json!({ "parent": parent });
    let path = format!("/v1/pages/{}/move", page_id);
    let result = client.post(&path, Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn property(
    client: &NotionClient,
    page_id: &str,
    property_id: &str,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/pages/{}/properties/{}", page_id, property_id);
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
