use crate::error::{Error, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use edit::{edit_with_builder, Builder};
use names::Generator;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// TODO: move to config
const CHANGES_DIR: &str = ".test_changes/";

// TODO: move to config
const CHANGE_TYPES: [&str; 7] = [
    "bug",
    "feature",
    "chore",
    "docs",
    "refactor",
    "performance",
    "test",
];

#[derive(Debug, Deserialize, Serialize)]
struct ChangeFrontMatter {
    created: u64,
    #[serde(rename = "type")]
    entry_type: String,
}

fn prompt_for_change_type() -> Result<String> {
    let change_type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of change is this?")
        .default(0)
        .items(&CHANGE_TYPES)
        .interact()?;

    Ok((*CHANGE_TYPES
        .get(change_type_idx)
        .ok_or_else(|| Error::UserInput("Invalid change type".to_owned()))?)
    .to_owned())
}

pub fn add() -> Result<()> {
    let frontmatter = ChangeFrontMatter {
        created: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(Error::SystemTime)?
            .as_secs(),
        entry_type: prompt_for_change_type()?,
    };

    Generator::default().next().map_or_else(
        || Err(Error::UnableToGenerateFilename),
        |filename| {
            fs::create_dir_all(CHANGES_DIR)?;
            let filepath = CHANGES_DIR.to_owned() + &filename + ".md";
            let mut file = File::create(filepath)?;
            file.write_all(
                serde_frontmatter::serialize(frontmatter, &edit_with_builder("", &Builder::new())?)
                    .map_err(Error::Ser)?
                    .as_bytes(),
            )
            .map_err(Error::Io)
        },
    )
}

pub fn generate() -> Result<()> {
    todo!()
}

pub fn check() -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_serialization() {
        let frontmatter = ChangeFrontMatter {
            created: 1234,
            entry_type: "bug".to_owned(),
        };
        assert_eq!(
            serde_frontmatter::serialize(frontmatter, "test").unwrap(),
            "---\ncreated: 1234\ntype: bug\n\n---\ntest"
        )
    }
}
