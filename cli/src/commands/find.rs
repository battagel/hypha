use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

use crate::cli::SortOrder;
use crate::core::index;
use crate::core::topic::Topic;

/// JSON output format for topics.
#[derive(Serialize)]
struct TopicJson {
    title: String,
    description: Option<String>,
    path: String,
    frontmatter: HashMap<String, serde_yaml::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
}

impl From<&Topic> for TopicJson {
    fn from(topic: &Topic) -> Self {
        TopicJson {
            title: topic.title.clone(),
            description: topic.description.clone(),
            path: topic.path.display().to_string(),
            frontmatter: topic.metadata.clone(),
            warnings: topic.warnings.iter().map(|w| w.to_string()).collect(),
        }
    }
}

/// Search topics by query.
pub fn run(root: &Path, query: &str, json: bool, sort: SortOrder) -> Result<()> {
    let topics = index::search(root, query, sort)?;

    if json {
        let json_topics: Vec<TopicJson> = topics.iter().map(TopicJson::from).collect();
        println!("{}", serde_json::to_string(&json_topics)?);
    } else {
        for topic in topics {
            println!("{}", topic.display());
        }
    }
    Ok(())
}
