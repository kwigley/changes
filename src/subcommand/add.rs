use super::App;
use crate::{error::Error, util::prompt_for_selection, ChangeFrontMatter, ChangeType};
use carli::prelude::cmd::*;
use chrono::Utc;
use edit::{edit_with_builder, Builder};
use names::Generator;
use std::fs::{self, File};
use std::io::Write;
use strum::IntoEnumIterator;

// TODO(kw): move to config
const CHANGES_DIR: &str = ".test_changes/";
/// The subcommand options.
#[derive(clap::Parser, Debug)]
pub struct Subcommand {}

impl Execute<App> for Subcommand {
    fn execute(&self, _context: &App) -> carli::error::Result<()> {
        let frontmatter = ChangeFrontMatter {
            created: Utc::now(),
            change_type: prompt_for_selection(ChangeType::iter().into_iter())?,
        };

        Generator::default().next().map_or_else(
            || Err(Error::UnableToGenerateFilename),
            |filename| {
                fs::create_dir_all(CHANGES_DIR)?;
                let filepath = CHANGES_DIR.to_owned() + &filename + ".md";
                let mut file = File::create(filepath)?;
                file.write_all(
                    serde_frontmatter::serialize(
                        frontmatter,
                        &edit_with_builder("", &Builder::new())?,
                    )
                    .map_err(Error::Ser)?
                    .as_bytes(),
                )
                .map_err(Error::Io)
            },
        );
        Ok(())
    }
}
