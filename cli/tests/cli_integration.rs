//! Integration tests for the Hypha CLI.
//!
//! These tests use the fixtures in `tests/fixtures/` to verify CLI behavior.

use std::path::PathBuf;
use std::process::Command;

/// Get the path to the test fixtures directory.
fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Run the hypha CLI with the given arguments.
fn run_hypha(args: &[&str]) -> (String, String, bool) {
    let binary = env!("CARGO_BIN_EXE_hypha");
    let output = Command::new(binary)
        .args(args)
        .output()
        .expect("Failed to execute hypha");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}

mod backlinks {
    use super::*;

    #[test]
    fn finds_backlinks_to_topic_b() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "topic-b"
        ]);

        assert!(success);
        assert!(stdout.contains("Topic A"), "Should find Topic A as backlink");
        assert!(stdout.contains("No Frontmatter"), "Should find No Frontmatter as backlink");
        assert!(stdout.contains("2 backlink(s) found"), "Should find 2 backlinks");
    }

    #[test]
    fn finds_backlinks_to_topic_c() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "topic-c"
        ]);

        assert!(success);
        assert!(stdout.contains("Topic A"), "Should find Topic A as backlink");
        assert!(stdout.contains("1 backlink(s) found"), "Should find 1 backlink");
    }

    #[test]
    fn orphan_has_no_backlinks() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "orphan"
        ]);

        assert!(success);
        assert!(stdout.contains("No backlinks found"), "Orphan should have no backlinks");
    }

    #[test]
    fn backlinks_json_output() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "topic-b", "--json"
        ]);

        assert!(success);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 2, "Should have 2 backlinks in JSON");
    }

    #[test]
    fn backlinks_not_found_topic() {
        let fixtures = fixtures_path();
        let (_, stderr, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "nonexistent"
        ]);

        assert!(!success);
        assert!(stderr.contains("not found") || stderr.contains("Topic not found"));
    }
}

mod lint {
    use super::*;

    #[test]
    fn finds_broken_links() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "lint"
        ]);

        // Lint exits with non-zero when issues found
        assert!(!success);
        assert!(stdout.contains("Broken link: missing.md"));
        assert!(stdout.contains("Broken link: ./does-not-exist.md"));
    }

    #[test]
    fn lint_json_output() {
        let fixtures = fixtures_path();
        let (stdout, _, _) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "lint", "--json"
        ]);

        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        assert!(parsed.is_array());
        
        // Find the broken-links.md entry
        let arr = parsed.as_array().unwrap();
        let broken = arr.iter()
            .find(|v| v["path"].as_str().unwrap_or("").contains("broken-links.md"))
            .expect("Should find broken-links.md in results");
        
        let warnings = broken["warnings"].as_array().unwrap();
        assert_eq!(warnings.len(), 2, "broken-links.md should have 2 broken link warnings");
    }

    #[test]
    fn lint_includes_line_and_column() {
        let fixtures = fixtures_path();
        let (stdout, _, _) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "lint", "--json"
        ]);

        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        let arr = parsed.as_array().unwrap();
        let broken = arr.iter()
            .find(|v| v["path"].as_str().unwrap_or("").contains("broken-links.md"))
            .expect("Should find broken-links.md");
        
        let warning = &broken["warnings"][0];
        assert!(warning["line"].is_number(), "Warning should have line number");
        assert!(warning["column"].is_number(), "Warning should have column number");
    }
}

mod list {
    use super::*;

    #[test]
    fn lists_all_topics() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list"
        ]);

        assert!(success);
        assert!(stdout.contains("Topic A"));
        assert!(stdout.contains("Topic B"));
        assert!(stdout.contains("Topic C"));
        assert!(stdout.contains("Orphan Topic"));
        assert!(stdout.contains("Broken Links"));
        assert!(stdout.contains("No Frontmatter"));
    }

    #[test]
    fn list_json_output() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list", "--json"
        ]);

        assert!(success);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 6, "Should list 6 topics");
    }

    #[test]
    fn list_sort_alpha() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list", "--sort", "alpha", "--json"
        ]);

        assert!(success);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        let arr = parsed.as_array().unwrap();
        let titles: Vec<&str> = arr.iter()
            .map(|v| v["title"].as_str().unwrap())
            .collect();
        
        // Check alphabetical order
        let mut sorted = titles.clone();
        sorted.sort();
        assert_eq!(titles, sorted, "Topics should be sorted alphabetically");
    }

    #[test]
    fn list_sort_modified() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list", "--sort", "modified", "--json"
        ]);

        assert!(success);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        // Just verify it runs successfully and returns valid JSON
        assert!(parsed.is_array());
    }

    #[test]
    fn list_sort_created() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list", "--sort", "created", "--json"
        ]);

        assert!(success);
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Should be valid JSON");
        
        // Just verify it runs successfully and returns valid JSON
        assert!(parsed.is_array());
    }
}

