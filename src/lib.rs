mod app;
mod error;
mod subcommand;
mod util;

#[doc(hidden)]
pub use app::App;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Deserialize, Serialize)]
struct ChangeFrontMatter {
    created: DateTime<Utc>,
    #[serde(rename = "type")]
    change_type: ChangeType,
}

#[derive(EnumIter, EnumString, Debug, Deserialize, Serialize, Clone, Copy, Display)]
#[serde(rename_all = "lowercase")]
enum ChangeType {
    Fix,
    Feature,
}
