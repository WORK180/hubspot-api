use std::sync::Arc;

use crate::api_configs::object::ObjectApi;
use crate::api_configs::BasicApi;
use crate::client::HubspotClient;

/// In HubSpot, deals represent transactions with contacts or companies.
/// Deals are tracked through your sales process in pipeline stages until they're won or lost.
/// The deals manager allow you to manage create and manage deal records,
/// as well as sync deal data between HubSpot and other systems.
#[derive(Clone)]
pub struct DealsManager(String, Arc<HubspotClient>);

impl ObjectApi for DealsManager {
    fn new(client: Arc<HubspotClient>) -> Self {
        Self("Deals".to_string(), client)
    }

    fn name(&self) -> String {
        self.0.to_string()
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }
}

impl BasicApi for DealsManager {}
