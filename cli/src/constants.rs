/// Project name - used for config file, default directory, CLI branding, etc.
pub const PROJECT_NAME: &str = "hypha";

/// Template file name stored in the notes directory
pub const TEMPLATE_FILE: &str = ".template.md";

/// Default template for new topics. Placeholder: {title}
pub const DEFAULT_TEMPLATE: &str = r#"---
---

# {title}

Brief description of this topic.
"#;
