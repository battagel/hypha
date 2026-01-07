use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::frontmatter;
use super::markdown::{self, ParsedLink};
use super::query::{FieldValue, Queryable};

/// Validation warning for a topic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopicWarning {
    MissingTitle,
    EmptyContent,
    /// Broken link with target, line, and column number.
    BrokenLink { target: String, line: usize, column: usize },
}

impl std::fmt::Display for TopicWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopicWarning::MissingTitle => write!(f, "Missing title (no # heading)"),
            TopicWarning::EmptyContent => write!(f, "Empty content"),
            TopicWarning::BrokenLink { target, line, column } => {
                write!(f, "Broken link: {} (line {}, col {})", target, line, column)
            }
        }
    }
}

/// A parsed topic with all its metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub path: PathBuf,
    pub title: String,
    /// Short description (first line after the heading, typically a blockquote)
    pub description: Option<String>,
    /// Links found in the document (with line numbers relative to file start).
    pub links: Vec<ParsedLink>,
    /// All frontmatter fields for queries
    #[serde(default)]
    pub metadata: HashMap<String, serde_yaml::Value>,
    /// Validation warnings
    #[serde(default)]
    pub warnings: Vec<TopicWarning>,
}

impl Topic {
    /// Parse a topic from a markdown file.
    pub fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read file")?;
        Ok(Self::from_content(path, &content))
    }

    /// Parse a topic from content string (for testing).
    pub fn from_content(path: &Path, content: &str) -> Self {
        let fm = frontmatter::parse(content);

        let mut warnings = Vec::new();

        // Check for empty content
        let body_trimmed = fm.body.trim();
        if body_trimmed.is_empty() {
            warnings.push(TopicWarning::EmptyContent);
        }

        // Parse markdown body
        let parsed = markdown::parse(&fm.body);

        // Offset link line numbers by frontmatter lines
        let links: Vec<ParsedLink> = parsed
            .links
            .into_iter()
            .map(|mut link| {
                link.line += fm.frontmatter_lines;
                link
            })
            .collect();

        // Check for missing title
        let title = if let Some(t) = parsed.title {
            t
        } else {
            warnings.push(TopicWarning::MissingTitle);
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        };

        Topic {
            path: path.to_path_buf(),
            title,
            description: parsed.description,
            links,
            metadata: fm.metadata,
            warnings,
        }
    }

    /// Get tags from metadata (convenience method).
    pub fn tags(&self) -> Vec<String> {
        frontmatter::parse_string_array(self.metadata.get("tags"))
    }

    /// Format for display.
    pub fn display(&self) -> TopicDisplay<'_> {
        TopicDisplay(self)
    }
}

impl Queryable for Topic {
    fn get_field(&self, key: &str) -> Option<FieldValue> {
        // Handle tag/tags alias
        let key = if key == "tag" { "tags" } else { key };

        // Title is always available
        if key == "title" {
            return Some(FieldValue::Single(self.title.clone()));
        }

        // Look up in metadata
        self.metadata.get(key).and_then(yaml_to_field_value)
    }

    fn searchable_text(&self) -> String {
        self.title.clone()
    }
}

/// Convert a YAML value to a FieldValue for querying.
fn yaml_to_field_value(value: &serde_yaml::Value) -> Option<FieldValue> {
    match value {
        serde_yaml::Value::String(s) => Some(FieldValue::Single(s.clone())),
        serde_yaml::Value::Number(n) => Some(FieldValue::Single(n.to_string())),
        serde_yaml::Value::Bool(b) => Some(FieldValue::Single(b.to_string())),
        serde_yaml::Value::Sequence(seq) => {
            let strings: Vec<String> = seq
                .iter()
                .filter_map(|v| match v {
                    serde_yaml::Value::String(s) => Some(s.clone()),
                    serde_yaml::Value::Number(n) => Some(n.to_string()),
                    _ => None,
                })
                .collect();
            Some(FieldValue::Multiple(strings))
        }
        _ => None,
    }
}

/// Display wrapper for Topic.
pub struct TopicDisplay<'a>(&'a Topic);

impl std::fmt::Display for TopicDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags = self.0.tags();
        if tags.is_empty() {
            write!(f, "{}", self.0.title)
        } else {
            write!(f, "{} [{}]", self.0.title, tags.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_topic() {
        let content = "---\ntags:\n  - test\n---\n\n# My Title\n\nSome content.";
        let topic = Topic::from_content(Path::new("test.md"), content);

        assert_eq!(topic.title, "My Title");
        assert_eq!(topic.tags(), vec!["test"]);
        assert!(topic.warnings.is_empty());
    }

    #[test]
    fn parse_missing_title() {
        let content = "Just some content without a heading.";
        let topic = Topic::from_content(Path::new("fallback.md"), content);

        assert_eq!(topic.title, "fallback"); // Falls back to filename stem
        assert!(topic.warnings.contains(&TopicWarning::MissingTitle));
    }

    #[test]
    fn parse_empty_content() {
        let content = "---\ntitle: Test\n---\n";
        let topic = Topic::from_content(Path::new("empty.md"), content);

        assert!(topic.warnings.contains(&TopicWarning::EmptyContent));
    }

    #[test]
    fn link_line_offset_with_frontmatter() {
        // 4 lines of frontmatter, link on line 3 of body = line 7 of file
        let content = "---\ntags:\n  - a\n---\n\n# Title\n\n[link](other.md)";
        let topic = Topic::from_content(Path::new("test.md"), content);

        assert_eq!(topic.links.len(), 1);
        assert_eq!(topic.links[0].target, "other.md");
        assert_eq!(topic.links[0].line, 8); // 5 frontmatter lines + 3 body lines
    }

    #[test]
    fn parse_with_description() {
        let content = "# Title\n\nThis is the description.\n\nMore content.";
        let topic = Topic::from_content(Path::new("test.md"), content);

        assert_eq!(topic.title, "Title");
        assert_eq!(topic.description, Some("This is the description.".to_string()));
    }

    #[test]
    fn tags_convenience_method() {
        let content = "---\ntags:\n  - one\n  - two\n---\n\n# Title";
        let topic = Topic::from_content(Path::new("test.md"), content);

        assert_eq!(topic.tags(), vec!["one", "two"]);
    }

    #[test]
    fn tags_empty_when_missing() {
        let content = "# Title\n\nNo frontmatter.";
        let topic = Topic::from_content(Path::new("test.md"), content);

        assert!(topic.tags().is_empty());
    }
}
