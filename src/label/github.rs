use reqwest::header::{ACCEPT, USER_AGENT};

use std::{env, fmt, time::Duration};

use crate::label::Label;

const AUTH_HEADER: &str = "x-oauth-basic";
const ACCEPT_HEADER: &str = "application/vnd.github.v3+json";
const USER_AGENT_HEADER: &str = "labels";
const API_URL: &str = "https://api.github.com";
const LABELS_TOKEN: &str = "LABELS_TOKEN";

#[derive(Debug)]
pub(crate) enum LabelsError {
    InvalidResponse,
    Http,
    JsonSerialization,
    GitHubLabelCreate,
    GitHubLabelDelete,
}

impl std::error::Error for LabelsError {}

impl fmt::Display for LabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelsError::Http => write!(f, "HTTP error"),
            LabelsError::InvalidResponse => write!(f, "Invalid response"),
            LabelsError::JsonSerialization => write!(f, "Could not serialize labels data"),
            LabelsError::GitHubLabelCreate => write!(f, "Could not create a label"),
            LabelsError::GitHubLabelDelete => write!(f, "Could not delete a label"),
        }
    }
}

impl From<reqwest::Error> for LabelsError {
    fn from(_: reqwest::Error) -> Self {
        LabelsError::Http
    }
}

impl From<serde_json::Error> for LabelsError {
    fn from(_: serde_json::Error) -> Self {
        LabelsError::JsonSerialization
    }
}

fn get_token<'a>() -> Result<String, &'a str> {
    let token = env::var(LABELS_TOKEN).unwrap_or_default();
    if token.is_empty() {
        return Err("Token not found");
    }
    Ok(token)
}

fn labels(owner: &str, repo: &str) -> Result<Vec<Label>, LabelsError> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "{base_url}/repos/{owner}/{repo}/labels",
        base_url = API_URL,
        owner = owner,
        repo = repo,
    );

    let response = reqwest::blocking::Client::new()
        .get(request_url)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, ACCEPT_HEADER)
        .header(USER_AGENT, USER_AGENT_HEADER)
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(LabelsError::InvalidResponse);
    }

    let labels: Vec<Label> = response.json()?;

    Ok(labels)
}

pub(crate) fn print_labels(owner: &str, repo: &str) -> Result<(), LabelsError> {
    let labels = labels(owner, repo)?;
    let pretty = serde_json::to_string_pretty(&labels)?;
    println!("{}", pretty);

    Ok(())
}

pub(crate) fn update_labels(
    owner: &str,
    repo: &str,
    labels_from_config: &Vec<Label>,
) -> Result<(), LabelsError> {
    let labels = labels(owner, repo)?;

    if !labels.is_empty() {
        for label in labels {
            if let Err(_) = delete_label(owner, repo, &label.name) {
                return Err(LabelsError::GitHubLabelDelete);
            }
        }
    }

    for label in labels_from_config {
        if let Err(_) = create_label(owner, repo, label) {
            return Err(LabelsError::GitHubLabelCreate);
        }
    }

    Ok(())
}

fn create_label(owner: &str, repo: &str, label: &Label) -> Result<(), LabelsError> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "{base_url}/repos/{owner}/{repo}/labels",
        base_url = API_URL,
        owner = owner,
        repo = repo,
    );

    let response = reqwest::blocking::Client::new()
        .post(request_url)
        .json(&label)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, ACCEPT_HEADER)
        .header(USER_AGENT, USER_AGENT_HEADER)
        .send()?;

    if response.status() != reqwest::StatusCode::CREATED {
        return Err(LabelsError::GitHubLabelCreate);
    }

    Ok(())
}

fn delete_label(owner: &str, repo: &str, name: &str) -> Result<(), LabelsError> {
    let token = match get_token() {
        Ok(v) => v,
        Err(e) => e.to_string(),
    };

    let timeout = Duration::new(5, 0);
    let request_url = format!(
        "{base_url}/repos/{owner}/{repo}/labels/{name}",
        base_url = API_URL,
        owner = owner,
        repo = repo,
        name = name,
    );

    let response = reqwest::blocking::Client::new()
        .delete(request_url)
        .timeout(timeout)
        .basic_auth(token, Some(AUTH_HEADER))
        .header(ACCEPT, ACCEPT_HEADER)
        .header(USER_AGENT, USER_AGENT_HEADER)
        .send()?;

    if response.status() != reqwest::StatusCode::NO_CONTENT {
        return Err(LabelsError::GitHubLabelDelete);
    }

    Ok(())
}
