use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/comments")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "block_id".into(),
            "block-1".into(),
        )]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = list(&client, "block-1", None, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/comments")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "page_id": "page-1" },
            "rich_text": [{ "type": "text", "text": { "content": "Hello world" } }]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"comment-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = create(&client, "page-1", "Hello world", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_with_page_size() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/comments")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("block_id".into(), "block-1".into()),
            mockito::Matcher::UrlEncoded("page_size".into(), "10".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = list(&client, "block-1", Some(10), None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_with_start_cursor() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/comments")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("block_id".into(), "block-1".into()),
            mockito::Matcher::UrlEncoded("start_cursor".into(), "cursor-abc".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = list(
        &client,
        "block-1",
        None,
        Some("cursor-abc"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
