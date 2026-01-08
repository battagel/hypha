use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

use crate::cli::SortOrder;
use crate::core::index;

/// Open a topic in the default editor.
pub fn run(root: &Path, query: &str) -> Result<()> {
    // First try exact path match
    let direct_path = root.join(query);
    if direct_path.exists() {
        return open_in_editor(&direct_path);
    }

    // Try with .md extension
    let with_ext = root.join(format!("{}.md", query));
    if with_ext.exists() {
        return open_in_editor(&with_ext);
    }

    // Search by title
    let topics = index::search(root, query, SortOrder::Alpha)?;

    match topics.len() {
        0 => bail!("No topic found matching: {}", query),
        1 => open_in_editor(&topics[0].path),
        _ => {
            println!("Multiple matches found:");
            for (i, topic) in topics.iter().enumerate() {
                println!("  {}: {}", i + 1, topic.title);
            }
            bail!("Please be more specific");
        }
    }
}

/// Open a file in the user's editor.
pub fn open_in_editor(path: &Path) -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    Command::new(&editor).arg(path).status().context(format!(
        "Failed to open {} with {}",
        path.display(),
        editor
    ))?;

    Ok(())
}
