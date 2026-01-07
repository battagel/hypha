use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

use crate::cli::SortOrder;
use crate::core::index;
use crate::utils::slugify;

/// Rename a topic and update all links pointing to it.
pub fn run(root: &Path, from: &str, to: &str) -> Result<()> {
    let topics = index::list(root, SortOrder::Alpha)?;
    
    // Find the source topic
    let source = topics.iter().find(|t| {
        t.title.eq_ignore_ascii_case(from) || 
        t.path.file_stem().map(|s| s.to_string_lossy().eq_ignore_ascii_case(from)).unwrap_or(false)
    });

    let source = match source {
        Some(t) => t,
        None => bail!("Topic not found: {}", from),
    };

    let old_path = source.path.clone();
    let old_filename = old_path.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let old_stem = old_path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Generate new filename
    let new_stem = slugify(to);
    let new_filename = format!("{}.md", new_stem);
    let new_path = old_path.parent().unwrap().join(&new_filename);

    if new_path.exists() {
        bail!("A topic already exists at: {}", new_path.display());
    }

    // Read the source file and update the heading
    let content = fs::read_to_string(&old_path)?;
    let new_content = update_heading(&content, to);

    // Write updated content to new path
    fs::write(&new_path, new_content)?;

    // Delete old file
    fs::remove_file(&old_path)?;

    println!("Renamed: {} -> {}", old_path.display(), new_path.display());

    // Update links in other topics
    let mut updated_count = 0;
    for t in &topics {
        if t.path == old_path {
            continue;
        }

        let content = fs::read_to_string(&t.path)?;
        let updated = update_links(&content, &old_filename, &old_stem, &new_filename, &new_stem);
        
        if updated != content {
            fs::write(&t.path, updated)?;
            updated_count += 1;
            println!("  Updated links in: {}", t.path.display());
        }
    }

    if updated_count > 0 {
        println!();
        println!("Updated {} file(s) with new links", updated_count);
    }

    Ok(())
}

/// Update the first heading in the content to the new title.
fn update_heading(content: &str, new_title: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut heading_updated = false;
    
    for line in lines {
        if !heading_updated && line.starts_with("# ") {
            result.push(format!("# {}", new_title));
            heading_updated = true;
        } else {
            result.push(line.to_string());
        }
    }
    
    result.join("\n")
}

/// Update links from old filename to new filename.
fn update_links(content: &str, old_filename: &str, old_stem: &str, new_filename: &str, new_stem: &str) -> String {
    let mut result = content.to_string();
    
    // Update markdown links: [text](old-file.md) -> [text](new-file.md)
    result = result.replace(&format!("]({})", old_filename), &format!("]({})", new_filename));
    result = result.replace(&format!("]({})", old_stem), &format!("]({})", new_stem));
    
    // Update wiki links: [[old-file]] -> [[new-file]]
    result = result.replace(&format!("[[{}]]", old_filename.trim_end_matches(".md")), &format!("[[{}]]", new_stem));
    result = result.replace(&format!("[[{}]]", old_stem), &format!("[[{}]]", new_stem));
    
    result
}
