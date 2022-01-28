use crate::{
    app::CHANGES_DIR,
    change::{Change, ChangeFrontMatter},
    error::Error,
    release::Release,
    template::Template,
    App,
};
use carli::prelude::cmd::*;
use chrono::Utc;
use semver::Version;
use std::io::Write;
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(clap::Parser, Debug)]
pub struct Subcommand {
    version: Version,
}

impl Execute<App> for Subcommand {
    fn execute(&self, _context: &App) -> Result<()> {
        // This will be driven by config
        let template = r#"
        {% if version %}
            ## [{{ version | trim_start_matches(pat="v") }}] - {{ timestamp | date(format="%Y-%m-%d") }}
        {% else %}
            ## [unreleased]
        {% endif %}
        {% for type, change in changes | group_by(attribute="type") %}
            ### {{ type | upper_first }}
            {% for change in change %}
                - {{ change.message | upper_first }}
            {% endfor %}
        {% endfor %}"#;
        let template = Template::new(template.to_string())?;

        let paths = fs::read_dir(CHANGES_DIR)?;
        let release = Release::new(
            self.version.clone(),
            paths
                .map(|path| {
                    let (frontmatter, message) =
                        serde_frontmatter::deserialize::<ChangeFrontMatter>(&fs::read_to_string(
                            path?.path(),
                        )?)
                        .map_err(|e| Error::Ser(e))?;
                    Ok(Change::new(
                        frontmatter.created,
                        frontmatter.change_type,
                        message,
                    ))
                })
                .collect::<Result<Vec<Change>>>()?,
            Utc::now(),
        );
        let path = PathBuf::from("CHANGELOG.md");
        let buf = &mut File::create(&path)?;
        let cl = fs::read_to_string(path)?;
        let content = template.render(&release)?;

        write!(buf, "{}\n{}", content, cl)?;

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {}
// }
