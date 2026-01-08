use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum SortOrder {
    /// Sort alphabetically by title (default)
    #[default]
    Alpha,
    /// Sort by most recently modified
    Modified,
    /// Sort by creation time
    Created,
}

#[derive(Parser)]
#[command(name = "hypha")]
#[command(about = "A CLI for managing markdown notes with rich metadata")]
#[command(version)]
pub struct Cli {
    /// Override root directory (ignores config)
    #[arg(short, long, global = true)]
    pub root: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new topic
    New {
        /// Title of the new topic
        title: String,
        /// Don't open the file in editor after creation
        #[arg(short, long)]
        no_edit: bool,
    },
    /// List all topics
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Sort order
        #[arg(long, short, value_enum, default_value = "alpha")]
        sort: SortOrder,
    },
    /// Search topics by query
    Search {
        /// Search query (supports filters like status:active priority:high)
        query: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Sort order
        #[arg(long, short, value_enum, default_value = "alpha")]
        sort: SortOrder,
    },
    /// Open a topic in your default editor
    Open {
        /// Topic title or path
        topic: String,
    },
    /// Delete a topic
    Delete {
        /// Topic title or path
        topic: String,
    },
    /// Lint topics for issues (missing title, empty content)
    Lint {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show setup info, topic count, and field usage
    Info {
        /// Show all topics
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show topics that link to this topic
    Backlinks {
        /// Topic title or filename
        topic: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Rename a topic and update all links
    Rename {
        /// Current topic title or filename
        from: String,
        /// New title
        to: String,
    },
}
