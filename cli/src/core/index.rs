use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use super::query::{self, Query};
use super::topic::Topic;
use crate::cli::SortOrder;
use crate::constants::TEMPLATE_FILE;

/// Statistics about the index.
#[derive(Debug, Default)]
pub struct IndexStats {
    pub total: usize,
    /// Count of topics using each field (field_name -> count)
    pub fields: HashMap<String, usize>,
    /// Values for each field with their counts (field_name -> (value -> count))
    pub field_values: HashMap<String, HashMap<String, usize>>,
}

/// Sort topics according to the specified order.
fn sort_topics(topics: &mut [Topic], sort: SortOrder) {
    match sort {
        SortOrder::Alpha => {
            topics.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        }
        SortOrder::Modified => {
            topics.sort_by(|a, b| {
                let a_time = fs::metadata(&a.path)
                    .and_then(|m| m.modified())
                    .ok();
                let b_time = fs::metadata(&b.path)
                    .and_then(|m| m.modified())
                    .ok();
                // Most recent first
                b_time.cmp(&a_time)
            });
        }
        SortOrder::Created => {
            topics.sort_by(|a, b| {
                let a_time = fs::metadata(&a.path)
                    .and_then(|m| m.created())
                    .ok();
                let b_time = fs::metadata(&b.path)
                    .and_then(|m| m.created())
                    .ok();
                // Most recent first
                b_time.cmp(&a_time)
            });
        }
    }
}

/// Build an index of all topics in the root directory.
pub fn build(root: &Path, sort: SortOrder) -> Result<Vec<Topic>> {
    let mut topics = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip template file
        if path
            .file_name()
            .map(|n| n == TEMPLATE_FILE)
            .unwrap_or(false)
        {
            continue;
        }

        // Only process .md files
        if path.extension().map(|e| e == "md").unwrap_or(false) {
            if let Ok(topic) = Topic::from_path(path) {
                topics.push(topic);
            }
        }
    }

    sort_topics(&mut topics, sort);

    Ok(topics)
}

/// List all topics.
pub fn list(root: &Path, sort: SortOrder) -> Result<Vec<Topic>> {
    build(root, sort)
}

/// Search topics by query string.
pub fn search(root: &Path, query_str: &str, sort: SortOrder) -> Result<Vec<Topic>> {
    let topics = build(root, sort)?;
    let query = Query::parse(query_str);

    if query.is_empty() {
        return Ok(topics);
    }

    let mut results: Vec<Topic> = topics
        .into_iter()
        .filter(|t| query::matches(t, &query))
        .collect();
    
    // Re-sort filtered results
    sort_topics(&mut results, sort);
    
    Ok(results)
}

/// Get statistics about the index.
pub fn stats(root: &Path) -> Result<IndexStats> {
    let topics = build(root, SortOrder::Alpha)?;
    let mut fields: HashMap<String, usize> = HashMap::new();
    let mut field_values: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for topic in &topics {
        for (key, value) in &topic.metadata {
            // Skip title since every topic has it
            if key == "title" {
                continue;
            }
            // Only count if field has a meaningful value
            let has_value = match value {
                serde_yaml::Value::Null => false,
                serde_yaml::Value::String(s) => !s.is_empty(),
                serde_yaml::Value::Sequence(seq) => !seq.is_empty(),
                _ => true,
            };
            if has_value {
                *fields.entry(key.clone()).or_insert(0) += 1;
                
                // Track individual values
                let values_map = field_values.entry(key.clone()).or_default();
                match value {
                    serde_yaml::Value::String(s) => {
                        *values_map.entry(s.clone()).or_insert(0) += 1;
                    }
                    serde_yaml::Value::Sequence(seq) => {
                        for item in seq {
                            if let serde_yaml::Value::String(s) = item {
                                *values_map.entry(s.clone()).or_insert(0) += 1;
                            }
                        }
                    }
                    serde_yaml::Value::Bool(b) => {
                        *values_map.entry(b.to_string()).or_insert(0) += 1;
                    }
                    serde_yaml::Value::Number(n) => {
                        *values_map.entry(n.to_string()).or_insert(0) += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(IndexStats {
        total: topics.len(),
        fields,
        field_values,
    })
}
