use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::client::HubspotClient;

pub trait ToPath {
    /// Returns the object's path for the api routes.
    fn to_path(&self) -> String;
}

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HubspotObject<Properties, PropertiesWithHistory, Associations> {
    pub id: String,
    pub properties: Properties,
    #[serde(default)]
    pub associations: Associations,
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    pub properties_with_history: PropertiesWithHistory,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

/// Hubspot Object with no associations or properties_with_history
#[derive(Serialize, Deserialize, Debug)]
pub struct HubspotBaseObject<Properties> {
    pub id: String,
    pub properties: Properties,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

impl<Properties> HubspotBaseObject<Properties> {
    pub fn new_outbound(properties: Properties) -> HubspotBaseObject<Properties> {
        HubspotBaseObject {
            properties,
            id: String::new(),
            created_at: None,
            updated_at: None,
            archived: None,
            archived_at: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct HubspotUpdatedObject<Properties, PropertiesWithHistory> {
    pub id: String,
    pub properties: Properties,
    #[serde(alias = "propertiesWithHistory")]
    #[serde(default)]
    pub properties_with_history: PropertiesWithHistory,
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    #[serde(alias = "archivedAt")]
    pub archived_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateAssociation {
    #[serde(rename = "to")]
    pub to: AssociationTo,
    pub types: Vec<AssociationType>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssociationTo {
    /// The ID of the record that you want to associate the note with.
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AssociationType {
    /// A unique identifier to indicate the association type between the note and the other object.
    /// You can retrieve the value through the associations API.
    #[serde(rename = "associationTypeId")]
    pub id: String,
    #[serde(rename = "associationCategory")]
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HubspotObjectToCreate<Properties> {
    pub properties: Properties,
    #[serde(default)]
    pub associations: Vec<CreateAssociation>,
}

impl<Properties> HubspotObjectToCreate<Properties> {
    pub fn new(properties: Properties) -> Self {
        Self {
            properties,
            associations: Vec::new(),
        }
    }

    /// Attach multiple associations of the same known built in associations
    pub fn attach_built_in_associations(
        mut self,
        association_type: KnownBuiltInAssociations,
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

/// Empty struct to represent hubspot option that is not required for a specific request.
#[derive(Deserialize, Debug, Default)]
pub struct OptionNotDesired {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssociationsResults {
    pub results: Vec<Association>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Association {
    pub id: String,
    #[serde(alias = "type")]
    pub association_type: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListResult<T> {
    pub results: Vec<T>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug, Default)]
pub struct Paging {
    pub next: PagingNext,
}

#[derive(Deserialize, Debug, Default)]
pub struct PagingNext {
    pub after: String,
    pub link: String,
}

pub enum KnownBuiltInAssociations {
    NoteToContact,
    NoteToCompany,
    NoteToDeal,
}

impl CreateAssociation {
    pub fn new_built_in(id: String, association_type: &KnownBuiltInAssociations) -> Self {
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

impl KnownBuiltInAssociations {
    pub fn build(&self) -> AssociationType {
        AssociationType {
            id: match self {
                KnownBuiltInAssociations::NoteToContact => "202".to_string(),
                KnownBuiltInAssociations::NoteToCompany => "190".to_string(),
                KnownBuiltInAssociations::NoteToDeal => "214".to_string(),
            },
            category: "HUBSPOT_DEFINED".to_string(),
        }
    }
}

impl AssociationType {
    pub fn new(id: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            category: category.to_string(),
        }
    }
}
