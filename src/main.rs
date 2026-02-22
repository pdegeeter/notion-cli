mod client;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use output::OutputFormat;
use std::path::PathBuf;

/// Notion CLI - Interact with the Notion API from the command line
#[derive(Parser)]
#[command(name = "notion", version, about)]
#[command(disable_version_flag = true)]
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

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: (),

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

    /// File upload operations
    #[command(name = "file-upload", subcommand)]
    FileUpload(FileUploadCommands),

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

#[derive(Subcommand)]
enum FileUploadCommands {
    /// Create a file upload session
    #[command(arg_required_else_help = true)]
    Create {
        /// Upload mode: single_part, multi_part, or external_url
        #[arg(long)]
        mode: String,

        /// Filename for the upload
        #[arg(long)]
        filename: Option<String>,

        /// MIME content type
        #[arg(long)]
        content_type: Option<String>,

        /// Number of parts (multi_part mode)
        #[arg(long)]
        number_of_parts: Option<u32>,

        /// External URL (external_url mode)
        #[arg(long)]
        external_url: Option<String>,
    },

    /// Send a file to an upload session
    #[command(arg_required_else_help = true)]
    Send {
        /// File upload ID
        id: String,

        /// Path to the file to upload
        #[arg(long)]
        file: PathBuf,

        /// Part number (multi_part mode)
        #[arg(long)]
        part_number: Option<u32>,
    },

    /// Complete a file upload
    #[command(arg_required_else_help = true)]
    Complete {
        /// File upload ID
        id: String,
    },

    /// Retrieve a file upload
    #[command(arg_required_else_help = true)]
    Get {
        /// File upload ID
        id: String,
    },

    /// List file uploads
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },

    /// Upload a file in one step (create + send + complete)
    #[command(arg_required_else_help = true)]
    Upload {
        /// Path to the file to upload
        file: PathBuf,

        /// MIME content type
        #[arg(long)]
        content_type: Option<String>,
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

        Commands::FileUpload(cmd) => match cmd {
            FileUploadCommands::Create {
                mode,
                filename,
                content_type,
                number_of_parts,
                external_url,
            } => {
                commands::file_upload::create(
                    &notion,
                    mode,
                    filename.as_deref(),
                    content_type.as_deref(),
                    *number_of_parts,
                    external_url.as_deref(),
                    &format,
                )
                .await
            }
            FileUploadCommands::Send {
                id,
                file,
                part_number,
            } => {
                commands::file_upload::send(&notion, id, file, *part_number, &format).await
            }
            FileUploadCommands::Complete { id } => {
                commands::file_upload::complete(&notion, id, &format).await
            }
            FileUploadCommands::Get { id } => {
                commands::file_upload::get(&notion, id, &format).await
            }
            FileUploadCommands::List { status } => {
                commands::file_upload::list(
                    &notion,
                    status.as_deref(),
                    cli.page_size,
                    cli.start_cursor.as_deref(),
                    &format,
                )
                .await
            }
            FileUploadCommands::Upload {
                file,
                content_type,
            } => {
                commands::file_upload::upload(&notion, file, content_type.as_deref(), &format)
                    .await
            }
        },

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
#[path = "main_tests.rs"]
mod tests;
