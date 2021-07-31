use reqwest::header::{self, ACCEPT, USER_AGENT};
use reqwest::Client;

use std::{
    env::{self},
    time::Duration,
};

use crate::label::{error::LabelsError, Label};

/// An authorization header.
const AUTH_HEADER: &str = "x-oauth-basic";

/// An accept header you should provide in order to use GitHub API.
/// Read more in [docs](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#current-version).
const ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

/// User agent you should provide in oreder to use GitHub API.
/// Read more in [docs](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required).
const USER_AGENT_HEADER: &str = "labels";

/// GitHub API base url.
const API_URL: &str = "https://api.github.com";

/// An environment variable with your personal access token.
const LABELS_TOKEN: &str = "LABELS_TOKEN";

pub(crate) struct GitHub<'a> {
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GitHub<'a> {
    /// Create a new GitHub instance.
    ///
    /// # Arguments
    ///
    /// - `owner` - a repository owner,
    /// - `repo` - a repo name.
    pub(crate) fn new(owner: &'a str, repo: &'a str) -> Self {
        Self { owner, repo }
    }

    /// Create GitHub client.
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

    /// Get all labels for a repository.
    /// See usage in [GitHub REST API docs](https://docs.github.com/en/rest/reference/issues#list-labels-for-a-repository).
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

        let labels = match response.json::<Vec<Label>>().await {
            Ok(v) => v,
            Err(_) => return Err(LabelsError::JsonSerialization),
        };

        Ok(labels)
    }

    /// Print the first 100 labels to stdout.
    pub(crate) async fn print_labels(&self) -> Result<String, LabelsError> {
        let labels = Self::labels(self).await?;
        let pretty = serde_json::to_string_pretty(&labels)?;

        Ok(pretty)
    }

    /// Create a label.
    /// See usage in [GitHub REST API docs](https://docs.github.com/en/rest/reference/issues#create-a-label).
    ///
    /// # Arguments
    ///
    /// - `label` - a label struct with name, description, and color values.
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

    /// Delete a label.
    /// See usage in [GitHub REST API docs](https://docs.github.com/en/rest/reference/issues#delete-a-label).
    ///
    /// # Arguments
    ///
    /// - `name` - a name of the specific label.
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

    /// Update all labels in a repository.
    ///
    /// `update_labels` removes all labels from a repostiory and
    /// after that creates all labels from your config file.
    ///
    /// # Arguments
    ///
    /// - `labels_from_config` - a list of labels from your configuration file.
    pub(crate) async fn update_labels(
        &self,
        labels_from_config: &[Label],
    ) -> Result<(), LabelsError> {
        let labels = Self::labels(self).await?;

        if !labels.is_empty() {
            for label in labels {
                if Self::delete_label(self, &label.name).await.is_err() {
                    return Err(LabelsError::GitHubLabelDelete);
                }
            }
        }

        for label in labels_from_config {
            if Self::create_label(self, label).await.is_err() {
                return Err(LabelsError::GitHubLabelCreate);
            }
        }

        Ok(())
    }
}
