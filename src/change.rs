use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const DEFAULT_CHANGE_KINDS: [&str; 9] = [
    "fix", "feature", "chore", "build", "ci", "doc", "test", "perf", "refactor",
];

pub const DEFAULT_CHANGELOG_EXT: &str = ".md";

#[derive(Debug, Deserialize, Serialize)]
pub struct Change {
    created: DateTime<Utc>,
    #[serde(rename = "type")]
    change_type: String,
    message: String,
}

impl Change {
    pub fn new(created: DateTime<Utc>, change_type: &str, message: &str) -> Self {
        Self {
            created,
            change_type: change_type.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeFrontMatter {
    pub created: DateTime<Utc>,
    #[serde(rename = "type")]
    pub change_type: String,
}

impl ChangeFrontMatter {
    pub fn new(created: DateTime<Utc>, change_type: &str) -> Self {
        ChangeFrontMatter {
            created,
            change_type: change_type.to_string(),
        }
    }
}
