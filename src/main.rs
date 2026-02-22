mod client;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use output::OutputFormat;

/// Notion CLI - Interact with the Notion API from the command line
#[derive(Parser)]
#[command(name = "notion", version, about)]
struct Cli {
    /// Output format
    #[arg(long, global = true, default_value = "pretty")]
    output: OutputFormat,

    /// Raw JSON output (shorthand for --output raw)
    #[arg(long, global = true)]
    raw: bool,

    /// Show the request without executing it (write operations only)
    #[arg(long, global = true)]
    dry_run: bool,

    /// Number of items per page (max 100)
    #[arg(long, global = true)]
    page_size: Option<u32>,

    /// Pagination cursor
    #[arg(long, global = true)]
    start_cursor: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration and test connection
    Init,

    /// Search pages and databases by title
    #[command(arg_required_else_help = true)]
    Search {
        /// Search query
        query: String,

        /// Filter by type: page or data_source
        #[arg(long, short)]
        filter: Option<String>,
    },

    /// User operations
    #[command(subcommand)]
    User(UserCommands),

    /// Page operations
    #[command(subcommand)]
    Page(PageCommands),

    /// Block operations
    #[command(subcommand)]
    Block(BlockCommands),

    /// Comment operations
    #[command(subcommand)]
    Comment(CommentCommands),

    /// Database operations
    #[command(subcommand)]
    Db(DbCommands),

    /// Data source operations
    #[command(subcommand)]
    Ds(DsCommands),

    /// Generate shell completions
    #[command(arg_required_else_help = true)]
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },

    /// Generate man page
    Manpage,
}

#[derive(Subcommand)]
enum UserCommands {
    /// Get the current bot user
    Me,

    /// Get a user by ID
    #[command(arg_required_else_help = true)]
    Get {
        /// User ID
        id: String,
    },

    /// List all users
    List,
}

#[derive(Subcommand)]
enum PageCommands {
    /// Retrieve a page
    #[command(arg_required_else_help = true)]
    Get {
        /// Page ID
        id: String,

        /// Filter to specific property IDs (comma-separated or repeated)
        #[arg(long, value_delimiter = ',')]
        filter_properties: Vec<String>,
    },

    /// Create a new page
    #[command(arg_required_else_help = true)]
    Create {
        /// Parent page or database ID
        #[arg(long)]
        parent: String,

        /// Properties as JSON string
        #[arg(long)]
        properties: String,

        /// Children blocks as JSON string
        #[arg(long)]
        children: Option<String>,

        /// Parent is a database (default: page)
        #[arg(long)]
        database_parent: bool,
    },

    /// Update page properties
    #[command(arg_required_else_help = true)]
    Update {
        /// Page ID
        id: String,

        /// Properties as JSON string
        #[arg(long)]
        properties: String,

        /// Archive/unarchive the page
        #[arg(long)]
        archived: Option<bool>,
    },

    /// Move a page to a different parent
    #[command(arg_required_else_help = true)]
    Move {
        /// Page ID to move
        id: String,

        /// Parent type: page, database, or workspace
        #[arg(long, default_value = "page")]
        parent_type: String,

        /// Destination parent ID (not needed for workspace)
        #[arg(long)]
        to: String,
    },

    /// Get a page property value
    #[command(arg_required_else_help = true)]
    Property {
        /// Page ID
        page_id: String,

        /// Property ID
        property_id: String,
    },
}

#[derive(Subcommand)]
enum BlockCommands {
    /// Retrieve a block
    #[command(arg_required_else_help = true)]
    Get {
        /// Block ID
        id: String,
    },

    /// List block children
    #[command(arg_required_else_help = true)]
    Children {
        /// Block ID
        id: String,
    },

    /// Append children to a block
    #[command(arg_required_else_help = true)]
    Append {
        /// Block ID
        id: String,

        /// Children blocks as JSON string
        #[arg(long)]
        children: String,

        /// Insert after this block ID
        #[arg(long)]
        after: Option<String>,
    },

    /// Update a block
    #[command(arg_required_else_help = true)]
    Update {
        /// Block ID
        id: String,

        /// Block data as JSON string
        #[arg(long)]
        data: String,

        /// Archive/unarchive the block
        #[arg(long)]
        archived: Option<bool>,
    },

    /// Delete a block
    #[command(arg_required_else_help = true)]
    Delete {
        /// Block ID
        id: String,
    },
}

#[derive(Subcommand)]
enum CommentCommands {
    /// List comments on a block or page
    #[command(arg_required_else_help = true)]
    List {
        /// Block or page ID
        #[arg(long)]
        block_id: String,
    },

    /// Create a comment on a page
    #[command(arg_required_else_help = true)]
    Create {
        /// Page ID
        #[arg(long)]
        page_id: String,

        /// Comment text
        #[arg(long)]
        text: String,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Retrieve database metadata
    #[command(arg_required_else_help = true)]
    Get {
        /// Database ID
        id: String,
    },
}

#[derive(Subcommand)]
enum DsCommands {
    /// Retrieve a data source
    #[command(arg_required_else_help = true)]
    Get {
        /// Data source ID
        id: String,
    },

    /// Create a data source
    #[command(arg_required_else_help = true)]
    Create {
        /// Parent page ID
        #[arg(long)]
        parent: String,

        /// Title
        #[arg(long)]
        title: String,

        /// Properties schema as JSON string
        #[arg(long)]
        properties: Option<String>,
    },

    /// Update a data source
    #[command(arg_required_else_help = true)]
    Update {
        /// Data source ID
        id: String,

        /// Data as JSON string
        #[arg(long)]
        data: String,
    },

