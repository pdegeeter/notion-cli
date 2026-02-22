use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/blocks/block-1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"block-1","object":"block"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = get(&client, "block-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_children() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/blocks/block-1/children")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page_size".into(), "10".into()),
            mockito::Matcher::UrlEncoded("start_cursor".into(), "cursor-abc".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = children(
        &client,
        "block-1",
        Some(10),
        Some("cursor-abc"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_append_with_after() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/blocks/block-1/children")
        .match_body(mockito::Matcher::Json(json!({
            "children": [{"object": "block", "type": "paragraph"}],
            "after": "after-block-id"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[]}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let children_json = r#"[{"object":"block","type":"paragraph"}]"#;
    let result = append(
        &client,
        "block-1",
        children_json,
        Some("after-block-id"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_append_without_after() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/blocks/block-1/children")
        .match_body(mockito::Matcher::Json(json!({
            "children": [{"object": "block", "type": "paragraph"}]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[]}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let children_json = r#"[{"object":"block","type":"paragraph"}]"#;
    let result = append(&client, "block-1", children_json, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/blocks/block-1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"block-1","archived":true}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = delete(&client, "block-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