mod search {
    use super::*;

    #[test]
    fn search_by_title() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "search", "orphan"
        ]);

        assert!(success);
        assert!(stdout.contains("Orphan Topic"));
    }

    #[test]
    fn search_by_tag() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "search", "tag:orphan"
        ]);

        assert!(success);
        assert!(stdout.contains("Topic C"));
        // Should not contain other topics without the orphan tag
        assert!(!stdout.contains("Topic A"));
    }

    #[test]
    fn search_no_results() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "search", "nonexistent_query_xyz"
        ]);

        assert!(success);
        assert!(stdout.contains("No topics found") || stdout.trim().is_empty() || stdout.contains("0"));
    }
}

mod info {
    use super::*;

    #[test]
    fn shows_workspace_info() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "info"
        ]);

        assert!(success);
        assert!(stdout.contains("Topics: 6")); // 6 fixture files
        assert!(stdout.contains("Root:"));
    }

    #[test]
    fn info_verbose_shows_field_details() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "info", "--verbose"
        ]);

        assert!(success);
        assert!(stdout.contains("tags")); // Should show tags field
    }
}

mod new_and_delete {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn creates_new_topic() {
        // Use a temp directory to avoid polluting fixtures
        let temp = TempDir::new().unwrap();
        
        // Copy a fixture to have at least one topic
        fs::copy(
            fixtures_path().join("topic-a.md"),
            temp.path().join("topic-a.md")
        ).unwrap();

        // Create a new topic (--no-edit to avoid opening editor)
        let (_, _, success) = run_hypha(&[
            "--root", temp.path().to_str().unwrap(),
            "new", "My New Topic", "--no-edit"
        ]);
        assert!(success, "Should create new topic");

        // Verify it exists
        let new_file = temp.path().join("my-new-topic.md");
        assert!(new_file.exists(), "New topic file should exist");

        // Verify content has title
        let content = fs::read_to_string(&new_file).unwrap();
        assert!(content.contains("# My New Topic"), "Should have title heading");
    }

    #[test]
    fn deletes_topic_by_filename() {
        let temp = TempDir::new().unwrap();
        
        // Create a topic to delete
        let topic_file = temp.path().join("to-delete.md");
        fs::write(&topic_file, "# To Delete\n\nContent").unwrap();

        let (stdout, _, success) = run_hypha(&[
            "--root", temp.path().to_str().unwrap(),
            "delete", "to-delete"
        ]);

        assert!(success, "Should delete topic");
        assert!(stdout.contains("Deleted"), "Should confirm deletion");
        assert!(!topic_file.exists(), "File should be deleted");
    }

    #[test]
    fn delete_nonexistent_fails() {
        let temp = TempDir::new().unwrap();
        
        // Create a dummy file so the root is valid
        fs::write(temp.path().join("dummy.md"), "# Dummy").unwrap();

        let (_, stderr, success) = run_hypha(&[
            "--root", temp.path().to_str().unwrap(),
            "delete", "nonexistent"
        ]);

        assert!(!success);
        assert!(
            stderr.contains("not found") || 
            stderr.contains("Not found") || 
            stderr.contains("No topic"),
            "Should report not found, got: {}", stderr
        );
    }
}

mod rename {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn renames_topic() {
        let temp = TempDir::new().unwrap();
        
        // Create a topic to rename
        let original = temp.path().join("original.md");
        fs::write(&original, "# Original Title\n\nSome content.").unwrap();

        let (_, _, success) = run_hypha(&[
            "--root", temp.path().to_str().unwrap(),
            "rename", "original", "renamed"
        ]);

        assert!(success, "Should rename topic");
        assert!(!original.exists(), "Original should not exist");
        assert!(temp.path().join("renamed.md").exists(), "Renamed file should exist");
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn handles_topic_without_frontmatter() {
        // No frontmatter topic should still be listable and searchable
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list"
        ]);

        assert!(success);
        assert!(stdout.contains("No Frontmatter"), "Topic without frontmatter should be listed");
    }

    #[test]
    fn backlinks_match_by_filename_stem() {
        // topic-a links to "topic-b.md", should match when searching for "topic-b"
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "backlinks", "Topic B"  // Using title, not filename
        ]);

        assert!(success);
        assert!(stdout.contains("Topic A"));
    }

    #[test]
    fn list_shows_tags() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "list"
        ]);

        assert!(success);
        // Topics with tags should show them
        assert!(stdout.contains("[test") || stdout.contains("test]"));
    }

    #[test]
    fn search_topic_without_frontmatter() {
        let fixtures = fixtures_path();
        let (stdout, _, success) = run_hypha(&[
            "--root", fixtures.to_str().unwrap(),
            "search", "frontmatter"
        ]);

        assert!(success);
        assert!(stdout.contains("No Frontmatter"));
    }
}
