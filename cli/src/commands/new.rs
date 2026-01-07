use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

use crate::core::template;
use crate::utils::slugify;

/// Create a new topic.
pub fn run(root: &Path, title: &str, open_after: bool) -> Result<()> {
    let slug = slugify(title);
    let filename = format!("{}.md", slug);
    let filepath = root.join(&filename);

    if filepath.exists() {
        bail!("Topic already exists: {}", filepath.display());
    }

    let tmpl = template::load(root);
    let content = template::render(&tmpl, title);

    fs::write(&filepath, content).context("Failed to write topic file")?;
    println!("Created: {}", filepath.display());

    if open_after {
        crate::commands::open::open_in_editor(&filepath)?;
    }

    Ok(())
}
