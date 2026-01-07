/// Convert a string into a URL/filename-safe slug.
///
/// - Lowercases the input
/// - Replaces non-alphanumeric characters with hyphens
/// - Collapses multiple hyphens into one
/// - Trims leading/trailing hyphens
///
/// # Example
/// ```
/// assert_eq!(slugify("My Great Topic"), "my-great-topic");
/// assert_eq!(slugify("Q1 2026 Planning!"), "q1-2026-planning");
/// ```
pub fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("My Great Topic"), "my-great-topic");
    }

    #[test]
    fn slugify_special_chars() {
        assert_eq!(slugify("Q1 2026 Planning!"), "q1-2026-planning");
    }

    #[test]
    fn slugify_colons() {
        assert_eq!(slugify("Fix: crash on startup"), "fix-crash-on-startup");
    }

    #[test]
    fn slugify_multiple_spaces() {
        assert_eq!(slugify("too   many   spaces"), "too-many-spaces");
    }
}
