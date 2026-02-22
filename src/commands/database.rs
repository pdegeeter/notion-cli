use anyhow::Result;

use crate::client::NotionClient;
use crate::output::{print_result, OutputFormat};

pub async fn get(
    client: &NotionClient,
    database_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/databases/{}", database_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}
