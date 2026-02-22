use super::*;
use client::NotionClient;
use output::OutputFormat;

fn mock_json() -> &'static str {
    r#"{"ok":true}"#
}

fn json_header() -> (&'static str, &'static str) {
    ("content-type", "application/json")
}

#[tokio::test]
async fn test_run_search() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Search {
            query: "test".into(),
            filter: None,
        },
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_search_with_filter() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/search")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Search {
            query: "test".into(),
            filter: Some("page".into()),
        },
        &client,
        Some(10),
        Some("cursor-1"),
        &OutputFormat::Pretty,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_user_me() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::User(UserCommands::Me),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_user_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/user-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::User(UserCommands::Get { id: "user-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_user_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::User(UserCommands::List),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_page_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/page-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Page(PageCommands::Get {
            id: "page-1".into(),
            filter_properties: vec![],
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_page_create() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Page(PageCommands::Create {
            parent: "parent-1".into(),
            properties: r#"{"title":[{"text":{"content":"Test"}}]}"#.into(),
            children: None,
            database_parent: false,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_page_update() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/pages/page-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Page(PageCommands::Update {
            id: "page-1".into(),
            properties: "{}".into(),
            archived: Some(true),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_page_move() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/pages/page-1/move")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Page(PageCommands::Move {
            id: "page-1".into(),
            parent_type: "page".into(),
            to: "target-1".into(),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_page_property() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/pages/page-1/properties/prop-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Page(PageCommands::Property {
            page_id: "page-1".into(),
            property_id: "prop-1".into(),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_block_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/blocks/block-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Block(BlockCommands::Get { id: "block-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_block_children() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/blocks/block-1/children")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Block(BlockCommands::Children { id: "block-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_block_append() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/blocks/block-1/children")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Block(BlockCommands::Append {
            id: "block-1".into(),
            children: r#"[{"object":"block","type":"paragraph"}]"#.into(),
            after: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_block_update() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/blocks/block-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Block(BlockCommands::Update {
            id: "block-1".into(),
            data: r#"{"type":"paragraph"}"#.into(),
            archived: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_block_delete() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("DELETE", "/v1/blocks/block-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Block(BlockCommands::Delete { id: "block-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_comment_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/comments")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("block_id".into(), "block-1".into()),
        ]))
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Comment(CommentCommands::List {
            block_id: "block-1".into(),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_comment_create() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/comments")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Comment(CommentCommands::Create {
            page_id: "page-1".into(),
            text: "Hello".into(),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_db_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/databases/db-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Db(DbCommands::Get { id: "db-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_ds_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/data_sources/ds-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Ds(DsCommands::Get { id: "ds-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_ds_create() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Ds(DsCommands::Create {
            parent: "page-1".into(),
            title: "My DS".into(),
            properties: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_ds_update() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("PATCH", "/v1/data_sources/ds-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Ds(DsCommands::Update {
            id: "ds-1".into(),
            data: r#"{"title":[{"text":{"content":"Updated"}}]}"#.into(),
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_ds_query() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/data_sources/ds-1/query")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Ds(DsCommands::Query {
            id: "ds-1".into(),
            filter: None,
            sorts: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_ds_templates() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/data_sources/ds-1/templates")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::Ds(DsCommands::Templates { id: "ds-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_create() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::Create {
            mode: "single_part".into(),
            filename: Some("test.png".into()),
            content_type: None,
            number_of_parts: None,
            external_url: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_send() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads/fu-1/send")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    tokio::fs::write(&file_path, b"hello").await.unwrap();

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::Send {
            id: "fu-1".into(),
            file: file_path,
            part_number: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_complete() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1/file_uploads/fu-1/complete")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::Complete { id: "fu-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_get() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/file_uploads/fu-1")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::Get { id: "fu-1".into() }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/file_uploads")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(mock_json())
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::List { status: None }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_file_upload_upload() {
    let mut server = mockito::Server::new_async().await;
    let mock_create = server
        .mock("POST", "/v1/file_uploads")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(r#"{"id":"fu-42","status":"created"}"#)
        .create_async()
        .await;
    let mock_send = server
        .mock("POST", "/v1/file_uploads/fu-42/send")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(r#"{"id":"fu-42","status":"uploaded"}"#)
        .create_async()
        .await;
    let mock_complete = server
        .mock("POST", "/v1/file_uploads/fu-42/complete")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(r#"{"id":"fu-42","status":"upload_completed"}"#)
        .create_async()
        .await;

    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("report.pdf");
    tokio::fs::write(&file_path, b"fake pdf").await.unwrap();

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::FileUpload(FileUploadCommands::Upload {
            file: file_path,
            content_type: None,
        }),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock_create.assert_async().await;
    mock_send.assert_async().await;
    mock_complete.assert_async().await;
}

#[tokio::test]
async fn test_run_with_raw_format() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(r#"{"id":"user-1","name":"Bot"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::User(UserCommands::Me),
        &client,
        None,
        None,
        &OutputFormat::Raw,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_run_with_pretty_format() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/users/me")
        .with_status(200)
        .with_header(json_header().0, json_header().1)
        .with_body(r#"{"id":"user-1","name":"Bot"}"#)
        .create_async()
        .await;

    let client = NotionClient::with_base_url("token", &server.url()).unwrap();
    let result = run_with_client(
        Commands::User(UserCommands::Me),
        &client,
        None,
        None,
        &OutputFormat::Pretty,
    )
    .await;

    assert!(result.is_ok());
    mock.assert_async().await;
}
