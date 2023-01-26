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
    /// Properties:  A struct of the properties to be returned in the response.
    ///     If the requested deal doesn't have a value for a property, it will not appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested deal doesn't have a value for a property, it will not appear in the response.
    ///
    /// Associations: A struct of the associations to be returned in the response.
    ///     If the requested deal doesn't have a value for a associations, it will not appear in the response.
    async fn read<Properties, PropertiesWithHistory, Associations>(
        &self,
        id: &str,
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
                "crm/v3/objects/{}/{}{}",
                self.path(),
                id,
                build_query_string(
                    serde_introspect::<Properties>(),
                    serde_introspect::<PropertiesWithHistory>(),
                    serde_introspect::<Associations>(),
                    archived
                )
            ),
        );

        println!("req: {:?}", req);

        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(req)
            .await
    }
}

fn build_query_string(
    properties: &[&str],
    properties_with_history: &[&str],
    associations: &[&str],
    archived: bool,
) -> String {
    let mut query_begun = false;

    let property_query = if properties.is_empty() {
        String::new()
    } else {
        query_begun = true;
        format!("?properties={}", properties.join(","))
    };
    let properties_with_history_query = if properties_with_history.is_empty() {
        String::new()
    } else {
        let query_starting_char = if query_begun {
            "&"
        } else {
            query_begun = true;
            "?"
        };
        format!(
            "{}propertiesWithHistory={}",
            query_starting_char,
            properties_with_history.join(",")
        )
    };
    let associations_query = if associations.is_empty() {
        String::new()
    } else {
        let query_starting_char = if query_begun {
            "&"
        } else {
            query_begun = true;
            "?"
        };
        format!(
            "{}associations={}",
            query_starting_char,
            associations.join(",")
        )
    };
    let archived_query = if query_begun {
        format!("&archived={}", archived)
    } else {
        format!("?archived={}", archived)
    };

    format!("{property_query}{properties_with_history_query}{associations_query}{archived_query}")
}

// List

// Create

// Read

// Update

// Archive
