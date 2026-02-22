use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

use crate::output::OutputFormat;

/// Notion CLI - Interact with the Notion API from the command line
#[derive(Parser)]
#[command(name = "notion", version, about)]
#[command(disable_version_flag = true)]
pub struct Cli {
    /// Output format
    #[arg(long, global = true, default_value = "pretty")]
    pub output: OutputFormat,

    /// Raw JSON output (shorthand for --output raw)
    #[arg(long, global = true)]
    pub raw: bool,

    /// Show the request without executing it (write operations only)
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Print version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    version: (),

    /// Number of items per page (max 100)
    #[arg(long, global = true)]
    pub page_size: Option<u32>,

    /// Pagination cursor
    #[arg(long, global = true)]
    pub start_cursor: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum UserCommands {
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
pub enum PageCommands {
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
pub enum BlockCommands {
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
pub enum CommentCommands {
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
pub enum DbCommands {
    /// Retrieve database metadata
    #[command(arg_required_else_help = true)]
    Get {
        /// Database ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum DsCommands {
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
pub enum FileUploadCommands {
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

#[cfg(test)]
#[path = "cli_tests.rs"]
mod tests;
