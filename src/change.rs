use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Deserialize, Serialize)]
pub struct Change {
    timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    change_type: ChangeType,
    message: String,
}

impl Change {
    pub fn new(created: DateTime<Utc>, change_type: ChangeType, message: String) -> Self {
        Self {
            timestamp: created,
            change_type,
            message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeFrontMatter {
    pub created: DateTime<Utc>,
    #[serde(rename = "type")]
    pub change_type: ChangeType,
}

impl ChangeFrontMatter {
    pub fn new(created: DateTime<Utc>, change_type: ChangeType) -> Self {
        ChangeFrontMatter {
            created,
            change_type,
        }
    }
}

#[derive(EnumIter, EnumString, Debug, Deserialize, Serialize, Clone, Copy, Display)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Fix,
    Feature,
    Chore,
    Build,
    Ci,
    Docs,
    Test,
    Perf,
    Refactor,
}
