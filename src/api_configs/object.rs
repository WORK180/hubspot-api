use serde::Deserialize;
use std::sync::Arc;

use crate::client::HubspotClient;

pub trait ObjectApi {
    /// Constructor
    fn new(object_name: &str, client: Arc<HubspotClient>) -> Self;

    /// Function to get the object's name.
    fn name(&self) -> String;

    /// Returns the object's path for the api routes.
    fn path(&self) -> String {
        self.name().to_lowercase()
    }

    fn client(&self) -> &Arc<HubspotClient>;
}

#[derive(Deserialize, Debug)]
pub struct HubspotObject<Properties, PropertiesWithHistory, Associations> {
    pub id: String,
    pub properties: Properties,
    #[serde(default)]
    pub associations: Associations,
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    pub properties_with_history: PropertiesWithHistory,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

/// Empty struct to represent hubspot option that is not required for a specific request.
#[derive(Deserialize, Debug, Default)]
pub struct OptionNotDesired {}

#[derive(Deserialize, Debug, Default)]
pub struct AssociationsResults {
    pub results: Vec<Association>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Association {
    pub id: String,
    #[serde(alias = "type")]
    pub association_type: String,
}
