use crate::error::{
    Error::{IoError, UserInputError},
    Result,
};
use indoc::formatdoc;
use names::Generator;
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use dialoguer::{theme::ColorfulTheme, Select};
use edit::{edit_with_builder, Builder};

// 1. get type of change (bug, feature, chore, docs, refactor, perf, test)
//      these can be defaults but can also be driven by a config file
// 2. ask to enter input or open editor to get changelog entry (starting with editor)
// 3. write to a unique file in `.changes`, include frontmatter

// TODO: move to config
const CHANGES_DIR: &str = ".test_changes/";

pub fn add() -> Result<()> {
    let change_types = &[
        "bug",
        "feature",
        "chore",
        "docs",
        "refactor",
        "performance",
        "test",
    ];
    let change_type = change_types
        .get(
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What type of change is this?")
                .default(0)
                .items(&change_types[..])
                .interact()?,
        )
        .ok_or_else(|| UserInputError("Invalid change type".to_owned()))?;

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"), // TODO: return an error here
    };

    let template = ""; // TODO: might be nice to provide placeholder?
    if let Ok(contents) = edit_with_builder(template, &Builder::new()) {
        if let Some(filename) = Generator::default().next() {
            fs::create_dir_all(CHANGES_DIR)?;
            let filepath = CHANGES_DIR.to_owned() + &filename + ".md";
            let mut file = File::create(filepath)?;
            let frontmatter = formatdoc! {"
                ---
                created: {now}
                type: {change_type}
                ---
                \n",
                now = now,
                change_type = change_type
            };
            // match file.write_all((frontmatter + &contents).as_bytes()) {
            //     Ok(_) => Ok(()),
            //     Err(e) => Err(IoError(e)),
            // }

            file.write_all((frontmatter + &contents).as_bytes())
                .or_else(|e| Err(IoError(e)))
        } else {
            // TODO: this should probably error
            Ok(())
        }
    } else {
        // TODO: this should probably be an error
        Ok(())
    }
}
