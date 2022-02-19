use reqwest::header::{self, ACCEPT, USER_AGENT};
use reqwest::Client;

use std::time::Duration;

use crate::labels::{error::LabelsError, Label};

/// An authorization header.
const AUTH_HEADER: &str = "x-oauth-basic";

/// An accept header you should provide in order to use GitHub API.
/// Read more in [docs](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#current-version).
const ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

/// User agent you should provide in oreder to use GitHub API.
/// Read more in [docs](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required).
const USER_AGENT_HEADER: &str = "labels";

pub(crate) struct GitHub {
    base_url: String,
    token: String,
    client: Client,
}

impl GitHub {
    /// Create a new GitHub client.
    ///
    /// # Arguments
    ///
    /// - `base_url` - base url for all requests
    /// - `token` - authorization token
    pub(crate) fn new(base_url: String, token: String) -> Result<Self, reqwest::Error> {
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

        Ok(Self {
            base_url,
            token,
            client,
        })
    }

    /// Get all labels for a repository.
    /// See usage in [GitHub REST API docs](https://docs.github.com/en/rest/reference/issues#list-labels-for-a-repository).
    ///
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    async fn labels(&self, owner: &str, repo: &str) -> Result<Vec<Label>, LabelsError> {
        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels",
            base_url = self.base_url,
            owner = owner,
            repo = repo,
        );

        let response = self
            .client
            .get(request_url)
            .basic_auth(&self.token, Some(AUTH_HEADER))
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
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    pub(crate) async fn print_labels(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<String, LabelsError> {
        let labels = Self::labels(self, owner, repo).await?;
        let pretty = serde_json::to_string_pretty(&labels)?;

        Ok(pretty)
    }

    /// Create a label.
    /// See usage in [GitHub REST API docs](https://docs.github.com/en/rest/reference/issues#create-a-label).
    ///
    /// # Arguments
    ///
    /// - `label` - a label struct with name, description, and color values.
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    async fn create_label(
        &self,
        label: &Label,
        owner: &str,
        repo: &str,
    ) -> Result<(), LabelsError> {
        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels",
            base_url = self.base_url,
            owner = owner,
            repo = repo,
        );

        let response = self
            .client
            .post(request_url)
            .json(&label)
            .basic_auth(&self.token, Some(AUTH_HEADER))
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
    /// - `name` - name of the specific label.
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    async fn delete_label(&self, name: &str, owner: &str, repo: &str) -> Result<(), LabelsError> {
        let request_url = format!(
            "{base_url}/repos/{owner}/{repo}/labels/{name}",
            base_url = self.base_url,
            owner = owner,
            repo = repo,
            name = name,
        );

        let response = self
            .client
            .delete(request_url)
            .basic_auth(&self.token, Some(AUTH_HEADER))
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
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    pub(crate) async fn update_labels(
        &self,
        labels_from_config: &[Label],
        owner: &str,
        repo: &str,
    ) -> Result<(), LabelsError> {
        let labels = Self::labels(self, owner, repo).await?;

        if labels.is_empty() {
            return Err(LabelsError::GitHubLabelUpdate);
        }

        for label in labels {
            if Self::delete_label(self, &label.name, owner, repo)
                .await
                .is_err()
            {
                return Err(LabelsError::GitHubLabelDelete);
            }
        }

        for label in labels_from_config {
            if Self::create_label(self, label, owner, repo).await.is_err() {
                return Err(LabelsError::GitHubLabelCreate);
            }
        }

        Ok(())
    }

    /// Remove all labels from a specific repository.
    ///
    /// # Arguments
    ///
    /// - `owner` - owner of the repo.
    /// - `repo` - repo name.
    pub(crate) async fn remove_labels(&self, owner: &str, repo: &str) -> Result<(), LabelsError> {
        let labels = Self::labels(self, owner, repo).await?;

        if !labels.is_empty() {
            for label in labels {
                if Self::delete_label(self, &label.name, owner, repo)
                    .await
                    .is_err()
                {
                    return Err(LabelsError::GitHubLabelDelete);
                }
            }
        }

        Ok(())
    }
}
