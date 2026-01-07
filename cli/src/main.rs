use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod constants;
mod core;
mod utils;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Resolve root directory (CLI flag overrides config)
    let root = match &cli.root {
        Some(path) => {
            // When --root is provided, skip config entirely
            path.clone()
        },
        None => {
            // Ensure config exists (interactive setup if needed)
            let config = config::ensure_configured()?;
            config.require_root_dir()?
        }
    };
    let root_override = cli.root.is_some();

    match cli.command {
        Commands::New { title, no_edit } => commands::new::run(&root, &title, !no_edit),
        Commands::List { json, sort } => commands::list::run(&root, json, sort),
        Commands::Search { query, json, sort } => commands::search::run(&root, &query, json, sort),
        Commands::Open { topic } => commands::open::run(&root, &topic),
        Commands::Delete { topic } => commands::delete::run(&root, &topic),
        Commands::Lint { json } => commands::lint::run(&root, json),
        Commands::Info { verbose } => commands::info::run(&root, verbose, root_override),
        Commands::Backlinks { topic, json } => commands::backlinks::run(&root, &topic, json),
        Commands::Rename { from, to } => commands::rename::run(&root, &from, &to),
    }
}
