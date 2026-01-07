use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

use crate::cli::SortOrder;
use crate::core::index;

/// Delete a topic.
pub fn run(root: &Path, query: &str) -> Result<()> {
    // First try exact path match
    let direct_path = root.join(query);
    if direct_path.exists() {
        fs::remove_file(&direct_path).context("Failed to delete topic")?;
        println!("Deleted: {}", direct_path.display());
        return Ok(());
    }

    // Try with .md extension
    let with_ext = root.join(format!("{}.md", query));
    if with_ext.exists() {
        fs::remove_file(&with_ext).context("Failed to delete topic")?;
        println!("Deleted: {}", with_ext.display());
        return Ok(());
    }

    // Search by title
    let topics = index::search(root, query, SortOrder::Alpha)?;

    match topics.len() {
        0 => bail!("No topic found matching: {}", query),
        1 => {
            fs::remove_file(&topics[0].path).context("Failed to delete topic")?;
            println!("Deleted: {}", topics[0].path.display());
            Ok(())
        }
        _ => {
            println!("Multiple matches found:");
            for (i, topic) in topics.iter().enumerate() {
                println!("  {}: {}", i + 1, topic.title);
            }
            bail!("Please be more specific");
        }
    }
}
