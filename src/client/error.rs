//! Error type for hubspot requests.
use serde_json::Error as JsonError;
use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::Deserialize;

/// Hubspot result type
pub type HubspotResult<T> = Result<T, HubspotError>;

/// The error returned when querying Hubspot.
#[derive(Debug)]
pub enum HubspotError {
    /// Json error
    Json(JsonError),
    /// Generic http error.
    Http(reqwest::Error),
    /// Hubspot server side error.
    Hubspot(String),
}

impl Display for HubspotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for HubspotError {}

impl From<JsonError> for HubspotError {
    fn from(err: JsonError) -> Self {
        HubspotError::Json(err)
    }
}

impl From<reqwest::Error> for HubspotError {
    fn from(inner: reqwest::Error) -> Self {
        HubspotError::Http(inner)
    }
}

/// The error response body returned by HubSpot on 4xx/5xx responses.
#[derive(Deserialize, Debug)]
pub struct HubspotErrorResponse {
    /// A human-readable description of the error.
    pub message: String,
    /// Additional context about which properties caused the error.
    pub context: HubspotErrorContext,
    /// The HubSpot error category (e.g. `VALIDATION_ERROR`).
    pub category: String,
}

/// Contextual detail about which properties triggered a HubSpot error.
#[derive(Deserialize, Debug)]
pub struct HubspotErrorContext {
    /// The property names that caused the error.
    pub properties: Vec<String>,
}

impl From<HubspotErrorResponse> for HubspotError {
    fn from(inner: HubspotErrorResponse) -> Self {
        HubspotError::Hubspot(format!(
            "{}: {}, {:?}",
            inner.category, inner.message, inner.context
        ))
    }
}
