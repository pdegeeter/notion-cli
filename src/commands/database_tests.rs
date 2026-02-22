use super::*;
use crate::output::OutputFormat;

#[tokio::test]
async fn test_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/databases/db-1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"db-1","object":"database"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = get(&client, "db-1", &OutputFormat::Raw).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
