use reqwest::header::{ACCEPT, USER_AGENT};

use std::env;
use std::time::Duration;

use crate::label::Label;

const AUTH_HEADER: &str = "x-oauth-basic";

pub(crate) fn view_labels(owner: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/labels",
        owner = owner,
        repo = repo,
    );

    let response = reqwest::blocking::Client::new()
        .get(request_url)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "labels")
        .send()?;

    if !response.status().is_success() {
        panic!("Error: status code {}", response.status());
    }

    let labels: Vec<Label> = response.json()?;
    let pretty = serde_json::to_string_pretty(&labels)?;
    println!("{}", pretty);

    Ok(())
}

fn get_token<'a>() -> Result<String, &'a str> {
    let token = env::var("LABELS_TOKEN").unwrap_or_default();
    if token.is_empty() {
        return Err("Token not found");
    }
    Ok(token)
}
