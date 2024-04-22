use std::fs::{self, File};
use std::io::Write;

use chrono::Utc;
use edit::{edit_with_builder, Builder};
use names::Generator;

use crate::change::{ChangeFrontMatter, DEFAULT_CHANGELOG_EXT, DEFAULT_CHANGE_KINDS};
use crate::cli::{select_input, Config};
use crate::error::{Error, Result};

#[derive(clap::Parser, Debug)]
pub struct Subcommand {}

impl Subcommand {
    pub fn execute(&self, config: &Config) -> Result<()> {
        let frontmatter = ChangeFrontMatter::new(
            Utc::now(),
            select_input(&DEFAULT_CHANGE_KINDS, config.prompt.to_string())?, //context.prompt)?,
        );

        // mkdir -p the directory to write a changes entry to
        Ok(fs::create_dir_all(&config.changes_dir).and(Ok(File::create(
            config.changes_dir.to_owned()
                + &Generator::default()
                    .next()
                    .ok_or(Error::UnableToGenerateFilename)?
                + DEFAULT_CHANGELOG_EXT,
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
            "fix",
        );
        assert_eq!(
            serde_frontmatter::serialize(frontmatter, "test").unwrap(),
            format!("---\ncreated: \"{}\"\ntype: fix\n\n---\ntest", timestamp)
        )
    }
}
