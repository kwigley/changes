use crate::app::{App, CHANGES_DIR};
use crate::change::{ChangeFrontMatter, ChangeType};
use crate::error::Error;
use carli::error;
use carli::prelude::cmd::*;
use chrono::Utc;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use edit::{edit_with_builder, Builder};
use names::Generator;
use std::fs::{self, File};
use std::io::Write;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(clap::Parser, Debug)]
pub struct Subcommand {}

impl Execute<App> for Subcommand {
    fn execute(&self, _context: &App) -> Result<()> {
        let change_types: Vec<String> = ChangeType::iter().map(|v| v.to_string()).collect();
        let change_type_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What type of change is this?")
            .default(0)
            .items(change_types.as_slice())
            .interact()?;

        let frontmatter = ChangeFrontMatter::new(
            Utc::now(),
            ChangeType::from_str(
                change_types
                    .get(change_type_idx)
                    .ok_or_else(|| error!(1, "Unknown change type"))?,
            )?,
        );

        // mkdir -p the directory to write a changes entry to
        fs::create_dir_all(CHANGES_DIR)
            .and(Ok(File::create(
                CHANGES_DIR.to_owned()
                    + &Generator::default()
                        .next()
                        .ok_or_else(|| Error::UnableToGenerateFilename)?
                    + ".md",
            )?
            .write_all(
                serde_frontmatter::serialize(frontmatter, &edit_with_builder("", &Builder::new())?)
                    .map_err(|e| Error::Ser(e))?
                    .as_bytes(),
            )
            .map_err(carli::error::Error::from)))
            .map_err(carli::error::Error::from)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_frontmatter_serialization() {
        let timestamp = "2021-12-14T16:31:17.265529Z";
        let frontmatter = ChangeFrontMatter::new(
            DateTime::parse_from_rfc3339(timestamp)
                .unwrap()
                .with_timezone(&Utc),
            ChangeType::Fix,
        );
        assert_eq!(
            serde_frontmatter::serialize(frontmatter, "test").unwrap(),
            format!("---\ncreated: \"{}\"\ntype: fix\n\n---\ntest", timestamp)
        )
    }
}
