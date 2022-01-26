impl Execute<Application> for Subcommand {
    fn execute(&self, context: &Application) -> Result<()> {
        match self {
            Self::Goodbye => writeln!(context.output(), "Goodbye, {}!", context.name())?,
            Self::Hello => writeln!(context.output(), "Hello, {}!", context.name())?,
        }

        Ok(())
    }
}

pub fn add() -> Result<()> {
    let frontmatter = ChangeFrontMatter {
        created: Utc::now(),
        change_type: prompt_for_change_type()?,
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
