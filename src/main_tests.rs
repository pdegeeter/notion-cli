use super::*;
use clap::Parser;
use clap_complete::Shell;
use client::NotionClient;
use std::path::PathBuf;

fn parse(args: &[&str]) -> Cli {
    Cli::parse_from(args)
}

fn try_parse(args: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

#[test]
fn test_init_command() {
    let cli = parse(&["notion", "init"]);
    assert!(matches!(cli.command, Commands::Init));
}

#[test]
fn test_search_command() {
    let cli = parse(&["notion", "search", "my query"]);
    if let Commands::Search { query, filter } = &cli.command {
        assert_eq!(query, "my query");
        assert!(filter.is_none());
    } else {
        panic!("Expected Search command");
    }
}

#[test]
fn test_search_with_filter() {
    let cli = parse(&["notion", "search", "test", "--filter", "page"]);
    if let Commands::Search { query, filter } = &cli.command {
        assert_eq!(query, "test");
        assert_eq!(filter.as_deref(), Some("page"));
    } else {
        panic!("Expected Search command");
    }
}

#[test]
fn test_search_requires_query() {
    let result = try_parse(&["notion", "search"]);
    assert!(result.is_err());
}

#[test]
fn test_user_me() {
    let cli = parse(&["notion", "user", "me"]);
    assert!(matches!(cli.command, Commands::User(UserCommands::Me)));
}

#[test]
fn test_user_get() {
    let cli = parse(&["notion", "user", "get", "user-123"]);
    if let Commands::User(UserCommands::Get { id }) = &cli.command {
        assert_eq!(id, "user-123");
    } else {
        panic!("Expected User Get command");
    }
}

#[test]
fn test_user_list() {
    let cli = parse(&["notion", "user", "list"]);
    assert!(matches!(cli.command, Commands::User(UserCommands::List)));
}

#[test]
fn test_page_get() {
    let cli = parse(&["notion", "page", "get", "page-abc"]);
    if let Commands::Page(PageCommands::Get {
        id,
        filter_properties,
    }) = &cli.command
    {
        assert_eq!(id, "page-abc");
        assert!(filter_properties.is_empty());
    } else {
        panic!("Expected Page Get command");
    }
}

#[test]
fn test_page_get_with_filter_properties() {
    let cli = parse(&[
        "notion",
        "page",
        "get",
        "page-abc",
        "--filter-properties",
        "title,status",
    ]);
    if let Commands::Page(PageCommands::Get {
        id,
        filter_properties,
    }) = &cli.command
    {
        assert_eq!(id, "page-abc");
        assert_eq!(filter_properties, &["title", "status"]);
    } else {
        panic!("Expected Page Get command");
    }
}

#[test]
fn test_page_create() {
    let cli = parse(&[
        "notion",
        "page",
        "create",
        "--parent",
        "parent-id",
        "--properties",
        r#"{"Name":{"title":[{"text":{"content":"Test"}}]}}"#,
    ]);
    if let Commands::Page(PageCommands::Create {
        parent,
        properties,
        children,
        database_parent,
    }) = &cli.command
    {
        assert_eq!(parent, "parent-id");
        assert!(properties.contains("Name"));
        assert!(children.is_none());
        assert!(!database_parent);
    } else {
        panic!("Expected Page Create command");
    }
}

#[test]
fn test_page_create_with_database_parent() {
    let cli = parse(&[
        "notion",
        "page",
        "create",
        "--parent",
        "db-id",
        "--properties",
        "{}",
        "--database-parent",
    ]);
    if let Commands::Page(PageCommands::Create {
        database_parent, ..
    }) = &cli.command
    {
        assert!(database_parent);
    } else {
        panic!("Expected Page Create command");
    }
}

#[test]
fn test_page_update() {
    let cli = parse(&[
        "notion",
        "page",
        "update",
        "page-123",
        "--properties",
        "{}",
        "--archived",
        "true",
    ]);
    if let Commands::Page(PageCommands::Update {
        id,
        properties,
        archived,
    }) = &cli.command
    {
        assert_eq!(id, "page-123");
        assert_eq!(properties, "{}");
        assert_eq!(*archived, Some(true));
    } else {
        panic!("Expected Page Update command");
    }
}

#[test]
fn test_page_move() {
    let cli = parse(&[
        "notion",
        "page",
        "move",
        "page-1",
        "--parent-type",
        "database",
        "--to",
        "db-2",
    ]);
    if let Commands::Page(PageCommands::Move {
        id,
        parent_type,
        to,
    }) = &cli.command
    {
        assert_eq!(id, "page-1");
        assert_eq!(parent_type, "database");
        assert_eq!(to, "db-2");
    } else {
        panic!("Expected Page Move command");
    }
}

#[test]
fn test_page_property() {
    let cli = parse(&["notion", "page", "property", "page-1", "prop-abc"]);
    if let Commands::Page(PageCommands::Property {
        page_id,
        property_id,
    }) = &cli.command
    {
        assert_eq!(page_id, "page-1");
        assert_eq!(property_id, "prop-abc");
    } else {
        panic!("Expected Page Property command");
    }
}

#[test]
fn test_block_get() {
    let cli = parse(&["notion", "block", "get", "block-1"]);
    if let Commands::Block(BlockCommands::Get { id }) = &cli.command {
        assert_eq!(id, "block-1");
    } else {
        panic!("Expected Block Get command");
    }
}

#[test]
fn test_block_children() {
    let cli = parse(&["notion", "block", "children", "block-1"]);
    if let Commands::Block(BlockCommands::Children { id }) = &cli.command {
        assert_eq!(id, "block-1");
    } else {
        panic!("Expected Block Children command");
    }
}

#[test]
fn test_block_append() {
    let cli = parse(&[
        "notion",
        "block",
        "append",
        "block-1",
        "--children",
        "[{}]",
        "--after",
        "block-2",
    ]);
    if let Commands::Block(BlockCommands::Append {
        id,
        children,
        after,
    }) = &cli.command
    {
        assert_eq!(id, "block-1");
        assert_eq!(children, "[{}]");
        assert_eq!(after.as_deref(), Some("block-2"));
    } else {
        panic!("Expected Block Append command");
    }
}

#[test]
fn test_block_delete() {
    let cli = parse(&["notion", "block", "delete", "block-1"]);
    if let Commands::Block(BlockCommands::Delete { id }) = &cli.command {
        assert_eq!(id, "block-1");
    } else {
        panic!("Expected Block Delete command");
    }
}

#[test]
fn test_comment_list() {
    let cli = parse(&["notion", "comment", "list", "--block-id", "page-1"]);
    if let Commands::Comment(CommentCommands::List { block_id }) = &cli.command {
        assert_eq!(block_id, "page-1");
    } else {
        panic!("Expected Comment List command");
    }
}

#[test]
fn test_comment_create() {
    let cli = parse(&[
        "notion",
        "comment",
        "create",
        "--page-id",
        "page-1",
        "--text",
        "Hello world",
    ]);
    if let Commands::Comment(CommentCommands::Create { page_id, text }) = &cli.command {
        assert_eq!(page_id, "page-1");
        assert_eq!(text, "Hello world");
    } else {
        panic!("Expected Comment Create command");
    }
}

#[test]
fn test_db_get() {
    let cli = parse(&["notion", "db", "get", "db-123"]);
    if let Commands::Db(DbCommands::Get { id }) = &cli.command {
        assert_eq!(id, "db-123");
    } else {
        panic!("Expected Db Get command");
    }
}

#[test]
fn test_ds_get() {
    let cli = parse(&["notion", "ds", "get", "ds-1"]);
    if let Commands::Ds(DsCommands::Get { id }) = &cli.command {
        assert_eq!(id, "ds-1");
    } else {
        panic!("Expected Ds Get command");
    }
}

#[test]
fn test_ds_query_with_filter() {
    let cli = parse(&[
        "notion",
        "ds",
        "query",
        "ds-1",
        "--filter",
        r#"{"property":"Status","equals":"Done"}"#,
        "--sorts",
        r#"[{"property":"Created","direction":"descending"}]"#,
    ]);
    if let Commands::Ds(DsCommands::Query { id, filter, sorts }) = &cli.command {
        assert_eq!(id, "ds-1");
        assert!(filter.is_some());
        assert!(sorts.is_some());
    } else {
        panic!("Expected Ds Query command");
    }
}

#[test]
fn test_ds_create() {
    let cli = parse(&[
        "notion", "ds", "create", "--parent", "page-1", "--title", "My DB",
    ]);
    if let Commands::Ds(DsCommands::Create {
        parent,
        title,
        properties,
    }) = &cli.command
    {
        assert_eq!(parent, "page-1");
        assert_eq!(title, "My DB");
        assert!(properties.is_none());
    } else {
        panic!("Expected Ds Create command");
    }
}

#[test]
fn test_ds_templates() {
    let cli = parse(&["notion", "ds", "templates", "ds-1"]);
    if let Commands::Ds(DsCommands::Templates { id }) = &cli.command {
        assert_eq!(id, "ds-1");
    } else {
        panic!("Expected Ds Templates command");
    }
}

#[test]
fn test_global_output_format() {
    let cli = parse(&["notion", "--output", "raw", "init"]);
    assert!(matches!(cli.output, OutputFormat::Raw));
}

#[test]
fn test_global_raw_flag() {
    let cli = parse(&["notion", "--raw", "init"]);
    assert!(cli.raw);
}

#[test]
fn test_global_page_size() {
    let cli = parse(&["notion", "--page-size", "50", "user", "list"]);
    assert_eq!(cli.page_size, Some(50));
}

#[test]
fn test_global_start_cursor() {
    let cli = parse(&["notion", "--start-cursor", "cursor-abc", "user", "list"]);
    assert_eq!(cli.start_cursor.as_deref(), Some("cursor-abc"));
}

#[test]
fn test_default_output_is_pretty() {
    let cli = parse(&["notion", "init"]);
    assert!(matches!(cli.output, OutputFormat::Pretty));
}

#[test]
fn test_completions_command() {
    let cli = parse(&["notion", "completions", "zsh"]);
    if let Commands::Completions { shell } = &cli.command {
        assert_eq!(*shell, Shell::Zsh);
    } else {
        panic!("Expected Completions command");
    }
}

#[test]
fn test_completions_bash() {
    let cli = parse(&["notion", "completions", "bash"]);
    if let Commands::Completions { shell } = &cli.command {
        assert_eq!(*shell, Shell::Bash);
    } else {
        panic!("Expected Completions command");
    }
}

#[test]
fn test_manpage_command() {
    let cli = parse(&["notion", "manpage"]);
    assert!(matches!(cli.command, Commands::Manpage));
}

#[test]
fn test_global_dry_run_flag() {
    let cli = parse(&["notion", "--dry-run", "init"]);
    assert!(cli.dry_run);
}

#[test]
fn test_dry_run_default_is_false() {
    let cli = parse(&["notion", "init"]);
    assert!(!cli.dry_run);
}

#[test]
fn test_unknown_command_fails() {
    let result = try_parse(&["notion", "foobar"]);
    assert!(result.is_err());
}

#[test]
fn test_file_upload_create() {
    let cli = parse(&[
        "notion",
        "file-upload",
        "create",
        "--mode",
        "single_part",
        "--filename",
        "test.png",
    ]);
    if let Commands::FileUpload(FileUploadCommands::Create {
        mode,
        filename,
        content_type,
        number_of_parts,
        external_url,
    }) = &cli.command
    {
        assert_eq!(mode, "single_part");
        assert_eq!(filename.as_deref(), Some("test.png"));
        assert!(content_type.is_none());
        assert!(number_of_parts.is_none());
        assert!(external_url.is_none());
    } else {
        panic!("Expected FileUpload Create command");
    }
}

#[test]
fn test_file_upload_send() {
    let cli = parse(&[
        "notion",
        "file-upload",
        "send",
        "fu-1",
        "--file",
        "/tmp/test.png",
    ]);
    if let Commands::FileUpload(FileUploadCommands::Send {
        id,
        file,
        part_number,
    }) = &cli.command
    {
        assert_eq!(id, "fu-1");
        assert_eq!(file, &PathBuf::from("/tmp/test.png"));
        assert!(part_number.is_none());
    } else {
        panic!("Expected FileUpload Send command");
    }
}

#[test]
fn test_file_upload_send_with_part_number() {
    let cli = parse(&[
        "notion",
        "file-upload",
        "send",
        "fu-1",
        "--file",
        "/tmp/test.png",
        "--part-number",
        "2",
    ]);
    if let Commands::FileUpload(FileUploadCommands::Send {
        id,
        file,
        part_number,
    }) = &cli.command
    {
        assert_eq!(id, "fu-1");
        assert_eq!(file, &PathBuf::from("/tmp/test.png"));
        assert_eq!(*part_number, Some(2));
    } else {
        panic!("Expected FileUpload Send command");
    }
}

#[test]
fn test_file_upload_complete() {
    let cli = parse(&["notion", "file-upload", "complete", "fu-1"]);
    if let Commands::FileUpload(FileUploadCommands::Complete { id }) = &cli.command {
        assert_eq!(id, "fu-1");
    } else {
        panic!("Expected FileUpload Complete command");
    }
}

#[test]
fn test_file_upload_get() {
    let cli = parse(&["notion", "file-upload", "get", "fu-1"]);
    if let Commands::FileUpload(FileUploadCommands::Get { id }) = &cli.command {
        assert_eq!(id, "fu-1");
    } else {
        panic!("Expected FileUpload Get command");
    }
}

#[test]
fn test_file_upload_list() {
    let cli = parse(&[
        "notion",
        "file-upload",
        "list",
        "--status",
        "upload_completed",
    ]);
    if let Commands::FileUpload(FileUploadCommands::List { status }) = &cli.command {
        assert_eq!(status.as_deref(), Some("upload_completed"));
    } else {
        panic!("Expected FileUpload List command");
    }
}

#[test]
fn test_file_upload_list_no_filter() {
    let cli = parse(&["notion", "file-upload", "list"]);
    if let Commands::FileUpload(FileUploadCommands::List { status }) = &cli.command {
        assert!(status.is_none());
    } else {
        panic!("Expected FileUpload List command");
    }
}

#[test]
fn test_file_upload_upload() {
    let cli = parse(&["notion", "file-upload", "upload", "/tmp/test.png"]);
    if let Commands::FileUpload(FileUploadCommands::Upload { file, content_type }) = &cli.command {
        assert_eq!(file, &PathBuf::from("/tmp/test.png"));
        assert!(content_type.is_none());
    } else {
        panic!("Expected FileUpload Upload command");
    }
}

#[test]
fn test_file_upload_upload_with_content_type() {
    let cli = parse(&[
        "notion",
        "file-upload",
        "upload",
        "/tmp/test.png",
        "--content-type",
        "image/png",
    ]);
    if let Commands::FileUpload(FileUploadCommands::Upload { file, content_type }) = &cli.command {
        assert_eq!(file, &PathBuf::from("/tmp/test.png"));
        assert_eq!(content_type.as_deref(), Some("image/png"));
    } else {
        panic!("Expected FileUpload Upload command");
    }
}

// --- Routing integration tests using run_with_client ---

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
        Commands::User(UserCommands::Get {
            id: "user-1".into(),
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
        Commands::Block(BlockCommands::Get {
            id: "block-1".into(),
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
        Commands::Block(BlockCommands::Children {
            id: "block-1".into(),
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
        Commands::Block(BlockCommands::Delete {
            id: "block-1".into(),
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
async fn test_run_comment_list() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/v1/comments")
        .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
            "block_id".into(),
            "block-1".into(),
        )]))
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
