use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_run_basic() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .match_body(mockito::Matcher::Json(json!({
            "query": "my search"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run(&client, "my search", None, None, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_with_filter() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .match_body(mockito::Matcher::Json(json!({
            "query": "test",
            "filter": { "value": "page", "property": "object" }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run(&client, "test", Some("page"), None, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_with_all_options() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .match_body(mockito::Matcher::Json(json!({
            "query": "test",
            "filter": { "value": "database", "property": "object" },
            "page_size": 5,
            "start_cursor": "cursor-xyz"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run(
        &client,
        "test",
        Some("database"),
        Some(5),
        Some("cursor-xyz"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
