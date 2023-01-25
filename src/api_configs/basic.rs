use async_trait::async_trait;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_aux::serde_introspection::serde_introspect;

use crate::client::error::HubspotResult;

use super::object::{HubspotObject, ObjectApi};

#[async_trait]
pub trait BasicApi: ObjectApi {
    /// Returns the object for the deal id.
    ///
    /// P:  A struct of the properties to be returned in the response.
    ///     If the requested deal doesn't have a value for a property, it will not appear in the response.
    ///
    /// H:  A struct of the properties with history to be returned in the response.
    ///     If the requested deal doesn't have a value for a property, it will not appear in the response.
    async fn read<Properties, PropertiesWithHistory, Associations>(
        &self,
        id: String,
        archived: bool,
    ) -> HubspotResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: DeserializeOwned,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: DeserializeOwned + Default,
    {
        let req = self.client().begin(
                Method::GET,
                &format!(
                    "crm/v3/objects/{}/{}?properties={}&propertiesWithHistory={}&associations={}&archived={}",
                    self.path(),
                    id,
                    serde_introspect::<Properties>().join(","),
                    serde_introspect::<PropertiesWithHistory>().join(","),
                    serde_introspect::<Associations>().join(","),
                    archived
                ),
            );

        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(req)
            .await
    }
}

// List

// Create

// Read

// Update

// Archive
