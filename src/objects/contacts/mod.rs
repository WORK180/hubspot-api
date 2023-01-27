use std::sync::Arc;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_aux::prelude::*;

use crate::client::{error::HubspotResult, HubspotClient};

pub use self::contact::Contact;

mod contact;

/// In HubSpot, contacts store information about the individual people that interact with your business.
/// The contacts manager allow you to create and manage contact records in your HubSpot account,
/// as well as sync contact data between HubSpot and other systems.
#[derive(Clone)]
pub struct ContactsManager(Arc<HubspotClient>);

impl ContactsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self(client)
    }

    /// Returns the contact for the given contact id.
    ///
    /// P:  A struct of the properties to be returned in the response.
    ///     If the requested contact doesn't have a value for a property, it will not appear in the response.
    pub async fn read<P>(&self, contact_id: &str) -> HubspotResult<Contact<P>>
    where
        P: DeserializeOwned,
    {
        let req = self.0.begin(
            Method::GET,
            &format!(
                "crm/v3/objects/contacts/{}?properties={}",
                contact_id,
                serde_introspect::<P>().join(","),
            ),
        );

        self.0.send::<Contact<P>>(req).await
    }
}
