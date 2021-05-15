use serde::Deserialize;

use std::fs;

mod cli;

const CONFIG_FILE: &str = ".labelsrc.json";

fn main() {
    let args = cli::args();
    let list = args.is_present("list");
    let update = args.is_present("update");

    let config = match Config::new() {
        Ok(v) => v,
        Err(e) => return eprint!("Config file not found: {}", e),
    };

    dbg!(&config);
    dbg!(&list);
    dbg!(&update);
}
#[derive(Deserialize, Debug)]
struct Label {
    name: String,
    description: String,
    color: String,
}
#[derive(Deserialize, Debug)]
struct Config {
    repo: String,
    owner: String,
    labels: Vec<Label>,
}

impl Config {
    fn new() -> Result<Self, std::io::Error> {
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
