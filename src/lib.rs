use std::sync::Arc;

use builder::HubspotBuilder;
use client::HubspotClient;
use engagements::EngagementsManager;
use objects::ObjectsManager;

mod api_configs;
mod builder;
mod client;
mod engagements;
mod objects;
mod owners;

pub mod associations {
    pub use super::api_configs::{AssociationCreationDetails, AssociationTypes};
}

pub use api_configs::types;
pub use client::error::{HubspotError, HubspotResult};
pub use engagements::notes;
pub use engagements::EngagementType;
pub use objects::ObjectType;
use owners::OwnerApi;

/// An unofficial async Rust client for the [HubSpot CRM API](https://developers.hubspot.com/docs/guides/crm/understanding-the-crm).
///
/// Construct via [`Hubspot::builder()`].
#[derive(Clone, Debug)]
pub struct Hubspot {
    /// The HubSpot portal (account) ID.
    pub portal_id: String,
    /// Objects represent types of relationships or processes.
    pub objects: ObjectsManager,
    /// Engagements store data from interactions with records.
    pub engagements: EngagementsManager,
    /// Owners are specific users assigned to contacts, companies, deals, tickets, or engagements.
    pub owners: OwnerApi,
}

impl Hubspot {
    /// Create hubspot api
    pub fn new(client: HubspotClient) -> Self {
        let portal_id = client.portal_id.clone();
        let client = Arc::new(client);

        Self {
            portal_id,
            objects: ObjectsManager::new(Arc::clone(&client)),
            engagements: EngagementsManager::new(Arc::clone(&client)),
            owners: OwnerApi::new(Arc::clone(&client)),
        }
    }

    /// Returns a builder for constructing a [`Hubspot`] client.
    pub fn builder() -> HubspotBuilder {
        HubspotBuilder::new()
    }
}
