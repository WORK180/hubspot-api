use std::sync::Arc;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::{error::HubspotResult, HubspotClient};

use super::{
    query::build_paging_query,
    types::{ListResult, ObjectApi, ToPath},
};

/// An association linking an object `to_object_id` to the parent object.
#[derive(Deserialize, Debug)]
pub struct Association {
    /// The id of the associated record.
    #[serde(alias = "toObjectId")]
    pub to_object_id: String,
    /// The association type to reflect the relationship between the two records.
    #[serde(alias = "associationTypes")]
    pub association_types: AssociationTypes,
}

/// An association type that represents the relationship between two records.
#[derive(Serialize, Deserialize, Debug)]
pub struct AssociationTypes {
    // Whether the association type was created by HubSpot or a user (HUBSPOT_DEFINED and USER_DEFINED)
    pub category: String,
    /// The numeric ID for that association type.
    #[serde(alias = "typeId")]
    pub type_id: String,
    /// Association labels describe relationships between all standard CRM objects
    pub label: String,
}

/// A struct for creating new associations.
#[derive(Serialize, Debug)]
pub struct AssociationCreationDetails {
    /// Whether the association type was created by HubSpot or a user (HUBSPOT_DEFINED and USER_DEFINED)
    #[serde(alias = "associationCategory")]
    pub category: String,
    /// The numeric ID for that association type.
    #[serde(alias = "associationTypeId")]
    pub type_id: AssociationTypes,
}

/// A  Hubspot result type for a created association.
#[derive(Deserialize, Debug)]
pub struct CreatedAssociationResult {
    /// The type of the object you're associating (e.g. contact).
    #[serde(alias = "fromObjectTypeId")]
    pub from_object_type_id: String,
    /// The ID of the record to associate.
    #[serde(alias = "fromObjectId")]
    pub from_object_id: String,
    /// The type of object you're associating the record to (e.g. company).
    #[serde(alias = "toObjectId")]
    pub to_object_id: String,
    /// Association labels describe relationships between all standard CRM objects
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
    /// Constructs a new AssociationsApiCollection for an object type.
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }

    /// List all associations of a record by object type. Limit 1000 per call.
    pub async fn list(
        &self,
        // The ID of the record retrieve the associations for.
        id: &str,
        // The type of object to retrieve associations for.
        to_object_type: &str,
        // The maximum number of results to display per page.
        limit: Option<i32>,
        // The paging cursor token of the last successfully read resource will be returned as the paging.next.after JSON property of a paged response containing more results.
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
