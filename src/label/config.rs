use serde::Deserialize;
use std::fs;

use super::Label;

const CONFIG_FILE: &str = ".labelsrc.json";

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) repo: String,
    pub(crate) owner: String,
    pub(crate) labels: Vec<Label>,
}

impl Config {
    pub(crate) fn new() -> Result<Self, std::io::Error> {
        let config = Self::parse()?;

        Ok(Self {
            repo: config.repo,
            owner: config.owner,
            labels: config.labels,
        })
    }

    fn parse() -> Result<Config, std::io::Error> {
        let config_data = fs::read_to_string(CONFIG_FILE)?;
        let config: Config = serde_json::from_str(&config_data)?;

        Ok(config)
    }
}
