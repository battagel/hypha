use std::collections::HashMap;

/// Result of parsing frontmatter.
pub struct FrontmatterResult {
    /// Parsed frontmatter key-value pairs.
    pub metadata: HashMap<String, serde_yaml::Value>,
    /// The body content (after frontmatter, with leading newlines trimmed).
    pub body: String,
    /// Number of newlines before body content starts.
    /// Add this to body-relative line numbers to get file-absolute line numbers.
    pub frontmatter_lines: usize,
}

/// Parse YAML frontmatter from markdown content.
///
/// Returns frontmatter metadata, body content, and line count.
/// If no valid frontmatter is found, returns empty HashMap and full content.
pub fn parse(content: &str) -> FrontmatterResult {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return FrontmatterResult {
            metadata: HashMap::new(),
            body: content.to_string(),
            frontmatter_lines: 0,
        };
    }

    let after_open = &content[3..];
    if let Some(close_idx) = after_open.find("\n---") {
        let yaml_str = &after_open[..close_idx];
        let body_with_leading = &after_open[close_idx + 4..];

        // Count newlines in the frontmatter section (up to and including closing ---)
        let frontmatter_end_offset = 3 + close_idx + 4; // "---" + yaml + "\n---"
        let mut frontmatter_lines = content[..frontmatter_end_offset]
            .chars()
            .filter(|&c| c == '\n')
            .count();

        // Count and trim leading newlines from body (these are still "before" content)
        let leading_newlines = body_with_leading
            .chars()
            .take_while(|&c| c == '\n')
            .count();
        frontmatter_lines += leading_newlines;
        let body = &body_with_leading[leading_newlines..];

        match serde_yaml::from_str(yaml_str) {
            Ok(map) => FrontmatterResult {
                metadata: map,
                body: body.to_string(),
                frontmatter_lines,
            },
            Err(_) => FrontmatterResult {
                metadata: HashMap::new(),
                body: content.to_string(),
                frontmatter_lines: 0,
            },
        }
    } else {
        FrontmatterResult {
            metadata: HashMap::new(),
            body: content.to_string(),
            frontmatter_lines: 0,
        }
    }
}

/// Parse a YAML value that might be an array of strings.
///
/// Handles both array syntax and comma-separated string syntax.
pub fn parse_string_array(value: Option<&serde_yaml::Value>) -> Vec<String> {
    match value {
        Some(serde_yaml::Value::Sequence(seq)) => seq
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        Some(serde_yaml::Value::String(s)) => s.split(',').map(|s| s.trim().to_string()).collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_frontmatter() {
        let content = "---\ntitle: Test\ntags:\n  - a\n  - b\n---\n\nBody content";
        let result = parse(content);
        assert_eq!(
            result.metadata.get("title").and_then(|v| v.as_str()),
            Some("Test")
        );
        assert!(result.body.contains("Body content"));
        // 1 (---) + 5 (yaml lines) + 1 (---) = 7, but yaml_lines counts the lines between ---
        // Actually: opening ---, then 5 lines of yaml, then closing ---
        assert_eq!(result.frontmatter_lines, 7);
    }

    #[test]
    fn parse_no_frontmatter() {
        let content = "Just some content";
        let result = parse(content);
        assert!(result.metadata.is_empty());
        assert_eq!(result.body, "Just some content");
        assert_eq!(result.frontmatter_lines, 0);
    }

    #[test]
    fn parse_string_array_sequence() {
        let val = serde_yaml::Value::Sequence(vec![
            serde_yaml::Value::String("a".to_string()),
            serde_yaml::Value::String("b".to_string()),
        ]);
        let result = parse_string_array(Some(&val));
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn parse_string_array_csv() {
        let val = serde_yaml::Value::String("a, b, c".to_string());
        let result = parse_string_array(Some(&val));
        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
