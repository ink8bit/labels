pub mod config;
pub mod github;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Label {
    name: String,
    description: String,
    color: String,
}
