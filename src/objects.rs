use std::sync::Arc;

use crate::api_configs::types::ToPath;
use crate::api_configs::ApiCollection;
use crate::client::HubspotClient;

#[derive(Clone, Debug)]
pub enum ObjectType {
    Contacts,
    Companies,
    Deals,
    LineItems,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::Contacts => "Contacts".to_string(),
            ObjectType::Companies => "Companies".to_string(),
            ObjectType::Deals => "Deals".to_string(),
            ObjectType::LineItems => "Line Items".to_string(),
        }
    }
}

impl ToPath for ObjectType {
    fn to_path(&self) -> String {
        match self {
            ObjectType::Contacts => "contacts".to_string(),
            ObjectType::Companies => "companies".to_string(),
            ObjectType::Deals => "deals".to_string(),
            ObjectType::LineItems => "line_items".to_string(),
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
    pub contacts: ApiCollection<ObjectType>,
    /// Companies store information about an individual business or organization.
    pub companies: ApiCollection<ObjectType>,
    /// Deals represent revenue opportunities with a contact or company. They’re tracked through pipeline stages, resulting in the deal being won or lost.
    pub deals: ApiCollection<ObjectType>,
    pub line_items: ApiCollection<ObjectType>,
}

impl ObjectsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            contacts: ApiCollection::new(ObjectType::Contacts, Arc::clone(&client)),
            companies: ApiCollection::new(ObjectType::Companies, Arc::clone(&client)),
            deals: ApiCollection::new(ObjectType::Deals, Arc::clone(&client)),
            line_items: ApiCollection::new(ObjectType::LineItems, Arc::clone(&client)),
        }
    }
}
