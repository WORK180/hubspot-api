use std::{fmt::Display, sync::Arc};

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::{error::HubspotResult, HubspotClient};

use super::{
    query::build_paging_query,
    types::{ListResult, ObjectApi, ToPath},
};

#[derive(Deserialize, Debug)]
pub struct Association {
    #[serde(alias = "toObjectId")]
    pub to_object_id: String,
    #[serde(alias = "associationTypes")]
    pub association_types: AssociationTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationTypes {
    pub category: String,
    #[serde(alias = "typeId")]
    pub type_id: String,
    pub label: String,
}

#[derive(Serialize, Debug)]
pub struct AssociationCreationDetails {
    #[serde(alias = "associationCategory")]
    pub category: String,
    #[serde(alias = "associationTypeId")]
    pub type_id: AssociationTypes,
}

#[derive(Deserialize, Debug)]
pub struct CreatedAssociationResult {
    #[serde(alias = "fromObjectTypeId")]
    pub from_object_type_id: String,
    #[serde(alias = "fromObjectId")]
    pub from_object_id: String,
    #[serde(alias = "toObjectId")]
    pub to_object_id: String,
    pub labels: Vec<String>,
}

// Association Api Collection
#[derive(Clone, Debug)]
pub struct AssociationsApiCollection<T>(T, Arc<HubspotClient>);

impl<T> ObjectApi<T> for AssociationsApiCollection<T>
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

impl<T> AssociationsApiCollection<T>
where
    T: ToPath,
{
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }

    /// List all associations of a deal by object type. Limit 1000 per call.
    pub async fn list(
        &self,
        id: &str,
        to_object_type: &str,
        limit: Option<i32>,
        after: Option<&str>,
    ) -> HubspotResult<ListResult<Association>> {
        let paging_query = build_paging_query(limit, after);

        self.client()
            .send::<ListResult<Association>>(self.client().begin(
                Method::GET,
                &format!(
                    "crm/v4/objects/{}/{}/associations/{}{}",
                    self.path(),
                    id,
                    to_object_type,
                    paging_query.0
                ),
            ))
            .await
    }

    /// Set association labels between two records.
    pub async fn create<O>(
        &self,
        id: &str,
        to_object_type: O,
        to_object_id: &str,
        associations_to_create: Vec<AssociationCreationDetails>,
    ) -> HubspotResult<CreatedAssociationResult>
    where
        O: ToPath + Send,
    {
        self.client()
            .send::<CreatedAssociationResult>(
                self.client()
                    .begin(
                        Method::PUT,
                        &format!(
                            "crm/v4/objects/{}/{}/associations/{}/{}",
                            self.path(),
                            id,
                            to_object_type.to_path(),
                            to_object_id
                        ),
                    )
                    .json::<Vec<AssociationCreationDetails>>(&associations_to_create),
            )
            .await
    }

    /// Deletes all associations between two records.
    pub async fn delete<O>(
        &self,
        id: &str,
        to_object_type: O,
        to_object_id: &str,
    ) -> HubspotResult<()>
    where
        O: ToPath + Send,
    {
        self.client()
            .send(self.client().begin(
                Method::DELETE,
                &format!(
                    "crm/v4/objects/{}/{}/associations/{}/{}",
                    self.path(),
                    id,
                    to_object_type.to_path(),
                    to_object_id
                ),
            ))
            .await
    }
}
