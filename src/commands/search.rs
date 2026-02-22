use anyhow::Result;
use serde_json::json;

use crate::client::NotionClient;
use crate::output::{print_result, OutputFormat};

pub async fn run(
    client: &NotionClient,
    query: &str,
    filter_type: Option<&str>,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut body = json!({
        "query": query,
    });

    if let Some(ft) = filter_type {
        body["filter"] = json!({
            "value": ft,
            "property": "object"
        });
    }

    if let Some(ps) = page_size {
        body["page_size"] = json!(ps);
    }

    if let Some(cursor) = start_cursor {
        body["start_cursor"] = json!(cursor);
    }

    let result = client.post("/v1/search", Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}
