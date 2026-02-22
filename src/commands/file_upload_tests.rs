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
