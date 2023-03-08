use std::{collections::HashMap, sync::Arc};

use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::client::{error::HubspotResult, HubspotClient};

use super::types::{HubspotRecord, ObjectApi, OptionNotDesired, ToPath};

/// A wrapper type for batch inputs.
#[derive(Serialize, Debug)]
struct BatchInputs<I> {
    /// The inputs for a Batch request.
    inputs: Vec<I>,
}

impl<I> BatchInputs<I> {
    /// Constructs a new BatchInput
    pub fn new(inputs: Vec<I>) -> BatchInputs<I> {
        BatchInputs { inputs }
    }
}

/// A struct of record Ids for the update batch api.
/// eg. Batch update.
#[derive(Serialize, Debug)]
struct BatchUpdateInput<Properties> {
    /// Hubspot record Ids for a batch request.
    id: String,
    /// The property inputs for a batch request
    properties: Properties,
}

impl<Properties: Clone> BatchUpdateInput<Properties> {
    /// Constructs a new BatchUpdateInput
    pub fn new(id: &str, properties: Properties) -> BatchUpdateInput<Properties> {
        BatchUpdateInput {
            id: id.to_string(),
            properties,
        }
    }

    /// Constructs a new vec of BatchUpdateInputs from a list of record IDs.
    pub fn new_batch(
        ids: Vec<String>,
        properties: Properties,
    ) -> Vec<BatchUpdateInput<Properties>> {
        ids.iter()
            .map(|id| BatchUpdateInput::new(id, properties.clone()))
            .collect()
    }
}

/// A struct of record Ids for the batch api.
/// eg. Batch read.
#[derive(Serialize, Debug)]
struct BatchIdInput {
    /// Hubspot record Ids for a batch request.
    id: String,
}

/// A wrapper type for batch properties inputs
#[derive(Serialize, Debug)]
struct BatchPropertiesInput<Properties> {
    /// The property inputs for a batch request
    properties: Properties,
}

/// The required inputs for a Batch Read request.
#[derive(Serialize, Debug)]
struct BatchReadInputs<Properties, PropertiesWithHistory, Associations> {
    /// The record ids to return for a batch request.
    inputs: Vec<BatchIdInput>,
    /// The record properties for a batch request
    properties: Properties,
    /// The record properties with history for a batch request
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    properties_with_history: PropertiesWithHistory,
    /// The record associations for a batch request
    associations: Associations,
    /// Whether to return only results that have been archived.
    archived: bool,
}

/// A Hubspot result type for a batch request.
#[derive(Deserialize, Debug, Default)]
struct BatchResult<Properties, PropertiesWithHistory, Associations>
where
    PropertiesWithHistory: Default,
    Associations: Default,
{
    /// The status result of the batch request.
    status: String,
    /// The result objects of the batch request.
    results: Vec<HubspotRecord<Properties, PropertiesWithHistory, Associations>>,
    #[serde(alias = "requestedAt")]
    /// The time the batch request was requested.
    requested_at: String,
    /// The time the batch request started.
    #[serde(alias = "startedAt")]
    started_at: String,
    /// The time the batch request was completed at.
    #[serde(alias = "completedAt")]
    completed_at: String,
    /// Links for the batch request.
    links: HashMap<String, String>,
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
    /// Constructs a new BatchApiCollection for a Hubspot Object
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
                    .json::<BatchInputs<BatchIdInput>>(&BatchInputs::<BatchIdInput> {
                        inputs: ids
                            .iter()
                            .map(|i| BatchIdInput { id: i.to_string() })
                            .collect(),
                    }),
            )
            .await
    }

    /// Creates a batch of objects
    pub async fn create<Properties>(
        &self,
        objects_to_create: Vec<Properties>,
    ) -> HubspotResult<HubspotRecord<Properties, OptionNotDesired, OptionNotDesired>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync + Clone,
    {
        self.client()
            .send::<HubspotRecord<Properties, OptionNotDesired, OptionNotDesired>>(
                self.client()
                    .begin(Method::POST, &format!("crm/v4/objects/{}", self.path()))
                    .json::<BatchInputs<BatchPropertiesInput<Properties>>>(&BatchInputs::<
                        BatchPropertiesInput<Properties>,
                    > {
                        inputs: objects_to_create
                            .into_iter()
                            .map(|properties| BatchPropertiesInput { properties })
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
                                .map(|i| BatchIdInput { id: i.to_string() })
                                .collect::<Vec<BatchIdInput>>(),
                        },
                    ),
            )
            .await
    }

    // Update a batch of objects
    pub async fn update<Properties, PropertiesWithHistory>(
        &self,
        ids: Vec<String>,
        properties: Properties,
    ) -> HubspotResult<BatchResult<Properties, PropertiesWithHistory, OptionNotDesired>>
    where
        Properties: Serialize + DeserializeOwned + Send + Sync + Clone,
        PropertiesWithHistory: DeserializeOwned + Default,
    {
        self.client()
            .send::<BatchResult<Properties, PropertiesWithHistory, OptionNotDesired>>(
                self.client()
                    .begin(
                        Method::PATCH,
                        &format!("/crm/v3/objects/{}/batch/update", self.path()),
                    )
                    .json::<BatchInputs<BatchUpdateInput<Properties>>>(&BatchInputs::new(
                        BatchUpdateInput::new_batch(ids, properties),
                    )),
            )
            .await
    }
}
