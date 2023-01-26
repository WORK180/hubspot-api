use std::sync::Arc;

use crate::BasicApi;
use crate::{api_configs::types::ObjectApi, client::HubspotClient};

#[derive(Clone, Debug)]
pub enum ObjectType {
    Contacts,
    Companies,
    Deals,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::Contacts => "Contacts".to_string(),
            ObjectType::Companies => "Companies".to_string(),
            ObjectType::Deals => "Deals".to_string(),
        }
    }
}

/// Objects represent types of relationships or processes.
///
/// All HubSpot accounts include four standard objects: contacts, companies, deals, and tickets.
/// Depending on your HubSpot subscription, there are additional objects, such as products and custom objects.
///
///Records are individual instances of an object (e.g., John Smith is a contact). For each record, you can store information in properties, track interactions, and create reports. You can also make associations between records to understand the relationships between them
#[derive(Clone)]
pub struct ObjectsManager {
    /// Contacts store information about an individual person.
    pub contacts: ObjectManager,
    /// Companies store information about an individual business or organization.
    pub companies: ObjectManager,
    /// Deals represent revenue opportunities with a contact or company. They’re tracked through pipeline stages, resulting in the deal being won or lost.
    pub deals: ObjectManager,
}

impl ObjectsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            contacts: ObjectManager::new(ObjectType::Contacts, Arc::clone(&client)),
            companies: ObjectManager::new(ObjectType::Companies, Arc::clone(&client)),
            deals: ObjectManager::new(ObjectType::Deals, Arc::clone(&client)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectManager(ObjectType, Arc<HubspotClient>);

impl ObjectManager {
    fn new(name: ObjectType, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }
}

impl ObjectApi<ObjectType> for ObjectManager {
    fn name(&self) -> &ObjectType {
        &self.0
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }

    fn name_str(&self) -> String {
        self.0.to_string()
    }
}

impl BasicApi<ObjectType> for ObjectManager {}
