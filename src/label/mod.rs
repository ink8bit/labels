pub mod config;
pub mod error;
pub mod github;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Label {
    name: String,
    color: String,
    #[serde(deserialize_with = "parse_description")]
    description: String,
}

fn parse_description<'a, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'a>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_else(|| "".to_string()))
}
