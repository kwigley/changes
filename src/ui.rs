use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::error::Result;

pub fn select_input<T, I>(items: I) -> Result<Option<T>>
where
    T: ToString + Copy,
    I: Iterator<Item = T> + Clone,
{
    Ok(items.clone().nth(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What type of change is this?")
            .default(0)
            .items(
                &items
                    .map(|v| v.clone().to_string())
                    .collect::<Vec<String>>(),
            )
            .interact()?,
    ))
}
