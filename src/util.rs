use crate::error::Result;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn prompt_for_selection<T, I>(options: I) -> Result<T>
where
    T: ToString,
    I: Iterator<Item = T>,
{
    let change_type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What type of change is this?")
        .default(0)
        .items(
            &options
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .as_slice(),
        )
        .interact()?;

    Ok(*options.collect::<Vec<T>>().get(change_type_idx).unwrap())
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn prompt_for_selection_with_vec() {
    //     prompt_for_selection(vec!["1".to_owned(), "2".to_owned(), "3".to_owned()].iter());
    // }
}
