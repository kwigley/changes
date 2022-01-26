use crate::error::{Error, Result};
use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Select};
use edit::{edit_with_builder, Builder};
use names::Generator;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

// TODO(kw): move to config
const CHANGES_DIR: &str = ".test_changes/";

#[derive(EnumIter, EnumString, Debug, Deserialize, Serialize, Clone, Copy, Display)]
#[serde(rename_all = "lowercase")]
enum ChangeType {
    Fix,
    Feature,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChangeFrontMatter {
    created: DateTime<Utc>,
    #[serde(rename = "type")]
    change_type: ChangeType,
}

fn prompt_for_change_type() -> Result<ChangeType> {
    let change_types: Vec<String> = ChangeType::iter().map(|v| v.to_string()).collect();
    let change_type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of change is this?")
        .default(0)
        .items(change_types.as_slice())
        .interact()?;

    // TODO(kw): rm unwraps
    Ok(ChangeType::from_str(change_types.get(change_type_idx).unwrap()).unwrap())
}

pub fn generate() -> Result<()> {
    // will use a config to determine behavior eventually
    // looks for files in changes dir
    // combine, group, and sort entrys by type and timestamp
    // eventually tie entries with GH user/PR using the current branch and GH API
    // set header with version number and date
    // skip "chore" or "trivial" changes
    todo!()
}

pub fn check() -> Result<()> {
    // ensure PR/push has a changelog entry
    // can be run in CI or in pre-commit (pre-push)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    mod add {
        use super::*;
        #[test]
        fn test_frontmatter_serialization() {
            let timestamp = "2021-12-14T16:31:17.265529Z";
            let frontmatter = ChangeFrontMatter {
                created: DateTime::parse_from_rfc3339(timestamp)
                    .unwrap()
                    .with_timezone(&Utc),
                change_type: ChangeType::Fix,
            };
            assert_eq!(
                serde_frontmatter::serialize(frontmatter, "test").unwrap(),
                format!("---\ncreated: \"{}\"\ntype: fix\n\n---\ntest", timestamp)
            )
        }
    }
}
