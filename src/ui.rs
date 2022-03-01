use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::error::Result;

pub fn select_input<T, I>(items: I) -> Result<Option<T>>
where
    T: ToString + Copy,
    I: IntoIterator<Item = T> + Clone,
{
    Ok(items.clone().into_iter().nth(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What type of change is this?")
            .default(0)
            .items(
                &items
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>(),
            )
            .interact()?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_select_input() -> Result<()> {
        select_input(vec!["option1", "option2"])?;
        Ok(())
    }
}
