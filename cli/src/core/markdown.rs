//! Markdown body parsing.
//!
//! Parses markdown content to extract title, description, and links.
//! Uses pulldown-cmark under the hood.

use serde::{Deserialize, Serialize};

/// A link found in the markdown document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedLink {
    /// The link target (URL or path).
    pub target: String,
    /// 1-based line number where the link appears.
    pub line: usize,
    /// 1-based column number where the link appears.
    pub column: usize,
}

/// Parsed content extracted from a markdown document.
#[derive(Debug, Clone, Default)]
pub struct ParsedMarkdown {
    /// The document title (first H1 heading).
    pub title: Option<String>,
    /// Description (first paragraph after the title).
    pub description: Option<String>,
    /// Local file links found in the document (with line numbers).
    pub links: Vec<ParsedLink>,
}

/// Position in a document (1-based line and column).
struct Position {
    line: usize,
    column: usize,
}

/// Convert a byte offset to a 1-based line and column number.
fn offset_to_position(content: &str, offset: usize) -> Position {
    let before = &content[..offset.min(content.len())];
    let line = before.chars().filter(|&c| c == '\n').count() + 1;
    let last_newline = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let column = before[last_newline..].chars().count() + 1;
    Position { line, column }
}

/// Parse markdown content and extract title, description, and links.
pub fn parse(content: &str) -> ParsedMarkdown {
        use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

        let parser = Parser::new_ext(content, Options::empty()).into_offset_iter();

        let mut result = ParsedMarkdown::default();

        let mut in_h1 = false;
        let mut after_h1 = false;
        let mut in_first_paragraph = false;
        let mut h1_text = String::new();
        let mut paragraph_text = String::new();

        for (event, range) in parser {
            match event {
                // Track when we enter/exit an H1 heading
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H1,
                    ..
                }) => {
                    if result.title.is_none() {
                        in_h1 = true;
                    }
                }
                Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                    if in_h1 {
                        in_h1 = false;
                        after_h1 = true;
                        let trimmed = h1_text.trim().to_string();
                        if !trimmed.is_empty() {
                            result.title = Some(trimmed);
                        }
                    }
                }

                // Track first paragraph after H1 for description
                Event::Start(Tag::Paragraph) => {
                    if after_h1 && result.description.is_none() {
                        in_first_paragraph = true;
                    }
                }
                Event::End(TagEnd::Paragraph) => {
                    if in_first_paragraph {
                        in_first_paragraph = false;
                        let trimmed = paragraph_text.trim().to_string();
                        if !trimmed.is_empty() {
                            result.description = Some(trimmed);
                        }
                    }
                }

                // Collect text
                Event::Text(text) | Event::Code(text) => {
                    if in_h1 {
                        h1_text.push_str(&text);
                    } else if in_first_paragraph {
                        paragraph_text.push_str(&text);
                    }
                }

                // Collect local links with line and column numbers
                Event::Start(Tag::Link { dest_url, .. }) => {
                    let url = dest_url.as_ref();
                    if !url.starts_with("http://")
                        && !url.starts_with("https://")
                        && !url.is_empty()
                    {
                        let pos = offset_to_position(content, range.start);
                        result.links.push(ParsedLink {
                            target: url.to_string(),
                            line: pos.line,
                            column: pos.column,
                        });
                    }
                }

                _ => {}
            }
        }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_title() {
        let result = parse("# Hello World\n\nSome content.");
        assert_eq!(result.title, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_description() {
        let result = parse("# Title\n\nThis is the description.\n\nMore content.");
        assert_eq!(result.description, Some("This is the description.".to_string()));
    }

    #[test]
    fn test_parse_links() {
        let result = parse("Check [this](other.md) and [that](https://example.com).");
        assert_eq!(result.links.len(), 1);
        assert_eq!(result.links[0].target, "other.md");
        assert_eq!(result.links[0].line, 1);
        assert_eq!(result.links[0].column, 7); // "Check " = 6 chars, link starts at 7
    }

    #[test]
    fn test_parse_reference_links() {
        let result = parse("See [docs][ref].\n\n[ref]: ./docs.md");
        assert_eq!(result.links.len(), 1);
        assert_eq!(result.links[0].target, "./docs.md");
    }

    #[test]
    fn test_parse_link_line_numbers() {
        let result = parse("Line 1\n\n[link1](a.md)\n\nLine 4\n\n[link2](b.md)");
        assert_eq!(result.links.len(), 2);
        assert_eq!(result.links[0].target, "a.md");
        assert_eq!(result.links[0].line, 3);
        assert_eq!(result.links[0].column, 1);
        assert_eq!(result.links[1].target, "b.md");
        assert_eq!(result.links[1].line, 7);
        assert_eq!(result.links[1].column, 1);
    }

    #[test]
    fn test_empty_content() {
        let result = parse("");
        assert_eq!(result.title, None);
        assert_eq!(result.description, None);
        assert!(result.links.is_empty());
    }
}
