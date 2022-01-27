use crate::App;
use carli::error;
use carli::prelude::cmd::*;
use chrono::Utc;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use edit::{edit_with_builder, Builder};
use names::Generator;
use std::fs::{self, File};
use std::io::Write;

#[derive(clap::Parser, Debug)]
pub struct Subcommand {}

impl Execute<App> for Subcommand {
    fn execute(&self, _context: &App) -> Result<()> {
        let frontmatter = ChangeFrontMatter {
            created: Utc::now(),
            change_type: prompt_for_change_type()?,
        };

        // generate random filename
        let filename = Generator::default()
            .next()
            .ok_or_else(|| error!(1, "Unable to generate changes entry"))?;

        // mkdir -p the directory to write a changes entry to
        fs::create_dir_all(CHANGES_DIR)?;

        let filepath = CHANGES_DIR.to_owned() + &filename + ".md";
        let mut file = File::create(filepath)?;
        file.write_all(
            serde_frontmatter::serialize(frontmatter, &edit_with_builder("", &Builder::new())?)
                .map_err(|_| {
                    carli::error::Error::new(1).message("Unable to serialize changes entry")
                })?
                .as_bytes(),
        )
        .map_err(carli::error::Error::from)
    }
}

fn prompt_for_change_type() -> Result<ChangeType> {
    let change_types: Vec<String> = ChangeType::iter().map(|v| v.to_string()).collect();
    let change_type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of change is this?")
        .default(0)
        .items(change_types.as_slice())
        .interact()?;

    Ok(ChangeType::from_str(
        change_types
            .get(change_type_idx)
            .ok_or_else(|| error!(1, "Unknown change type"))?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod add {

        use chrono::DateTime;

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
