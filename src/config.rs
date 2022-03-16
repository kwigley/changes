use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChangesConfig {
    prompt: String,
}

impl ::std::default::Default for ChangesConfig {
    fn default() -> Self {
        Self {
            prompt: "What type of change is this?".to_string(),
        }
    }
}
