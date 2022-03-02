use std::fs::{self, File};
use std::io::Write;

use chrono::Utc;
use edit::{edit_with_builder, Builder};
use names::Generator;
use strum::IntoEnumIterator;

use crate::app::{App, CHANGES_DIR};
use crate::change::{ChangeFrontMatter, ChangeType};
use crate::error::{Error, Result};
use crate::ui::select_input;

#[derive(clap::Parser, Debug)]
pub struct Subcommand {}

impl Subcommand {
    pub fn execute(&self, _context: &App) -> Result<()> {
        let frontmatter = ChangeFrontMatter::new(
            Utc::now(),
            *select_input(&ChangeType::iter().collect::<Vec<_>>())?,
        );

        // mkdir -p the directory to write a changes entry to
        Ok(fs::create_dir_all(CHANGES_DIR).and(Ok(File::create(
            CHANGES_DIR.to_owned()
                + &Generator::default()
                    .next()
                    .ok_or(Error::UnableToGenerateFilename)?
                + ".md",
        )?
        .write_all(
            serde_frontmatter::serialize(frontmatter, &edit_with_builder("", &Builder::new())?)
                .map_err(Error::Ser)?
                .as_bytes(),
        )?))?)
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
