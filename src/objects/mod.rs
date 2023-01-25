mod companies;
mod contacts;
mod deals;

use std::sync::Arc;

use crate::client::HubspotClient;

use self::companies::CompaniesManager;
pub use self::companies::Company;

pub use self::contacts::Contact;
use self::contacts::ContactsManager;

use self::deals::DealsManager;
pub use self::deals::{
    Association as DealAssociations, AssociationsResults as DealAssociationsResults, Deal,
};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum ObjectType {
    Contact,
    Company,
    Deal,
}

/// Objects represent types of relationships or processes.
///
/// All HubSpot accounts include four standard objects: 
/// contacts, companies, deals, and tickets.
/// Depending on your HubSpot subscription, there are additional objects, 
/// such as products and custom objects.
///
/// Records are individual instances of an object (e.g., John Smith is a contact). 
/// For each record, you can store information in properties, track interactions, 
/// and create reports. You can also make associations between records to understand 
/// the relationships between them
#[derive(Clone)]
pub struct ObjectsManager {
    /// Contacts store information about an individual person.
    pub contacts: ContactsManager,
    /// Companies store information about an individual business or organization.
    pub companies: CompaniesManager,
    /// Deals represent revenue opportunities with a contact or company. 
    /// They’re tracked through pipeline stages, resulting in the deal being won or lost.
    pub deals: DealsManager,
}

impl ObjectsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            contacts: ContactsManager::new(Arc::clone(&client)),
            companies: CompaniesManager::new(Arc::clone(&client)),
            deals: DealsManager::new(Arc::clone(&client)),
        }
    }
}
