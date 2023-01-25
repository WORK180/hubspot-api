use std::sync::Arc;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_aux::serde_introspection::serde_introspect;

use crate::client::{error::HubspotResult, HubspotClient};

pub use self::deal::Association;
pub use self::deal::AssociationsResults;
pub use self::deal::Deal;

mod deal;
/// In HubSpot, deals represent transactions with contacts or companies.
///
/// The deals manager allow you to manage create and manage deal records,
/// as well as sync deal data between HubSpot and other systems.
#[derive(Clone)]
pub struct DealsManager(Arc<HubspotClient>);

impl DealsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self(client)
    }

    /// Returns the deal for the given deal id.
    ///
    /// A:  A struct of the associations to be returned in the response.
    ///     If the requested deal doesn't have a value for a associations, it will not appear in the response.
    ///
    /// P:  A struct of the properties to be returned in the response.
    ///     If the requested deal doesn't have a value for a property, it will not appear in the response.
    pub async fn read<A, P>(&self, deal_id: &str) -> HubspotResult<Deal<A, P>>
    where
        A: DeserializeOwned,
        P: DeserializeOwned,
    {
        let req = self.0.begin(
            Method::GET,
            &format!(
                "crm/v3/objects/deals/{}?properties={}&associations={}",
                deal_id,
                serde_introspect::<P>().join(","),
                serde_introspect::<A>().join(",")
            ),
        );

        self.0.send::<Deal<A, P>>(req).await
    }
}
