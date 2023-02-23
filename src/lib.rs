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

pub use engagements::EngagementType;
pub use objects::ObjectType;

pub use crate::api_configs::{
    types::OptionNotDesired, AssociationsResults, HubspotObject, HubspotUpdatedObject,
};

// A Rust implementation of the Hubspot CRM API
#[derive(Clone)]
pub struct Hubspot {
    pub portal_id: String,
    /// Objects represent types of relationships or processes.
    pub objects: ObjectsManager,
    /// Engagements store data from interactions with records.
    pub engagements: EngagementsManager,
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
        }
    }

    /// Create Hubspot client
    pub fn builder() -> HubspotBuilder {
        HubspotBuilder::new()
    }
}
