use super::*;

#[tokio::test]
async fn test_verify_connection_success() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "id": "bot-1",
                "type": "bot",
                "name": "My Integration",
                "bot": {
                    "workspace_name": "Test Workspace"
                }
            }"#,
        )
        .create_async()
        .await;

    let client = NotionClient::with_base_url("test-token", &server.url()).unwrap();
    let result = verify_connection(&client).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_verify_connection_failure() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"object":"error","status":401,"code":"unauthorized","message":"API token is invalid."}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("bad-token", &server.url()).unwrap();
    let result = verify_connection(&client).await;

    assert!(result.is_err());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_verify_connection_missing_fields() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "bot-1"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("test-token", &server.url()).unwrap();
    let result = verify_connection(&client).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
