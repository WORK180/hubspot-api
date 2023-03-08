use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::client::HubspotClient;

/// ToPath trait represents a Hubspot object's path.
pub trait ToPath {
    /// Returns the object's path for the api routes.
    fn to_path(&self) -> String;
}

/// The common functionality for all objects within the Hubspot api.
pub trait ObjectApi<T>
where
    T: ToPath,
{
    /// Function to get the object's name.
    fn name(&self) -> &T;

    /// Returns the object's path for the api routes.
    fn path(&self) -> String {
        self.name().to_path()
    }

    fn client(&self) -> &Arc<HubspotClient>;
}

/// A representation of a generic Hubspot record. Regardless of object type.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HubspotRecord<Properties, PropertiesWithHistory, Associations> {
    /// The record's ID.
    pub id: String,
    /// The requested properties for the record.
    pub properties: Properties,
    #[serde(default)]
    /// The requested associations for the record
    pub associations: Associations,
    /// The requested properties with history for the record
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    pub properties_with_history: PropertiesWithHistory,
    /// The dateTime that the record was created.
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    /// The dateTime that the record was updated.
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    /// Whether or not the record is archived.
    pub archived: Option<bool>,
    /// The dateTime that the record was archived.
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

/// Implementation of HubspotRecord where only Properties are required.
impl<Properties> HubspotRecord<Properties, OptionNotDesired, OptionNotDesired> {
    /// Create a new HubspotRecord with the given properties and default values for all other fields.
    /// Suggested use for the Core Update endpoint.
    pub fn with_properties(
        properties: Properties,
    ) -> HubspotRecord<Properties, OptionNotDesired, OptionNotDesired> {
        HubspotRecord {
            properties,
            id: String::new(),
            created_at: None,
            updated_at: None,
            archived: None,
            archived_at: None,
            associations: OptionNotDesired::default(),
            properties_with_history: OptionNotDesired::default(),
        }
    }
}

/// Implementation of HubspotRecord where Properties and AssociationsToCreate are required.
/// Recommended use for the base create endpoint.
impl<Properties> HubspotRecord<Properties, OptionNotDesired, Vec<CreateAssociation>> {
    /// Create a new HubspotRecord with the given properties. Initializes a vec for associations to create. Default values for all other fields.
    /// Suggested use for the Core New endpoint.
    pub fn with_properties_and_associations(
        properties: Properties,
    ) -> HubspotRecord<Properties, OptionNotDesired, Vec<CreateAssociation>> {
        HubspotRecord {
            properties,
            id: String::new(),
            created_at: None,
            updated_at: None,
            archived: None,
            archived_at: None,
            associations: Vec::new(),
            properties_with_history: OptionNotDesired::default(),
        }
    }

    /// Attach multiple associations of the same known built in associations
    pub fn attach_built_in_associations(
        mut self,
        association_type: AssociationLinks,
        ids: Vec<String>,
    ) -> Self {
        for id in ids {
            self.associations
                .push(CreateAssociation::new_built_in(id, &association_type))
        }
        self
    }

    /// Attach multiple associations of the same custom association type
    pub fn attach_associations(
        mut self,
        association_type: AssociationType,
        ids: Vec<String>,
    ) -> Self {
        for id in ids {
            self.associations
                .push(CreateAssociation::new(id, &association_type))
        }
        self
    }
}

/// The struct to create a new association between two records.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateAssociation {
    #[serde(rename = "to")]
    pub to: AssociationTo,
    pub types: Vec<AssociationType>,
}

/// The struct for the record to associate a record with.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssociationTo {
    /// The ID of the record that you want to associate the note with.
    pub id: String,
}

/// The association type a new association should be.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AssociationType {
    /// A unique identifier to indicate the association type between the note and the other object.
    /// You can retrieve the value through the associations API.
    #[serde(rename = "associationTypeId")]
    pub id: String,
    // Whether the association type was created by HubSpot or a user (HUBSPOT_DEFINED and USER_DEFINED)
    #[serde(rename = "associationCategory")]
    pub category: String,
}

/// Empty struct to represent hubspot option that is not required for a specific request.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OptionNotDesired {}

/// A list of association results
/// Recommended use when creating the Association Struct.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssociationResults {
    pub results: Vec<Association>,
}

/// An representation of an association as returned by Hubspot
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Association {
    pub id: String,
    #[serde(alias = "type")]
    pub association_type: String,
}

/// A paged result type.
#[derive(Deserialize, Debug, Default)]
pub struct ListResult<T> {
    /// A Vec of the results.
    pub results: Vec<T>,
    /// Paging information.
    pub paging: Paging,
}

/// Paging information
#[derive(Deserialize, Debug, Default)]
pub struct Paging {
    /// The next page
    pub next: PagingNext,
}

#[derive(Deserialize, Debug, Default)]
pub struct PagingNext {
    pub after: String,
    pub link: String,
}

/// An enum of Built In Hubspot Associations.
/// To be built upon in the future.
pub enum AssociationLinks {
    NoteToContact,
    NoteToCompany,
    NoteToDeal,
}

/// Implementation of CreateAssociation
impl CreateAssociation {
    /// Create a new association using the AssociationLinks
    pub fn new_built_in(id: String, association_type: &AssociationLinks) -> Self {
        Self {
            to: AssociationTo { id },
            types: vec![association_type.build()],
        }
    }

    pub fn new(id: String, association_type: &AssociationType) -> Self {
        Self {
            to: AssociationTo { id },
            types: vec![association_type.clone()],
        }
    }
}

/// Implementation of AssociationLinks
impl AssociationLinks {
    /// Build a new AssociationType from the given AssociationLinks
    pub fn build(&self) -> AssociationType {
        AssociationType {
            id: match self {
                AssociationLinks::NoteToContact => "202".to_string(),
                AssociationLinks::NoteToCompany => "190".to_string(),
                AssociationLinks::NoteToDeal => "214".to_string(),
            },
            category: "HUBSPOT_DEFINED".to_string(),
        }
    }
}

/// Implementation of AssociationType
impl AssociationType {
    /// Constructs a new AssociationType
    pub fn new(id: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            category: category.to_string(),
        }
    }
}
