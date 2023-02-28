use std::{collections::HashMap, sync::Arc};

use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::client::{error::HubspotResult, HubspotClient};

use super::types::{HubspotBaseObject, HubspotObject, ObjectApi, ToPath};

#[derive(Serialize, Debug)]
pub struct BatchInputs<I> {
    pub inputs: Vec<I>,
}

#[derive(Serialize, Debug)]
pub struct BatchIdInputs {
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct BatchPropertiesInputs<Properties> {
    pub properties: Properties,
}

#[derive(Serialize, Debug)]
pub struct BatchReadInputs<Properties, PropertiesWithHistory, Associations> {
    pub properties: Properties,
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    pub properties_with_history: PropertiesWithHistory,
    pub associations: Associations,
    pub archived: bool,
    pub inputs: Vec<BatchIdInputs>,
}

#[derive(Deserialize, Debug, Default)]
pub struct BatchResult<Properties, PropertiesWithHistory, Associations>
where
    PropertiesWithHistory: Default,
    Associations: Default,
{
    pub status: String,
    pub results: Vec<HubspotObject<Properties, PropertiesWithHistory, Associations>>,
    #[serde(alias = "requestedAt")]
    pub requested_at: String,
    #[serde(alias = "startedAt")]
    pub started_at: String,
    #[serde(alias = "completedAt")]
    pub completed_at: String,
    pub links: HashMap<String, String>,
}

// Batch Api Collection
#[derive(Clone, Debug)]
pub struct BatchApiCollection<T>(T, Arc<HubspotClient>);

impl<T> ObjectApi<T> for BatchApiCollection<T>
where
    T: ToPath,
{
    fn name(&self) -> &T {
        &self.0
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }
}

impl<T> BatchApiCollection<T>
where
    T: ToPath,
{
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }

    /// Archive a batch of objects by ID
    pub async fn archive(&self, ids: Vec<&str>) -> HubspotResult<()> {
        self.client()
            .send(
                self.client()
                    .begin(
                        Method::DELETE,
                        &format!("crm/v3/objects/{}/batch/archive", self.path()),
                    )
                    .json::<BatchInputs<BatchIdInputs>>(&BatchInputs::<BatchIdInputs> {
                        inputs: ids
                            .iter()
                            .map(|i| BatchIdInputs { id: i.to_string() })
                            .collect(),
                    }),
            )
            .await
    }

    /// Creates a batch of objects
    pub async fn create<Properties>(
        &self,
        objects_to_create: Vec<Properties>,
    ) -> HubspotResult<HubspotBaseObject<Properties>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync + Clone,
    {
        self.client()
            .send::<HubspotBaseObject<Properties>>(
                self.client()
                    .begin(Method::POST, &format!("crm/v4/objects/{}", self.path()))
                    .json::<BatchInputs<BatchPropertiesInputs<Properties>>>(&BatchInputs::<
                        BatchPropertiesInputs<Properties>,
                    > {
                        inputs: objects_to_create
                            .into_iter()
                            .map(|properties| BatchPropertiesInputs { properties })
                            .collect(),
                    }),
            )
            .await
    }

    /// Read a batch of objects by internal ID
    pub async fn read<Properties, PropertiesWithHistory, Associations>(
        &self,
        ids: Vec<&str>,
        properties: Properties,
        properties_with_history: PropertiesWithHistory,
        associations: Associations,
        archived: Option<bool>,
    ) -> HubspotResult<BatchResult<Properties, PropertiesWithHistory, Associations>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync,
        PropertiesWithHistory: Serialize + DeserializeOwned + Default + Send + Sync,
        Associations: Serialize + DeserializeOwned + Default + Send + Sync,
    {
        self.client()
            .send::<BatchResult<Properties, PropertiesWithHistory, Associations>>(
                self.client()
                    .begin(
                        Method::POST,
                        &format!("crm/v3/objects/{}/batch/read", self.path()),
                    )
                    .json::<BatchReadInputs<Properties, PropertiesWithHistory, Associations>>(
                        &BatchReadInputs::<Properties, PropertiesWithHistory, Associations> {
                            properties,
                            properties_with_history,
                            associations,
                            archived: archived.unwrap_or(false),
                            inputs: ids
                                .into_iter()
                                .map(|i| BatchIdInputs { id: i.to_string() })
                                .collect::<Vec<BatchIdInputs>>(),
                        },
                    ),
            )
            .await
    }

    // Update
}
