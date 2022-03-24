use crate::{
    change::{Change, ChangeFrontMatter},
    cli::Config,
    error::Error,
    error::Result,
    release::Release,
    template::Template,
    CHANGES_DIR,
};
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

impl Subcommand {
    pub fn execute(&self, _config: &Config) -> Result<()> {
        // This will be driven by config
        let template = r#"
        {%- if version -%}
            ## [{{ version | trim_start_matches(pat="v") }}] - {{ timestamp | date(format="%Y-%m-%d") }}
        {% else %}
            ## [unreleased]
        {%- endif -%}
        {% for type, change in changes | group_by(attribute="type") %}
            ### {{ type | upper_first }}
            {% for change in change -%}
                - {{ change.message | upper_first }}
            {%- endfor %}
        {% endfor %}"#;

        let template = Template::new(
            template
                .lines()
                .map(|l| l.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
        )?;

        let paths = fs::read_dir(CHANGES_DIR)?;
        let release = Release::new(
            self.version.clone(),
            paths
                .map(|path| {
                    let (frontmatter, message) =
                        serde_frontmatter::deserialize::<ChangeFrontMatter>(&fs::read_to_string(
                            path?.path(),
                        )?)
                        .map_err(Error::Ser)?;
                    Ok(Change::new(
                        frontmatter.created,
                        &frontmatter.change_type,
                        message.trim(),
                    ))
                })
                .collect::<Result<Vec<Change>>>()?,
            Utc::now(),
        );
        let path = PathBuf::from("CHANGELOG.md");
        let cl = fs::read_to_string(&path)?;
        let buf = &mut File::create(&path)?;
        let content = template.render(&release)?;

        write!(buf, "{}\n{}", content, cl)?;

        fs::remove_dir_all(CHANGES_DIR)?;
        fs::create_dir(CHANGES_DIR)?;

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