    /// Query a data source
    #[command(arg_required_else_help = true)]
    Query {
        /// Data source ID
        id: String,

        /// Filter as JSON string
        #[arg(long)]
        filter: Option<String>,

        /// Sorts as JSON string
        #[arg(long)]
        sorts: Option<String>,
    },

    /// List templates in a data source
    #[command(arg_required_else_help = true)]
    Templates {
        /// Data source ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let format = if cli.raw {
        OutputFormat::Raw
    } else {
        cli.output
    };

    // Commands that don't need an API client
    match &cli.command {
        Commands::Init => {
            return commands::init::run().await;
        }
        Commands::Completions { shell } => {
            clap_complete::generate(
                *shell,
                &mut Cli::command(),
                "notion",
                &mut std::io::stdout(),
            );
            return Ok(());
        }
        Commands::Manpage => {
            let cmd = Cli::command();
            let man = clap_mangen::Man::new(cmd);
            man.render(&mut std::io::stdout())?;
            return Ok(());
        }
        _ => {}
    }

    // All other commands need an authenticated client
    let config = config::Config::load()?;
    let token = config.get_token()?;
    let mut notion = client::NotionClient::new(token)?;
    notion.set_dry_run(cli.dry_run);

    match &cli.command {
        Commands::Init | Commands::Completions { .. } | Commands::Manpage => unreachable!(),

        Commands::Search { query, filter } => {
            commands::search::run(
                &notion,
                query,
                filter.as_deref(),
                cli.page_size,
                cli.start_cursor.as_deref(),
                &format,
            )
            .await
        }

        Commands::User(cmd) => match cmd {
            UserCommands::Me => commands::user::me(&notion, &format).await,
            UserCommands::Get { id } => commands::user::get(&notion, id, &format).await,
            UserCommands::List => {
                commands::user::list(
                    &notion,
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
        },

        Commands::Page(cmd) => match cmd {
            PageCommands::Get { id, filter_properties } => {
                commands::page::get(&notion, id, filter_properties, &format).await
            }
            PageCommands::Create {
                parent,
                properties,
                children,
                database_parent,
            } => {
                commands::page::create(
                    &notion,
                    parent,
                    properties,
                    children.as_deref(),
                    *database_parent,
                    &format,
                )
                .await
            }
            PageCommands::Update {
                id,
                properties,
                archived,
            } => commands::page::update(&notion, id, properties, *archived, &format).await,
            PageCommands::Move {
                id,
                parent_type,
                to,
            } => commands::page::move_page(&notion, id, parent_type, to, &format).await,
            PageCommands::Property {
                page_id,
                property_id,
            } => {
                commands::page::property(
                    &notion,
                    page_id,
                    property_id,
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
        },

        Commands::Block(cmd) => match cmd {
            BlockCommands::Get { id } => commands::block::get(&notion, id, &format).await,
            BlockCommands::Children { id } => {
                commands::block::children(
                    &notion,
                    id,
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
            BlockCommands::Append {
                id,
                children,
                after,
            } => {
                commands::block::append(&notion, id, children, after.as_deref(), &format).await
            }
            BlockCommands::Update { id, data, archived } => {
                commands::block::update(&notion, id, data, *archived, &format).await
            }
            BlockCommands::Delete { id } => commands::block::delete(&notion, id, &format).await,
        },

        Commands::Comment(cmd) => match cmd {
            CommentCommands::List { block_id } => {
                commands::comment::list(
                    &notion,
                    block_id,
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
            CommentCommands::Create { page_id, text } => {
                commands::comment::create(&notion, page_id, text, &format).await
            }
        },

        Commands::Db(cmd) => match cmd {
            DbCommands::Get { id } => commands::database::get(&notion, id, &format).await,
        },

        Commands::Ds(cmd) => match cmd {
            DsCommands::Get { id } => commands::datasource::get(&notion, id, &format).await,
            DsCommands::Create {
                parent,
                title,
                properties,
            } => {
                commands::datasource::create(
                    &notion,
                    parent,
                    title,
                    properties.as_deref(),
                    &format,
                )
                .await
            }
            DsCommands::Update { id, data } => {
                commands::datasource::update(&notion, id, data, &format).await
            }
            DsCommands::Query {
                id,
                filter,
                sorts,
            } => {
                commands::datasource::query(
                    &notion,
                    id,
                    filter.as_deref(),
                    sorts.as_deref(),
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
            DsCommands::Templates { id } => {
                commands::datasource::templates(&notion, id, &format).await
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

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
        assert!(matches!(
            cli.command,
            Commands::User(UserCommands::List)
        ));
    }

    #[test]
    fn test_page_get() {
        let cli = parse(&["notion", "page", "get", "page-abc"]);
        if let Commands::Page(PageCommands::Get { id, filter_properties }) = &cli.command {
            assert_eq!(id, "page-abc");
            assert!(filter_properties.is_empty());
        } else {
            panic!("Expected Page Get command");
        }
    }

    #[test]
    fn test_page_get_with_filter_properties() {
        let cli = parse(&[
            "notion", "page", "get", "page-abc",
            "--filter-properties", "title,status",
        ]);
        if let Commands::Page(PageCommands::Get { id, filter_properties }) = &cli.command {
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
        if let Commands::Ds(DsCommands::Query {
            id,
            filter,
            sorts,
        }) = &cli.command
        {
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
            "notion",
            "ds",
            "create",
            "--parent",
            "page-1",
            "--title",
            "My DB",
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
        let cli = parse(&[
            "notion",
            "--start-cursor",
            "cursor-abc",
            "user",
            "list",
        ]);
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
}
