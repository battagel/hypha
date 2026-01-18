use anyhow::Result;
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// A match found in a file.
#[derive(Serialize)]
struct SearchMatch {
    path: String,
    title: String,
    line: usize,
    column: usize,
    content: String,
}

/// JSON output for search results grouped by file.
#[derive(Serialize)]
struct SearchResult {
    path: String,
    title: String,
    matches: Vec<SearchMatchLine>,
}

#[derive(Serialize)]
struct SearchMatchLine {
    line: usize,
    column: usize,
    content: String,
}

/// Search for a pattern in the body content of all markdown files.
pub fn run(root: &Path, pattern: &str, json: bool, case_insensitive: bool) -> Result<()> {
    // Build regex pattern
    let regex = if case_insensitive {
        Regex::new(&format!("(?i){}", regex::escape(pattern)))?
    } else {
        Regex::new(&regex::escape(pattern))?
    };

    let mut all_matches: Vec<SearchMatch> = Vec::new();

    // Walk all markdown files
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|e| e.to_str());
        if extension != Some("md") && extension != Some("markdown") {
            continue;
        }

        // Read file content
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Extract title from first # heading
        let title = content
            .lines()
            .find(|line| line.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").to_string())
            .unwrap_or_else(|| {
                path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default()
            });

        // Search each line for matches
        for (line_num, line) in content.lines().enumerate() {
            for mat in regex.find_iter(line) {
                all_matches.push(SearchMatch {
                    path: path.display().to_string(),
                    title: title.clone(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    content: line.to_string(),
                });
            }
        }
    }

    // Sort matches by path and line number for consistent output
    all_matches.sort_by(|a, b| {
        a.path.cmp(&b.path).then_with(|| a.line.cmp(&b.line))
    });

    if json {
        // Group matches by file for JSON output, preserving order
        let mut grouped: indexmap::IndexMap<String, SearchResult> =
            indexmap::IndexMap::new();

        for m in all_matches {
            let entry = grouped.entry(m.path.clone()).or_insert_with(|| SearchResult {
                path: m.path.clone(),
                title: m.title.clone(),
                matches: Vec::new(),
            });
            entry.matches.push(SearchMatchLine {
                line: m.line,
                column: m.column,
                content: m.content,
            });
        }

        let results: Vec<SearchResult> = grouped.into_values().collect();
        println!("{}", serde_json::to_string(&results)?);
    } else if all_matches.is_empty() {
        println!("No matches found for: {}", pattern);
    } else {
        // Group by file for display
        let mut current_path = String::new();
        for m in &all_matches {
            if m.path != current_path {
                if !current_path.is_empty() {
                    println!();
                }
                println!("{}:", m.path);
                println!("  {}", m.title);
                current_path = m.path.clone();
            }
            println!("  {}:{}: {}", m.line, m.column, m.content.trim());
        }
        println!();
        println!("{} match(es) found", all_matches.len());
    }

    Ok(())
}
