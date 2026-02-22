mod cli;
mod client;
mod commands;
mod config;
mod output;

pub use cli::*;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use output::OutputFormat;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli).await
}

pub async fn run(cli: Cli) -> Result<()> {
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

    run_with_client(
        cli.command,
        &notion,
        cli.page_size,
        cli.start_cursor.as_deref(),
        &format,
    )
    .await
}

pub async fn run_with_client(
    command: Commands,
    notion: &client::NotionClient,
    page_size: Option<u32>,
    start_cursor: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    match &command {
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
                    notion,
                    mode,
                    filename.as_deref(),
                    content_type.as_deref(),
                    *number_of_parts,
                    external_url.as_deref(),
                    format,
                )
                .await
            }
            FileUploadCommands::Send {
                id,
                file,
                part_number,
            } => commands::file_upload::send(notion, id, file, *part_number, format).await,
            FileUploadCommands::Complete { id } => {
                commands::file_upload::complete(notion, id, format).await
            }
            FileUploadCommands::Get { id } => commands::file_upload::get(notion, id, format).await,
            FileUploadCommands::List { status } => {
                commands::file_upload::list(
                    notion,
                    status.as_deref(),
                    page_size,
                    start_cursor,
                    format,
                )
                .await
            }
            FileUploadCommands::Upload { file, content_type } => {
                commands::file_upload::upload(notion, file, content_type.as_deref(), format).await
            }
        },

        Commands::Search { query, filter } => {
            commands::search::run(
                notion,
                query,
                filter.as_deref(),
                page_size,
                start_cursor,
                format,
            )
            .await
        }

        Commands::User(cmd) => match cmd {
            UserCommands::Me => commands::user::me(notion, format).await,
            UserCommands::Get { id } => commands::user::get(notion, id, format).await,
            UserCommands::List => {
                commands::user::list(notion, page_size, start_cursor, format).await
            }
        },

        Commands::Page(cmd) => match cmd {
            PageCommands::Get {
                id,
                filter_properties,
            } => commands::page::get(notion, id, filter_properties, format).await,
            PageCommands::Create {
                parent,
                properties,
                children,
                database_parent,
            } => {
                commands::page::create(
                    notion,
                    parent,
                    properties,
                    children.as_deref(),
                    *database_parent,
                    format,
                )
                .await
            }
            PageCommands::Update {
                id,
                properties,
                archived,
            } => commands::page::update(notion, id, properties, *archived, format).await,
            PageCommands::Move {
                id,
                parent_type,
                to,
            } => commands::page::move_page(notion, id, parent_type, to, format).await,
            PageCommands::Property {
                page_id,
                property_id,
            } => {
                commands::page::property(
                    notion,
                    page_id,
                    property_id,
                    page_size,
                    start_cursor,
                    format,
                )
                .await
            }
        },

        Commands::Block(cmd) => match cmd {
            BlockCommands::Get { id } => commands::block::get(notion, id, format).await,
            BlockCommands::Children { id } => {
                commands::block::children(notion, id, page_size, start_cursor, format).await
            }
            BlockCommands::Append {
                id,
                children,
                after,
            } => commands::block::append(notion, id, children, after.as_deref(), format).await,
            BlockCommands::Update { id, data, archived } => {
                commands::block::update(notion, id, data, *archived, format).await
            }
            BlockCommands::Delete { id } => commands::block::delete(notion, id, format).await,
        },

        Commands::Comment(cmd) => match cmd {
            CommentCommands::List { block_id } => {
                commands::comment::list(notion, block_id, page_size, start_cursor, format).await
            }
            CommentCommands::Create { page_id, text } => {
                commands::comment::create(notion, page_id, text, format).await
            }
        },

        Commands::Db(cmd) => match cmd {
            DbCommands::Get { id } => commands::database::get(notion, id, format).await,
        },

        Commands::Ds(cmd) => match cmd {
            DsCommands::Get { id } => commands::datasource::get(notion, id, format).await,
            DsCommands::Create {
                parent,
                title,
                properties,
            } => {
                commands::datasource::create(notion, parent, title, properties.as_deref(), format)
                    .await
            }
            DsCommands::Update { id, data } => {
                commands::datasource::update(notion, id, data, format).await
            }
            DsCommands::Query { id, filter, sorts } => {
                commands::datasource::query(
                    notion,
                    id,
                    filter.as_deref(),
                    sorts.as_deref(),
                    page_size,
                    start_cursor,
                    format,
                )
                .await
            }
            DsCommands::Templates { id } => {
                commands::datasource::templates(notion, id, format).await
            }
        },
    }
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
