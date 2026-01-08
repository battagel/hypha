use anyhow::Result;
use std::path::Path;

use crate::config::Config;
use crate::core::index;

/// Show setup info, topic count, and field usage.
pub fn run(root: &Path, verbose: bool, root_override: bool) -> Result<()> {
    let stats = index::stats(root)?;

    if !root_override {
        let config_path = Config::config_path()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        println!("Config: {}", config_path);
    }
    println!("Root:   {}", root.display());
    println!();
    println!("Topics: {}", stats.total);

    if !stats.fields.is_empty() {
        let mut fields: Vec<_> = stats.fields.iter().collect();
        fields.sort_by(|a, b| b.1.cmp(a.1));
        let field_strs: Vec<_> = fields
            .iter()
            .map(|(k, v)| format!("{} ({})", k, v))
            .collect();
        println!("Fields: {}", field_strs.join(", "));
    }

    if verbose && !stats.field_values.is_empty() {
        println!();
        println!("Field Details:");
        println!("{}", "=".repeat(40));

        // Sort fields by occurrence count (descending)
        let mut fields: Vec<_> = stats.fields.iter().collect();
        fields.sort_by(|a, b| b.1.cmp(a.1));

        for (field_name, field_count) in fields {
            println!();
            println!("{} ({} topics):", field_name, field_count);

            if let Some(values) = stats.field_values.get(field_name) {
                let mut sorted_values: Vec<_> = values.iter().collect();
                sorted_values.sort_by(|a, b| b.1.cmp(a.1));

                for (value, count) in sorted_values {
                    println!("  {} ({})", value, count);
                }
            }
        }
    }

    Ok(())
}
