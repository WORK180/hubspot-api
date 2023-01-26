use std::sync::Arc;

use crate::api_configs::object::ObjectApi;
use crate::api_configs::BasicApi;
use crate::client::HubspotClient;

/// In HubSpot, contacts store information about the individual people that interact with your business.
/// The contacts manager allow you to create and manage contact records in your HubSpot account,
/// as well as sync contact data between HubSpot and other systems.
#[derive(Clone)]
pub struct ContactsManager(String, Arc<HubspotClient>);

impl ObjectApi for ContactsManager {
    fn new(client: Arc<HubspotClient>) -> Self {
        Self("Contacts".to_string(), client)
    }

    fn name(&self) -> String {
        self.0.to_string()
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }
}

impl BasicApi for ContactsManager {}
