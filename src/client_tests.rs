use super::*;
use serde_json::json;
use tokio;

#[test]
fn test_new_client_with_valid_token() {
    let client = NotionClient::new("ntn_test_token");
    assert!(client.is_ok());
}

#[test]
fn test_new_client_with_empty_token() {
    // Empty token is valid (will fail at API level, not at construction)
    let client = NotionClient::new("");
    assert!(client.is_ok());
}

#[test]
fn test_with_base_url() {
    let client = NotionClient::with_base_url("token", "http://localhost:8080").unwrap();
    assert_eq!(client.base_url, "http://localhost:8080");
}

#[test]
fn test_default_base_url() {
    let client = NotionClient::new("token").unwrap();
    assert_eq!(client.base_url, "https://api.notion.com");
}

#[test]
fn test_notion_version_constant() {
    assert_eq!(NOTION_VERSION, "2025-09-03");
}

#[tokio::test]
async fn test_get_with_mock_server() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"object":"user","id":"abc-123","name":"Test Bot","type":"bot"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("test_token", &server.url()).unwrap();
    let result = client.get("/v1/users/me", &[]).await.unwrap();

    assert_eq!(result["object"], "user");
    assert_eq!(result["name"], "Test Bot");
    assert_eq!(result["type"], "bot");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_with_query_params() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "page_size".into(),
            "10".into(),
        )]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client
        .get("/v1/users", &[("page_size", "10")])
        .await
        .unwrap();

    assert_eq!(result["has_more"], false);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_post_with_body() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .match_body(mockito::Matcher::Json(json!({"query": "test"})))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[{"id":"page-1"}],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let body = json!({"query": "test"});
    let result = client.post("/v1/search", Some(&body)).await.unwrap();

    assert_eq!(result["results"][0]["id"], "page-1");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_post_without_body() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/endpoint")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"ok":true}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client.post("/v1/endpoint", None).await.unwrap();

    assert_eq!(result["ok"], true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_patch_request() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/pages/page-123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-123","properties":{}}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let body = json!({"properties": {"Name": {"title": [{"text": {"content": "Updated"}}]}}});
    let result = client.patch("/v1/pages/page-123", &body).await.unwrap();

    assert_eq!(result["id"], "page-123");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_request() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/blocks/block-456")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"block-456","archived":true}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client.delete("/v1/blocks/block-456").await.unwrap();

    assert_eq!(result["archived"], true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_handle_api_error_401() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"object":"error","status":401,"code":"unauthorized","message":"API token is invalid."}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("bad_token", &server.url()).unwrap();
    let result = client.get("/v1/users/me", &[]).await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("unauthorized"));
    assert!(err.contains("API token is invalid"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_handle_api_error_404() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"object":"error","status":404,"code":"object_not_found","message":"Could not find page."}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client.get("/v1/pages/nonexistent", &[]).await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("object_not_found"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_handle_api_error_with_missing_fields() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/test")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"unexpected":"format"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client.get("/v1/test", &[]).await;

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Unknown error"));
    mock.assert_async().await;
}

#[test]
fn test_dry_run_default_is_false() {
    let client = NotionClient::new("token").unwrap();
    assert!(!client.dry_run);
}

#[test]
fn test_set_dry_run() {
    let mut client = NotionClient::new("token").unwrap();
    client.set_dry_run(true);
    assert!(client.dry_run);
}

#[tokio::test]
async fn test_dry_run_post_does_not_send_request() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages")
        .expect(0)
        .create_async()
        .await;

    let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
    client.set_dry_run(true);
    let body = json!({"properties": {}});
    let result = client.post("/v1/pages", Some(&body)).await.unwrap();

    assert_eq!(result["dry_run"], true);
    assert_eq!(result["method"], "POST");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_dry_run_patch_does_not_send_request() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/pages/p1")
        .expect(0)
        .create_async()
        .await;

    let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
    client.set_dry_run(true);
    let body = json!({});
    let result = client.patch("/v1/pages/p1", &body).await.unwrap();

    assert_eq!(result["dry_run"], true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_dry_run_delete_does_not_send_request() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/blocks/b1")
        .expect(0)
        .create_async()
        .await;

    let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
    client.set_dry_run(true);
    let result = client.delete("/v1/blocks/b1").await.unwrap();

    assert_eq!(result["dry_run"], true);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_retry_on_429_then_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_429 = server
        .mock("GET", "/v1/test")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_header("retry-after", "0")
        .with_body(
            r#"{"object":"error","status":429,"code":"rate_limited","message":"Rate limited"}"#,
        )
        .expect(1)
        .create_async()
        .await;
    let mock_200 = server
        .mock("GET", "/v1/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"ok":true}"#)
        .expect(1)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client.get("/v1/test", &[]).await.unwrap();

    assert_eq!(result["ok"], true);
    mock_429.assert_async().await;
    mock_200.assert_async().await;
}

#[tokio::test]
async fn test_post_multipart_with_mock_server() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads/upload-1/send")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"upload-1","status":"uploaded"}"#)
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.png");
    tokio::fs::write(&file_path, b"fake png content")
        .await
        .unwrap();

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client
        .post_multipart("/v1/file_uploads/upload-1/send", &file_path, None)
        .await
        .unwrap();

    assert_eq!(result["id"], "upload-1");
    assert_eq!(result["status"], "uploaded");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_post_multipart_dry_run() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads/upload-1/send")
        .expect(0)
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    tokio::fs::write(&file_path, b"hello").await.unwrap();

    let mut client = NotionClient::with_base_url("token", &server.url()).unwrap();
    client.set_dry_run(true);
    let result = client
        .post_multipart("/v1/file_uploads/upload-1/send", &file_path, Some(1))
        .await
        .unwrap();

    assert_eq!(result["dry_run"], true);
    assert_eq!(result["method"], "POST");
    assert_eq!(result["file_size"], 5);
    mock.assert_async().await;
}

#[test]
fn test_mime_from_filename() {
    assert_eq!(super::mime_from_filename("photo.png"), "image/png");
    assert_eq!(super::mime_from_filename("doc.pdf"), "application/pdf");
    assert_eq!(super::mime_from_filename("data.csv"), "text/csv");
    assert_eq!(
        super::mime_from_filename("unknown.xyz"),
        "application/octet-stream"
    );
    assert_eq!(super::mime_from_filename("IMAGE.JPG"), "image/jpeg");
}

#[test]
fn test_mime_from_filename_all_types() {
    assert_eq!(super::mime_from_filename("a.gif"), "image/gif");
    assert_eq!(super::mime_from_filename("a.webp"), "image/webp");
    assert_eq!(super::mime_from_filename("a.svg"), "image/svg+xml");
    assert_eq!(super::mime_from_filename("a.html"), "text/html");
    assert_eq!(super::mime_from_filename("a.htm"), "text/html");
    assert_eq!(super::mime_from_filename("a.mp4"), "video/mp4");
    assert_eq!(super::mime_from_filename("a.mp3"), "audio/mpeg");
    assert_eq!(super::mime_from_filename("a.zip"), "application/zip");
    assert_eq!(super::mime_from_filename("a.doc"), "application/msword");
    assert_eq!(
        super::mime_from_filename("a.docx"),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    );
    assert_eq!(
        super::mime_from_filename("a.xls"),
        "application/vnd.ms-excel"
    );
    assert_eq!(
        super::mime_from_filename("a.xlsx"),
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    );
    assert_eq!(super::mime_from_filename("a.json"), "application/json");
    assert_eq!(super::mime_from_filename("a.txt"), "text/plain");
    assert_eq!(super::mime_from_filename("a.jpeg"), "image/jpeg");
}

#[tokio::test]
async fn test_post_multipart_file_not_found() {
    let server = mockito::Server::new_async().await;
    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client
        .post_multipart(
            "/v1/file_uploads/fu-1/send",
            std::path::Path::new("/nonexistent/file.txt"),
            None,
        )
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to read file")
    );
}

#[tokio::test]
async fn test_post_multipart_with_part_number() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads/fu-1/send")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"fu-1","status":"uploaded"}"#)
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("chunk.bin");
    tokio::fs::write(&file_path, b"chunk data").await.unwrap();

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = client
        .post_multipart("/v1/file_uploads/fu-1/send", &file_path, Some(2))
        .await
        .unwrap();

    assert_eq!(result["id"], "fu-1");
    mock.assert_async().await;
}
