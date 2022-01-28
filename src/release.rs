use crate::change::Change;
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    version: Version,
    changes: Vec<Change>,
    timestamp: DateTime<Utc>,
}

impl Release {
    pub fn new(version: Version, changes: Vec<Change>, timestamp: DateTime<Utc>) -> Self {
        Self {
            version,
            changes,
            timestamp,
        }
    }
}
