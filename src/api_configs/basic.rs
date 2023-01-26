use std::fmt::Display;

use async_trait::async_trait;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_aux::serde_introspection::serde_introspect;

use crate::{client::error::HubspotResult, HubspotUpdatedObject};

use super::{
    query::{build_query_string, query_begun_check},
    types::{HubspotObject, HubspotObjectToCreate, ObjectApi},
};

#[async_trait]
pub trait BasicApi<T>: ObjectApi<T>
where
    T: Display,
{
    /// Read a page of deals. Control what is returned via the properties query param.
    ///
    /// Properties:  A struct of the properties to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// Associations: A struct of the associations to be returned in the response.
    ///     If the requested object doesn't have a value for a associations, it will not appear in the response.
    async fn list<Properties, PropertiesWithHistory, Associations>(
        &self,
        limit: Option<i32>,
        after: Option<&str>,
        archived: Option<bool>,
    ) -> HubspotResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: DeserializeOwned,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: DeserializeOwned + Default,
    {
        let mut query_begun = false;

        let limit_query = match limit {
            Some(limit) => {
                query_begun = true;
                format!("?{}", limit)
            }
            None => String::new(),
        };

        let after_query = match after {
            Some(after) => {
                let query_check = query_begun_check(query_begun);
                query_begun = query_check.1;
                format!("{}{}", query_check.0, after)
            }
            None => String::new(),
        };

        let req = self.client().begin(
            Method::GET,
            &format!(
                "crm/v3/objects/{}{}{}{}",
                self.path(),
                limit_query,
                after_query,
                build_query_string(
                    query_begun,
                    serde_introspect::<Properties>(),
                    serde_introspect::<PropertiesWithHistory>(),
                    serde_introspect::<Associations>(),
                    match archived {
                        Some(archived) => archived,
                        None => false,
                    }
                )
            ),
        );

        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(req)
            .await
    }

    /// Creates a new object
    ///
    /// Properties:  A struct of the properties to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// Associations: A struct of the associations to be returned in the response.
    ///     If the requested object doesn't have a value for a associations, it will not appear in the response.
    async fn create<Properties, PropertiesWithHistory, Associations>(
        &self,
        object_to_create: HubspotObjectToCreate<Properties, Associations>,
    ) -> HubspotResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: Serialize + DeserializeOwned + Send,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: Serialize + DeserializeOwned + Default + Send,
    {
        let req = self
            .client()
            .begin(Method::POST, &format!("crm/v4/objects/{}", self.path()))
            .json::<HubspotObjectToCreate<Properties, Associations>>(&object_to_create);

        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(req)
            .await
    }

    /// Returns the object for the id.
    ///
    /// Properties:  A struct of the properties to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    ///
    /// Associations: A struct of the associations to be returned in the response.
    ///     If the requested object doesn't have a value for a associations, it will not appear in the response.
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
                    false,
                    serde_introspect::<Properties>(),
                    serde_introspect::<PropertiesWithHistory>(),
                    serde_introspect::<Associations>(),
                    archived
                )
            ),
        );

        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(req)
            .await
    }

    /// Updates the object for the given id.
    ///
    /// Properties:  A struct of the properties to be updated and returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not be updated or appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    async fn update<Properties, PropertiesWithHistory>(
        &self,
        id: String,
        properties: Properties,
    ) -> HubspotResult<HubspotUpdatedObject<Properties, PropertiesWithHistory>>
    where
        Properties: Serialize + DeserializeOwned + Send,
        PropertiesWithHistory: DeserializeOwned + Default,
    {
        let req = self
            .client()
            .begin(
                Method::PATCH,
                &format!("crm/v3/objects/{}/{}", self.path(), id,),
            )
            .json::<Properties>(&properties);

        self.client()
            .send::<HubspotUpdatedObject<Properties, PropertiesWithHistory>>(req)
            .await
    }

    /// Move an Object identified by id to the recycling bin.
    async fn archived(&self, id: String) -> HubspotResult<()> {
        let req = self.client().begin(
            Method::DELETE,
            &format!("crm/v3/objects/{}/{}", self.path(), id,),
        );

        self.client().send(req).await
    }
}
