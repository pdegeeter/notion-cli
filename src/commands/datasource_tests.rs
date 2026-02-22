use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/data_sources/ds-1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"ds-1","object":"data_source"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = get(&client, "ds-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_without_properties() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "page_id": "page-1" },
            "title": [{ "type": "text", "text": { "content": "My DS" } }]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"ds-new"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = create(&client, "page-1", "My DS", None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_properties() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources")
        .match_body(mockito::Matcher::Json(json!({
            "parent": { "page_id": "page-1" },
            "title": [{ "type": "text", "text": { "content": "My DS" } }],
            "properties": { "Name": { "title": {} } }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"ds-new"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let props = r#"{"Name":{"title":{}}}"#;
    let result = create(&client, "page-1", "My DS", Some(props), &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_with_all_options() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources/ds-1/query")
        .match_body(mockito::Matcher::Json(json!({
            "filter": { "property": "Status", "select": { "equals": "Done" } },
            "sorts": [{ "property": "Name", "direction": "ascending" }],
            "page_size": 10,
            "start_cursor": "cursor-1"
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let filter = r#"{"property":"Status","select":{"equals":"Done"}}"#;
    let sorts = r#"[{"property":"Name","direction":"ascending"}]"#;
    let result = query(
        &client,
        "ds-1",
        Some(filter),
        Some(sorts),
        Some(10),
        Some("cursor-1"),
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_templates() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/data_sources/ds-1/templates")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[]}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = templates(&client, "ds-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_without_options() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources/ds-1/query")
        .match_body(mockito::Matcher::Json(json!({})))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = query(&client, "ds-1", None, None, None, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_query_with_filter_only() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources/ds-1/query")
        .match_body(mockito::Matcher::Json(json!({
            "filter": { "property": "Status", "select": { "equals": "Done" } }
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let filter = r#"{"property":"Status","select":{"equals":"Done"}}"#;
    let result = query(&client, "ds-1", Some(filter), None, None, None, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_update() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/data_sources/ds-1")
        .match_body(mockito::Matcher::Json(json!({
            "title": [{ "text": { "content": "Updated" } }]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"ds-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let data = r#"{"title":[{"text":{"content":"Updated"}}]}"#;
    let result = update(&client, "ds-1", data, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_with_invalid_json() {
    let server = mockito::Server::new_async().await;
    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = create(&client, "page-1", "My DS", Some("not json"), &OutputFormat::Raw).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
}
