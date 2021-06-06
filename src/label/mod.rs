pub mod config;
pub mod github;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Label {
    name: String,
    #[serde(default = "default_string_value")]
    description: String,
    #[serde(default = "default_string_value")]
    color: String,
}

fn default_string_value() -> String {
    "".to_string()
}
