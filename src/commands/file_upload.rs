use anyhow::{Context, Result};
use serde_json::json;
use std::path::Path;

use crate::client::NotionClient;
use crate::output::{print_info, print_result, print_success, OutputFormat};

pub async fn create(
    client: &NotionClient,
    mode: &str,
    filename: Option<&str>,
    content_type: Option<&str>,
    number_of_parts: Option<u32>,
    external_url: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut body = json!({
        "mode": mode,
    });

    if let Some(f) = filename {
        body["filename"] = json!(f);
    }
    if let Some(ct) = content_type {
        body["content_type"] = json!(ct);
    }
    if let Some(n) = number_of_parts {
        body["number_of_parts"] = json!(n);
    }
    if let Some(url) = external_url {
        body["external_url"] = json!(url);
    }

    let result = client.post("/v1/file_uploads", Some(&body)).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn send(
    client: &NotionClient,
    upload_id: &str,
    file_path: &Path,
    part_number: Option<u32>,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/file_uploads/{}/send", upload_id);
    let result = client.post_multipart(&path, file_path, part_number).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn complete(
    client: &NotionClient,
    upload_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/file_uploads/{}/complete", upload_id);
    let result = client.post(&path, None).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn get(
    client: &NotionClient,
    upload_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let path = format!("/v1/file_uploads/{}", upload_id);
    let result = client.get(&path, &[]).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn list(
    client: &NotionClient,
    status: Option<&str>,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut query: Vec<(&str, String)> = Vec::new();

    if let Some(s) = status {
        query.push(("status", s.to_string()));
    }
    if let Some(ps) = page_size {
        query.push(("page_size", ps.to_string()));
    }
    if let Some(cursor) = start_cursor {
        query.push(("start_cursor", cursor.to_string()));
    }

    let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let result = client.get("/v1/file_uploads", &query_refs).await?;
    print_result(&result, format)?;
    Ok(())
}

pub async fn upload(
    client: &NotionClient,
    file_path: &Path,
    content_type: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Could not determine filename")?;

    // Step 1: Create upload session
    print_info(&format!("Creating upload session for '{}'...", filename));
    let mut body = json!({
        "mode": "single_part",
        "filename": filename,
    });
    if let Some(ct) = content_type {
        body["content_type"] = json!(ct);
    }

    let create_result = client.post("/v1/file_uploads", Some(&body)).await?;
    let upload_id = create_result["id"]
        .as_str()
        .context("Missing upload ID in create response")?;

    // Step 2: Send file
    print_info(&format!("Uploading file to session '{}'...", upload_id));
    let send_path = format!("/v1/file_uploads/{}/send", upload_id);
    client.post_multipart(&send_path, file_path, None).await?;

    // Step 3: Complete upload
    print_info("Completing upload...");
    let complete_path = format!("/v1/file_uploads/{}/complete", upload_id);
    let result = client.post(&complete_path, None).await?;

    print_success(&format!("File '{}' uploaded successfully", filename));
    print_result(&result, format)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::OutputFormat;

    #[tokio::test]
    async fn test_create() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/file_uploads")
            .match_body(mockito::Matcher::Json(json!({
                "mode": "single_part",
                "filename": "test.png",
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-1","status":"created","mode":"single_part"}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = create(
            &client,
            "single_part",
            Some("test.png"),
            None,
            None,
            None,
            &OutputFormat::Raw,
        )
        .await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/file_uploads/fu-1/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-1","status":"uploaded"}"#)
            .create_async()
            .await;

        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        tokio::fs::write(&file_path, b"hello").await.unwrap();

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = send(&client, "fu-1", &file_path, None, &OutputFormat::Raw).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_complete() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/file_uploads/fu-1/complete")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-1","status":"upload_completed"}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = complete(&client, "fu-1", &OutputFormat::Raw).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/file_uploads/fu-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-1","status":"upload_completed","filename":"test.png"}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = get(&client, "fu-1", &OutputFormat::Raw).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_list() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/v1/file_uploads")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("status".into(), "upload_completed".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"results":[{"id":"fu-1"}],"has_more":false}"#)
            .create_async()
            .await;

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = list(
            &client,
            Some("upload_completed"),
            None,
            None,
            &OutputFormat::Raw,
        )
        .await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_upload_orchestrates_create_send_complete() {
        let mut server = mockito::Server::new_async().await;

        let mock_create = server
            .mock("POST", "/v1/file_uploads")
            .match_body(mockito::Matcher::Json(json!({
                "mode": "single_part",
                "filename": "report.pdf",
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-42","status":"created"}"#)
            .expect(1)
            .create_async()
            .await;

        let mock_send = server
            .mock("POST", "/v1/file_uploads/fu-42/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-42","status":"uploaded"}"#)
            .expect(1)
            .create_async()
            .await;

        let mock_complete = server
            .mock("POST", "/v1/file_uploads/fu-42/complete")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"fu-42","status":"upload_completed"}"#)
            .expect(1)
            .create_async()
            .await;

        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("report.pdf");
        tokio::fs::write(&file_path, b"fake pdf content").await.unwrap();

        let client = NotionClient::with_base_url("token", &server.url()).unwrap();
        let result = upload(&client, &file_path, None, &OutputFormat::Raw).await;

        assert!(result.is_ok());
        mock_create.assert_async().await;
        mock_send.assert_async().await;
        mock_complete.assert_async().await;
    }
}
