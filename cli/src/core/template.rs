use std::fs;
use std::path::Path;

use crate::constants::{DEFAULT_TEMPLATE, TEMPLATE_FILE};

/// Load template from .template.md in root, or use default.
pub fn load(root: &Path) -> String {
    let template_path = root.join(TEMPLATE_FILE);
    if template_path.exists() {
        fs::read_to_string(&template_path).unwrap_or_else(|_| DEFAULT_TEMPLATE.to_string())
    } else {
        DEFAULT_TEMPLATE.to_string()
    }
}

/// Render a template with the given title.
pub fn render(template: &str, title: &str) -> String {
    template.replace("{title}", title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_replaces_title() {
        let template = "---\ntitle: {title}\n---\n";
        let result = render(template, "My Topic");
        assert!(result.contains("title: My Topic"));
    }
}
