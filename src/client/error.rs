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

/// Hubspot error response.
#[derive(Deserialize, Debug)]
pub struct HubspotErrorResponse {
    pub message: String,
    pub context: HubspotErrorContext,
    pub category: String,
}

#[derive(Deserialize, Debug)]
pub struct HubspotErrorContext {
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
