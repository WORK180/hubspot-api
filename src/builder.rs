use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;

use crate::Hubspot;

use super::client::HubspotClient;

/// Builder for constructing a [`Hubspot`] client.
///
/// Obtain via [`Hubspot::builder()`]. All three fields — [`domain`](HubspotBuilder::domain),
/// [`token`](HubspotBuilder::token), and [`portal_id`](HubspotBuilder::portal_id) — are required.
#[derive(Default)]
pub struct HubspotBuilder {
    client: Option<Client>,
    domain: Option<String>,
    token: Option<String>,
    portal_id: Option<String>,
}

impl HubspotBuilder {
    /// Create an instance of the HubSpot API builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Build the Hubspot API Library.
    ///
    /// Valid builder options requires all fields to be populated.
    pub fn build(&self) -> Result<Hubspot, HubspotBuilderError> {
        let domain = self
            .domain
            .as_ref()
            .ok_or(HubspotBuilderError::MissingDomain)?;
        let token = self
            .token
            .as_ref()
            .ok_or(HubspotBuilderError::MissingToken)?;
        let portal_id = self
            .portal_id
            .as_ref()
            .ok_or(HubspotBuilderError::MissingPortalId)?;
        let client = match &self.client {
            Some(client) => client.to_owned(),
            None => Client::new(),
        };

        let client = HubspotClient::new(client, domain, token, portal_id);

        Ok(Hubspot::new(client))
    }

    /// The hubspot domain.
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_owned());
        self
    }

    /// The HubSpot Private App access token used to authenticate requests.
    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());
        self
    }

    /// The HubSpot portal (account) ID. Used to scope and validate requests.
    pub fn portal_id(mut self, portal_id: &str) -> Self {
        self.portal_id = Some(portal_id.to_owned());
        self
    }

    /// An optional custom [`reqwest::Client`] to use for HTTP requests.
    /// If not provided, a default client is created.
    pub fn client(mut self, client: &Client) -> Self {
        self.client = Some(client.to_owned());
        self
    }
}

/// The error type which is returned from building a [Hubspot].
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum HubspotBuilderError {
    /// Indicates builder didn't set [HubspotBuilder::domain].
    MissingDomain,
    /// Indicates builder didn't set [HubspotBuilder::token].
    MissingToken,
    /// Indicates builder didn't set [HubspotBuilder::portal_id].
    MissingPortalId,
}

impl Display for HubspotBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for HubspotBuilderError {}
