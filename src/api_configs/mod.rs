mod associations;
mod batch;
pub mod query;
pub mod types;

use std::sync::Arc;

pub use types::AssociationsResults;
pub use types::{
    AssociationType, HubspotObject, HubspotObjectToCreate, HubspotUpdatedObject,
    KnownBuiltInAssociations,
};

use crate::client::HubspotClient;

use self::associations::AssociationsApiCollection;
use self::batch::BatchApiCollection;
use self::query::{build_paging_query, build_query_string};
use self::types::{HubspotBaseObject, ListResult, ObjectApi, ToPath};

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_aux::serde_introspection::serde_introspect;

use crate::client::error::HubspotResult;

#[derive(Clone, Debug)]
pub struct ApiCollection<T>
where
    T: ToPath,
{
    name: T,
    client: Arc<HubspotClient>,
    pub associations: AssociationsApiCollection<T>,
    pub batch: BatchApiCollection<T>,
}

impl<T> ObjectApi<T> for ApiCollection<T>
where
    T: ToPath,
{
    fn name(&self) -> &T {
        &self.name
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.client
    }
}

impl<T> ApiCollection<T>
where
    T: Clone + ToPath,
{
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self {
            name: name.clone(),
            client: Arc::clone(&client),
            associations: AssociationsApiCollection::new(name.clone(), Arc::clone(&client)),
            batch: BatchApiCollection::new(name, Arc::clone(&client)),
        }
    }

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
    pub async fn list<Properties, PropertiesWithHistory, Associations>(
        &self,
        limit: Option<i32>,
        after: Option<&str>,
        archived: Option<bool>,
    ) -> HubspotResult<ListResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>>
    where
        Properties: DeserializeOwned,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: DeserializeOwned + Default,
    {
        let paging_query = build_paging_query(limit, after);

        self.client()
            .send::<ListResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>>(
                self.client().begin(
                    Method::GET,
                    &format!(
                        "crm/v3/objects/{}{}{}",
                        self.path(),
                        paging_query.0,
                        build_query_string(
                            paging_query.1,
                            serde_introspect::<Properties>(),
                            serde_introspect::<PropertiesWithHistory>(),
                            serde_introspect::<Associations>(),
                            archived.unwrap_or(false)
                        )
                    ),
                ),
            )
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
    pub async fn create<Properties, PropertiesWithHistory, Associations>(
        &self,
        object_to_create: HubspotObjectToCreate<Properties>,
    ) -> HubspotResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: DeserializeOwned + Default + Send + Sync,
    {
        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(
                self.client()
                    .begin(Method::POST, &format!("crm/v4/objects/{}", self.path()))
                    .json::<HubspotObjectToCreate<Properties>>(&object_to_create),
            )
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
    pub async fn read<Properties, PropertiesWithHistory, Associations>(
        &self,
        id: &str,
        archived: bool,
    ) -> HubspotResult<HubspotObject<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: DeserializeOwned,
        PropertiesWithHistory: DeserializeOwned + Default,
        Associations: DeserializeOwned + Default,
    {
        self.client()
            .send::<HubspotObject<Properties, PropertiesWithHistory, Associations>>(
                self.client().begin(
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
                ),
            )
            .await
    }

    /// Updates the object for the given id.
    ///
    /// Properties:  A struct of the properties to be updated and returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not be updated or appear in the response.
    ///
    /// PropertiesWithHistory:  A struct of the properties with history to be returned in the response.
    ///     If the requested object doesn't have a value for a property, it will not appear in the response.
    pub async fn update<Properties, PropertiesWithHistory>(
        &self,
        id: String,
        properties: Properties,
    ) -> HubspotResult<HubspotUpdatedObject<Properties, PropertiesWithHistory>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync + std::fmt::Debug,
        PropertiesWithHistory: DeserializeOwned + Default,
    {
        self.client()
            .send::<HubspotUpdatedObject<Properties, PropertiesWithHistory>>(
                self.client()
                    .begin(
                        Method::PATCH,
                        &format!("crm/v3/objects/{}/{}", self.path(), id,),
                    )
                    .json::<HubspotBaseObject<Properties>>(&HubspotBaseObject::new_outbound(
                        properties,
                    )),
            )
            .await
    }

    /// Move an Object identified by id to the recycling bin.
    pub async fn archive(&self, id: String) -> HubspotResult<()> {
        self.client()
            .send(self.client().begin(
                Method::DELETE,
                &format!("crm/v3/objects/{}/{}", self.path(), id,),
            ))
            .await
    }
}
