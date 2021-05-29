use reqwest::header::{ACCEPT, USER_AGENT};

use std::{env, error::Error, time::Duration};

use crate::label::Label;

const AUTH_HEADER: &str = "x-oauth-basic";

fn get_token<'a>() -> Result<String, &'a str> {
    let token = env::var("LABELS_TOKEN").unwrap_or_default();
    if token.is_empty() {
        return Err("Token not found");
    }
    Ok(token)
}

fn labels(owner: &str, repo: &str) -> Result<Vec<Label>, Box<dyn Error>> {
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

    Ok(labels)
}

pub(crate) fn print_labels(owner: &str, repo: &str) -> Result<(), Box<dyn Error>> {
    let labels = labels(owner, repo)?;
    let pretty = serde_json::to_string_pretty(&labels)?;
    println!("{}", pretty);

    Ok(())
}

pub(crate) fn update_labels(
    owner: &str,
    repo: &str,
    labels_from_config: &Vec<Label>,
) -> Result<(), Box<dyn Error>> {
    let labels = labels(owner, repo)?;

    if !labels.is_empty() {
        for label in labels {
            if let Err(e) = delete_label(owner, repo, &label.name) {
                eprintln!("Error while deleting a label: {}\n{}", label.name, e);
            }
        }
    }

    for label in labels_from_config {
        if let Err(e) = create_label(owner, repo, label) {
            eprintln!("Error while creating a label: {}\n{}", label.name, e);
        }
    }

    Ok(())
}

fn create_label(owner: &str, repo: &str, label: &Label) -> Result<(), Box<dyn Error>> {
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
        .post(request_url)
        .json(&label)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "labels")
        .send()?;

    if !response.status().is_success() {
        panic!("Error: status code {}", response.status());
    }

    Ok(())
}

fn delete_label(owner: &str, repo: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/labels/{name}",
        owner = owner,
        repo = repo,
        name = name,
    );

    let response = reqwest::blocking::Client::new()
        .delete(request_url)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "labels")
        .send()?;

    if !response.status().is_success() {
        panic!("Error: status code {}", response.status());
    }

    Ok(())
}
