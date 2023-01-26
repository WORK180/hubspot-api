mod associations;
mod basic;
mod batch;
pub mod query;
pub mod types;

use std::fmt::Display;
use std::sync::Arc;

pub use basic::BasicApi;
pub use types::AssociationsResults;
pub use types::{HubspotObject, HubspotUpdatedObject};

use crate::client::HubspotClient;

use self::batch::BatchApi;
use self::types::ObjectApi;

#[derive(Clone, Debug)]
pub struct ApiCollection<T> {
    pub associations: AssocaitionsApiCollection<T>,
    pub basic: BasicApiCollection<T>,
    pub batch: BatchApiCollection<T>,
}

impl<T> ApiCollection<T>
where
    T: Clone,
{
    pub fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self {
            associations: AssocaitionsApiCollection::new(name.clone(), Arc::clone(&client)),
            basic: BasicApiCollection::new(name.clone(), Arc::clone(&client)),
            batch: BatchApiCollection::new(name.clone(), Arc::clone(&client)),
        }
    }
}

// Assocation Api Collection
#[derive(Clone, Debug)]
pub struct AssocaitionsApiCollection<T>(T, Arc<HubspotClient>);

impl<T> AssocaitionsApiCollection<T> {
    fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }
}

impl<T> ObjectApi<T> for AssocaitionsApiCollection<T>
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

impl<T> BasicApi<T> for AssocaitionsApiCollection<T> where T: Display {}

// Basic Api Collection
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

// Batch Api Collection
#[derive(Clone, Debug)]
pub struct BatchApiCollection<T>(T, Arc<HubspotClient>);

impl<T> BatchApiCollection<T> {
    fn new(name: T, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }
}

impl<T> ObjectApi<T> for BatchApiCollection<T>
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

impl<T> BatchApi<T> for BatchApiCollection<T> where T: Display {}
