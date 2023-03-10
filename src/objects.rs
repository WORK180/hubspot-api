use std::sync::Arc;

use strum_macros::Display;

use crate::api_configs::types::ToPath;
use crate::api_configs::ApiCollection;
use crate::client::HubspotClient;

#[derive(Clone, Debug, Display)]
pub enum ObjectType {
    Contacts,
    Companies,
    Deals,
    LineItems,
}

// TODO see if we can use strum
impl ToPath for ObjectType {
    fn to_path(&self) -> String {
        match self {
            ObjectType::LineItems => "line_items".to_string(),
            object_type => object_type.to_string().to_lowercase(),
        }
    }
}

/// Objects represent types of relationships or processes.
///
/// All HubSpot accounts include four standard objects: contacts, companies, deals, and tickets.
/// Depending on your HubSpot subscription, there are additional objects, such as products and custom objects.
///
/// Records are individual instances of an object (e.g., John Smith is a contact). For each record, you can store information in properties, track interactions, and create reports. You can also make associations between records to understand the relationships between them
#[derive(Clone, Debug)]
pub struct ObjectsManager {
    /// Contacts store information about an individual person.
    pub contacts: ApiCollection<ObjectType>,
    /// Companies store information about an individual business or organization.
    pub companies: ApiCollection<ObjectType>,
    /// Deals represent revenue opportunities with a contact or company. They’re tracked through pipeline stages, resulting in the deal being won or lost.
    pub deals: ApiCollection<ObjectType>,
    /// Line items are individual instances of products. When a product is attached to a deal, it becomes a line item.
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

    pub fn get_api_collection(&self, object_type: ObjectType) -> &ApiCollection<ObjectType> {
        match object_type {
            ObjectType::Contacts => &self.contacts,
            ObjectType::Companies => &self.companies,
            ObjectType::Deals => &self.deals,
            ObjectType::LineItems => &self.line_items,
        }
    }
}
