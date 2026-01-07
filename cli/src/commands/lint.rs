use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

use crate::cli::SortOrder;
use crate::core::index;
use crate::core::markdown::ParsedLink;
use crate::core::topic::{Topic, TopicWarning};

/// A structured warning with message and optional position.
#[derive(Serialize)]
struct WarningInfo {
    message: String,
    /// 1-based line number (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    /// 1-based column number (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<usize>,
}

impl From<&TopicWarning> for WarningInfo {
    fn from(warning: &TopicWarning) -> Self {
        match warning {
            TopicWarning::MissingTitle => WarningInfo {
                message: "Missing title (no # heading)".to_string(),
                line: None,
                column: None,
            },
            TopicWarning::EmptyContent => WarningInfo {
                message: "Empty content".to_string(),
                line: None,
                column: None,
            },
            TopicWarning::BrokenLink { target, line, column } => WarningInfo {
                message: format!("Broken link: {}", target),
                line: Some(*line),
                column: Some(*column),
            },
        }
    }
}

/// JSON output for lint results.
#[derive(Serialize)]
struct LintResult {
    title: String,
    path: String,
    warnings: Vec<WarningInfo>,
    frontmatter: HashMap<String, serde_yaml::Value>,
}

impl From<&Topic> for LintResult {
    fn from(topic: &Topic) -> Self {
        LintResult {
            title: topic.title.clone(),
            path: topic.path.display().to_string(),
            warnings: topic.warnings.iter().map(WarningInfo::from).collect(),
            frontmatter: topic.metadata.clone(),
        }
    }
}

/// Extract the file path from a link target.
/// Returns None if the link should be skipped (anchor-only or empty).
fn extract_link_path(target: &str) -> Option<&str> {
    // Skip anchor-only links
    if target.starts_with('#') {
        return None;
    }

    // Remove anchor from link (e.g., "file.md#section" -> "file.md")
    let link_path = target.split('#').next().unwrap_or(target);

    // Skip empty links
    if link_path.is_empty() {
        return None;
    }

    Some(link_path)
}

/// Check if a link target exists relative to the topic's directory.
fn check_link(topic_path: &Path, link: &ParsedLink) -> Option<TopicWarning> {
    let link_path = extract_link_path(&link.target)?;

    // Get the directory containing the topic
    let topic_dir = topic_path.parent()?;

    // Resolve the link relative to the topic's directory
    let target = topic_dir.join(link_path);

    // Check if target exists
    if !target.exists() {
        Some(TopicWarning::BrokenLink {
            target: link.target.clone(),
            line: link.line,
            column: link.column,
        })
    } else {
        None
    }
}

/// Lint all topics for issues (missing title, empty content, broken links).
/// Returns only topics with warnings.
pub fn run(root: &Path, json: bool) -> Result<()> {
    let mut topics = index::build(root, SortOrder::Alpha)?;

    // Check for broken links in each topic
    for topic in &mut topics {
        for link in &topic.links.clone() {
            if let Some(warning) = check_link(&topic.path, link) {
                topic.warnings.push(warning);
            }
        }
    }

    // Filter to only topics with warnings
    let issues: Vec<&Topic> = topics.iter().filter(|t| !t.warnings.is_empty()).collect();

    if json {
        let results: Vec<LintResult> = issues.iter().map(|t| LintResult::from(*t)).collect();
        println!("{}", serde_json::to_string(&results)?);
        if !issues.is_empty() {
            std::process::exit(1);
        }
        return Ok(());
    }

    if issues.is_empty() {
        println!("No issues found.");
        return Ok(());
    }

    println!("Found issues in {} file(s):\n", issues.len());
    for topic in &issues {
        println!("{}:", topic.path.display());
        for warning in &topic.warnings {
            println!("  - {}", warning);
        }
        println!();
    }

    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn extract_link_path_basic() {
        assert_eq!(extract_link_path("file.md"), Some("file.md"));
        assert_eq!(extract_link_path("./docs/file.md"), Some("./docs/file.md"));
    }

    #[test]
    fn extract_link_path_with_anchor() {
        assert_eq!(extract_link_path("file.md#section"), Some("file.md"));
        assert_eq!(extract_link_path("./docs/file.md#heading"), Some("./docs/file.md"));
    }

    #[test]
    fn extract_link_path_anchor_only() {
        assert_eq!(extract_link_path("#section"), None);
        assert_eq!(extract_link_path("#"), None);
    }

    #[test]
    fn extract_link_path_empty() {
        assert_eq!(extract_link_path(""), None);
    }

    #[test]
    fn check_link_existing_file() {
        let temp = TempDir::new().unwrap();
        let topic_path = temp.path().join("topic.md");
        let target_path = temp.path().join("target.md");

        std::fs::write(&topic_path, "# Topic").unwrap();
        std::fs::write(&target_path, "# Target").unwrap();

        let link = ParsedLink {
            target: "target.md".to_string(),
            line: 1,
            column: 1,
        };

        assert!(check_link(&topic_path, &link).is_none());
    }

    #[test]
    fn check_link_missing_file() {
        let temp = TempDir::new().unwrap();
        let topic_path = temp.path().join("topic.md");
        std::fs::write(&topic_path, "# Topic").unwrap();

        let link = ParsedLink {
            target: "missing.md".to_string(),
            line: 5,
            column: 3,
        };

        let result = check_link(&topic_path, &link);
        assert!(result.is_some());

        if let Some(TopicWarning::BrokenLink { target, line, column }) = result {
            assert_eq!(target, "missing.md");
            assert_eq!(line, 5);
            assert_eq!(column, 3);
        } else {
            panic!("Expected BrokenLink warning");
        }
    }

    #[test]
    fn check_link_skips_anchors() {
        let temp = TempDir::new().unwrap();
        let topic_path = temp.path().join("topic.md");
        std::fs::write(&topic_path, "# Topic").unwrap();

        let link = ParsedLink {
            target: "#section".to_string(),
            line: 1,
            column: 1,
        };

        assert!(check_link(&topic_path, &link).is_none());
    }

    #[test]
    fn check_link_with_anchor_to_existing_file() {
        let temp = TempDir::new().unwrap();
        let topic_path = temp.path().join("topic.md");
        let target_path = temp.path().join("target.md");

        std::fs::write(&topic_path, "# Topic").unwrap();
        std::fs::write(&target_path, "# Target").unwrap();

        let link = ParsedLink {
            target: "target.md#section".to_string(),
            line: 1,
            column: 1,
        };

        // Should pass because file exists (anchor not validated)
        assert!(check_link(&topic_path, &link).is_none());
    }
}
