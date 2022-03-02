use std::fmt::Display;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::error::{Error, Result};

pub fn select_input<T: Display>(options: &[T]) -> Result<&T> {
    let idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of change is this?")
        .default(0)
        .items(options)
        .interact()?;
    options
        .get(idx)
        .ok_or_else(|| Error::InvalidChangeType("None".to_owned()))
}
