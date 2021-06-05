use reqwest::header::{self, ACCEPT, USER_AGENT};
use reqwest::Client;

use std::{
    env::{self, VarError},
    fmt,
    time::Duration,
};

use crate::label::Label;

const AUTH_HEADER: &str = "x-oauth-basic";
const ACCEPT_HEADER: &str = "application/vnd.github.v3+json";
const USER_AGENT_HEADER: &str = "labels";
const API_URL: &str = "https://api.github.com";
const LABELS_TOKEN: &str = "LABELS_TOKEN";

#[derive(Debug)]
pub(crate) enum LabelsError {
    NoTokenValue,
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
            LabelsError::NoTokenValue => write!(f, "Could not get a token from LABELS_TOKEN env. Make sure you set LABELS_TOKEN env variable."),
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

impl From<VarError> for LabelsError {
    fn from(_: VarError) -> Self {
        LabelsError::NoTokenValue
    }
}

pub(crate) struct GitHub<'a> {
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GitHub<'a> {
    pub(crate) fn new(owner: &'a str, repo: &'a str) -> Self {
        Self { owner, repo }
    }

    fn client() -> Result<Client, reqwest::Error> {
        let timeout = Duration::from_secs(3);

        let mut headers = header::HeaderMap::new();
        headers.insert(ACCEPT, header::HeaderValue::from_static(ACCEPT_HEADER));
        headers.insert(
            USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT_HEADER),
        );

        let client = reqwest::Client::builder()
            .timeout(timeout)
            .default_headers(headers)
            .build()?;

        Ok(client)
    }

    async fn labels(&self) -> Result<Vec<Label>, LabelsError> {
        let token = env::var(LABELS_TOKEN)?;

        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels",
            base_url = API_URL,
            owner = self.owner,
            repo = self.repo,
        );

        let client = GitHub::client()?;

        let response = client
            .get(request_url)
            .basic_auth(token, Some(AUTH_HEADER))
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::OK {
            return Err(LabelsError::InvalidResponse);
        }

        let labels: Vec<Label> = response.json().await?;

        Ok(labels)
    }

    pub(crate) async fn print_labels(&self) -> Result<(), LabelsError> {
        let labels = Self::labels(&self).await?;
        let pretty = serde_json::to_string_pretty(&labels)?;
        println!("{}", pretty);

        Ok(())
    }

    async fn create_label(&self, label: &Label) -> Result<(), LabelsError> {
        let token = env::var(LABELS_TOKEN)?;

        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels",
            base_url = API_URL,
            owner = self.owner,
            repo = self.repo,
        );

        let client = GitHub::client()?;

        let response = client
            .post(request_url)
            .json(&label)
            .basic_auth(token, Some(AUTH_HEADER))
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::CREATED {
            return Err(LabelsError::GitHubLabelCreate);
        }

        Ok(())
    }

    async fn delete_label(&self, name: &str) -> Result<(), LabelsError> {
        let token = env::var(LABELS_TOKEN)?;

        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels/{name}",
            base_url = API_URL,
            owner = self.owner,
            repo = self.repo,
            name = name,
        );

        let client = GitHub::client()?;

        let response = client
            .delete(request_url)
            .basic_auth(token, Some(AUTH_HEADER))
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::NO_CONTENT {
            return Err(LabelsError::GitHubLabelDelete);
        }

        Ok(())
    }

    pub(crate) async fn update_labels(
        &self,
        labels_from_config: &Vec<Label>,
    ) -> Result<(), LabelsError> {
        let labels = Self::labels(&self).await?;

        if !labels.is_empty() {
            for label in labels {
                if let Err(_) = Self::delete_label(&self, &label.name).await {
                    return Err(LabelsError::GitHubLabelDelete);
                }
            }
        }

        for label in labels_from_config {
            if let Err(_) = Self::create_label(&self, label).await {
                return Err(LabelsError::GitHubLabelCreate);
            }
        }

        Ok(())
    }
}
