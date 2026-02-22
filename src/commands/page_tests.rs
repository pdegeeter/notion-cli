use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/page-1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1","object":"page"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = get(&client, "page-1", &[], &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_with_filter_properties() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/page-1")
        .match_query(mockito::Matcher::Regex(
            "filter_properties=Name".to_string(),
        ))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = vec!["Name".to_string(), "Status".to_string()];
    let result = get(&client, "page-1", &props, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_page_parent() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "page_id": "parent-1" },
            "properties": { "title": [{ "text": { "content": "Test" } }] }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"new-page"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = r#"{"title":[{"text":{"content":"Test"}}]}"#;
    let result = create(&client, "parent-1", props, None, false, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_database_parent() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "database_id": "db-1" },
            "properties": { "Name": { "title": [{ "text": { "content": "Row" } }] } }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"new-page"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = r#"{"Name":{"title":[{"text":{"content":"Row"}}]}}"#;
    let result = create(&client, "db-1", props, None, true, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_children() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "page_id": "parent-1" },
            "properties": { "title": [{ "text": { "content": "Test" } }] },
            "children": [{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"text":{"content":"Hello"}}]}}]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"new-page"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = r#"{"title":[{"text":{"content":"Test"}}]}"#;
    let children = r#"[{"object":"block","type":"paragraph","paragraph":{"rich_text":[{"text":{"content":"Hello"}}]}}]"#;
    let result = create(&client, "parent-1", props, Some(children), false, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_with_archived() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/pages/page-1")
        .match_body(mockito::Matcher::Json(json!({
            "properties": {},
            "archived": true
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1","archived":true}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = update(&client, "page-1", "{}", Some(true), &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_move_page_parent() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages/page-1/move")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "type": "page_id", "page_id": "target-1" }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = move_page(&client, "page-1", "page", "target-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_move_invalid_parent_type() {
    let server = mockito::Server::new_async().await;
    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = move_page(&client, "page-1", "invalid", "target-1", &OutputFormat::Raw).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid parent type")
    );
}

#[tokio::test]
async fn test_update_without_archived() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/pages/page-1")
        .match_body(mockito::Matcher::Json(json!({
            "properties": { "Name": { "title": [{ "text": { "content": "Updated" } }] } }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = r#"{"Name":{"title":[{"text":{"content":"Updated"}}]}}"#;
    let result = update(&client, "page-1", props, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_invalid_json() {
    let server = mockito::Server::new_async().await;
    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = create(&client, "parent-1", "not valid json", None, false, &OutputFormat::Raw).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
}

#[tokio::test]
async fn test_property_with_pagination() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/page-1/properties/prop-1")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page_size".into(), "25".into()),
            mockito::Matcher::UrlEncoded("start_cursor".into(), "cursor-xyz".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = property(
        &client,
        "page-1",
        "prop-1",
        Some(25),
        Some("cursor-xyz"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_move_to_database_parent() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages/page-1/move")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "type": "database_id", "database_id": "db-1" }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = move_page(&client, "page-1", "database", "db-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_move_to_workspace() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages/page-1/move")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "type": "workspace" }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"page-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = move_page(&client, "page-1", "workspace", "unused", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
