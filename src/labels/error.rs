use std::env::VarError;
use std::fmt;

#[derive(Debug)]
pub(crate) enum LabelsError {
    NoTokenValue,
    InvalidResponse,
    Http,
    JsonSerialization,
    GitHubLabelCreate,
    GitHubLabelUpdate,
    GitHubLabelDelete,
}

impl std::error::Error for LabelsError {}

impl fmt::Display for LabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelsError::Http => write!(f, "Error while performing a HTTP request"),
            LabelsError::InvalidResponse => write!(f, "Invalid response"),
            LabelsError::JsonSerialization => write!(f, "Could not serialize labels data"),
            LabelsError::GitHubLabelCreate => write!(f, "Could not create a label"),
            LabelsError::GitHubLabelUpdate => write!(f, "Could not update labels"),
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
