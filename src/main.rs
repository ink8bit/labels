use reqwest::header::{ACCEPT, USER_AGENT};
// use reqwest::Error;
use serde::{Deserialize, Serialize};

use std::env;
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

    let repo = config.repo;
    let owner = config.owner;

    if list {
        if let Err(e) = view_labels(&owner, &repo) {
            panic!("{}", e);
        }
    }

    if update {
        todo!();
    }
}
#[derive(Deserialize, Serialize, Debug)]
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

const AUTH_HEADER: &str = "x-oauth-basic";

fn view_labels(owner: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/labels",
        owner = owner,
        repo = repo,
    );

    let response = reqwest::blocking::Client::new()
        .get(request_url)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "labels")
        .send()?;

    if response.status().is_success() {
        let labels: Vec<Label> = response.json()?;
        let pretty = serde_json::to_string_pretty(&labels)?;
        println!("{}", pretty);
    } else {
        eprintln!("{:?} failure", response);
    }

    Ok(())
}

fn get_token<'a>() -> Result<String, &'a str> {
    let token = env::var("LABELS_TOKEN").unwrap_or_default();
    if token.is_empty() {
        return Err("Token not found");
    }
    Ok(token)
}
