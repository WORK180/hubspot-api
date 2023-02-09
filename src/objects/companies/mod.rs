use std::sync::Arc;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use serde_aux::serde_introspection::serde_introspect;

use crate::client::{error::HubspotResult, HubspotClient};

pub use self::company::Company;

mod company;

/// In HubSpot, companies store information about the organizations that interact with your business.
/// The companies manager allow you to manage create and manage company records, as well as sync company data between HubSpot
/// and other systems.
#[derive(Clone)]
pub struct CompaniesManager(Arc<HubspotClient>);

impl CompaniesManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self(client)
    }

    /// Returns the company for the given company id.
    ///
    /// P:  A struct of the properties to be returned in the response.
    ///     If the requested company doesn't have a value for a property, it will not appear in the response.
    pub async fn read<P>(&self, company_id: &str) -> HubspotResult<Company<P>>
    where
        P: DeserializeOwned,
    {
        let req = self.0.begin(
            Method::GET,
            &format!(
                "crm/v3/objects/companies/{}?properties={}",
                company_id,
                serde_introspect::<P>().join(",")
            ),
        );

        self.0.send::<Company<P>>(req).await
    }

    /// Updates the company for the given company id.
    ///
    /// P:  A struct of the properties to be updated and returned in the response.
    ///     If the requested company doesn't have a value for a property, it will not be updated or appear in the response.
    pub async fn update<P>(&self, company_id: String, properties: P) -> HubspotResult<Company<P>>
    where
        P: Serialize + DeserializeOwned,
    {
        let req = self
            .0
            .begin(
                Method::PATCH,
                &format!("crm/v3/objects/companies/{company_id}"),
            )
            .json::<Company<P>>(&Company { properties });

        self.0.send::<Company<P>>(req).await
    }
}
