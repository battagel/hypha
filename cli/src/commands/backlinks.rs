use anyhow::{bail, Result};
use serde::Serialize;
use std::path::Path;

use crate::cli::SortOrder;
use crate::core::index;

#[derive(Serialize)]
struct BacklinkJson {
    title: String,
    path: String,
}

/// Find topics that link to the given topic.
pub fn run(root: &Path, topic: &str, json: bool) -> Result<()> {
    let topics = index::list(root, SortOrder::Alpha)?;
    
    // Find the target topic
    let target = topics.iter().find(|t| {
        t.title.eq_ignore_ascii_case(topic) || 
        t.path.file_stem().map(|s| s.to_string_lossy().eq_ignore_ascii_case(topic)).unwrap_or(false)
    });

    let target = match target {
        Some(t) => t,
        None => bail!("Topic not found: {}", topic),
    };

    let target_filename = target.path.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let target_stem = target.path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Find topics that link to this target
    let mut backlinks: Vec<&crate::core::topic::Topic> = Vec::new();

    for t in &topics {
        if t.path == target.path {
            continue; // Skip self
        }

        for link in &t.links {
            // Extract the filename or stem from the link target
            let link_path = std::path::Path::new(&link.target);
            let link_filename = link_path
                .file_name()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let link_stem = link_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            // Exact match on filename or stem
            if link_filename == target_filename.to_lowercase()
                || link_stem == target_stem.to_lowercase()
            {
                backlinks.push(t);
                break;
            }
        }
    }

    if json {
        let output: Vec<BacklinkJson> = backlinks.iter().map(|t| BacklinkJson {
            title: t.title.clone(),
            path: t.path.display().to_string(),
        }).collect();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if backlinks.is_empty() {
        println!("No backlinks found for: {}", target.title);
    } else {
        println!("Topics linking to \"{}\":", target.title);
        println!();
        for t in &backlinks {
            println!("  {} ({})", t.title, t.path.display());
        }
        println!();
        println!("{} backlink(s) found", backlinks.len());
    }

    Ok(())
}
