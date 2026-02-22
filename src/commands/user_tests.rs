use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_me() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"user-1","type":"person","name":"Test User"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = me(&client, &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/user-42")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"user-42","type":"person"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = get(&client, "user-42", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("page_size".into(), "25".into()),
            mockito::Matcher::UrlEncoded("start_cursor".into(), "cursor-1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"results":[],"has_more":false}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = list(&client, Some(25), Some("cursor-1"), &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
