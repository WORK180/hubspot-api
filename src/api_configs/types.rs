use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::client::HubspotClient;

pub trait ToPath {
    /// Returns the object's path for the api routes.
    fn to_path(&self) -> String;
}

pub trait ObjectApi<T>
where
    T: ToPath,
{
    /// Function to get the object's name.
    fn name(&self) -> &T;

    /// Returns the object's path for the api routes.
    fn path(&self) -> String {
        self.name().to_path()
    }

    fn client(&self) -> &Arc<HubspotClient>;
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug)]
pub struct HubspotCreatedObject<Properties> {
    pub id: String,
    pub properties: Properties,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HubspotUpdatedObject<Properties, PropertiesWithHistory> {
    pub id: String,
    pub properties: Properties,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct HubspotObjectToCreate<Properties, Associations> {
    pub properties: Properties,
    #[serde(default)]
    pub associations: Option<Associations>,
}

/// Empty struct to represent hubspot option that is not required for a specific request.
#[derive(Deserialize, Debug, Default)]
pub struct OptionNotDesired {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssociationsResults {
    pub results: Vec<Association>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Association {
    pub id: String,
    #[serde(alias = "type")]
    pub association_type: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListResult<T> {
    pub results: Vec<T>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug, Default)]
pub struct Paging {
    pub next: PagingNext,
}

#[derive(Deserialize, Debug, Default)]
pub struct PagingNext {
    pub after: String,
    pub link: String,
}
