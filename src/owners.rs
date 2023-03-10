use std::sync::Arc;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::client::HubspotClient;

use reqwest::Method;

use crate::client::error::HubspotResult;

/// HubSpot uses owners to assign specific users to contacts, companies, deals, tickets,
/// or engagements. Any HubSpot user with access to contacts can be assigned as an owner,
/// and multiple owners can be assigned to an object by creating a custom property for
/// this purpose. Owners can only be created in HubSpot, but you can use the owners endpoints
///  to get their identifying details, including IDs and email addresses. This data can
/// then be assigned to CRM records in HubSpot or via property change API calls.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    /// Owner Identifier
    pub id: String,
    /// Owner email address
    pub email: String,
    /// Owner first name
    pub first_name: String,
    /// Owner last name
    pub last_name: String,
    // The user ID of the owner
    pub user_id: i64,
    /// The date the owner was created
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    // The date the owner was last updated
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    /// Whether or not the owner has been archived
    pub archived: bool,
    /// If teams are available for your HubSpot tier,
    /// this indicates which team(s) the owner can access.
    pub teams: Option<Vec<Team>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Team {
    /// The team identifier
    pub id: String,
    /// The name of the team
    pub name: String,
    /// If the team is the owner's primary team
    pub primary: bool,
}

/// The endpoints described here are used to get a list of the owners
/// that are available  for an account. To assign an owner to an object,
/// set the hubspot_owner_id property using the appropriate CRM object
/// update or create a request.
#[derive(Clone, Debug)]
pub struct OwnerApi {
    client: Arc<HubspotClient>,
}

/// Implementation of Hubspot's Owner Api
impl OwnerApi {
    /// Construct a new Owner API collection.
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            client: Arc::clone(&client),
        }
    }

    /// Returns the owner for the given ID.
    pub async fn read(&self, id: &str, archived: bool) -> HubspotResult<Owner> {
        self.client
            .send::<Owner>(self.client.begin(
                Method::GET,
                &format!("crm/v3/owners/{}?archived={}", id, archived),
            ))
            .await
    }
}
