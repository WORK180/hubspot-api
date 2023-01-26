mod basic;
pub mod query;
pub mod types;

use std::fmt::Display;
use std::sync::Arc;

pub use basic::BasicApi;
pub use types::AssociationsResults;
pub use types::{HubspotObject, HubspotUpdatedObject};

use crate::client::HubspotClient;

use self::types::ObjectApi;

#[derive(Clone, Debug)]
pub struct ApiCollection<T> {
    pub basic: BasicApiCollection<T>,
}

impl<T> ApiCollection<T> {
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self {
            basic: BasicApiCollection::new(name, client),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BasicApiCollection<T>(T, Arc<HubspotClient>);

impl<T> BasicApiCollection<T> {
    fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }
}

impl<T> ObjectApi<T> for BasicApiCollection<T>
where
    T: Display,
{
    fn name(&self) -> &T {
        &self.0
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }
}

impl<T> BasicApi<T> for BasicApiCollection<T> where T: Display {}
