use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize, Debug)]
pub struct UpdateNotesProperty {
    /// The note's text content, limited to 65,536 characters.
    #[serde(rename = "hs_note_body")]
    pub body: String,
    /// This field marks the note's time of creation and
    /// determines where the note sits on the record timeline.
    #[serde(rename = "hs_timestamp", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    /// The contact id of the contact this note is owned by.
    pub hubspot_owner_id: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Association {
    #[serde(rename = "to")]
    pub to: AssociationTo,
    pub types: Vec<AssociationType>,
}

#[derive(Serialize, Debug)]
pub struct AssociationTo {
    /// The ID of the record that you want to associate the note with.
    pub id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct AssociationType {
    /// A unique identifier to indicate the association type between the note and the other object.
    /// You can retrieve the value through the associations API.
    #[serde(rename = "associationTypeId")]
    pub id: String,
    #[serde(rename = "associationCategory")]
    pub category: String,
}

pub enum AvailableNoteAssociation {
    Contacts,
    Companies,
    Deals,
}

impl Association {
    pub fn new(id: String, object_type: &AvailableNoteAssociation) -> Association {
        Association {
            to: AssociationTo { id },
            types: vec![match object_type {
                AvailableNoteAssociation::Contacts => AssociationType::hubspot_default("202"),
                AvailableNoteAssociation::Companies => AssociationType::hubspot_default("190"),
                AvailableNoteAssociation::Deals => AssociationType::hubspot_default("214"),
            }],
        }
    }
}

impl AssociationType {
    pub fn hubspot_default(id: &str) -> Self {
        Self {
            id: id.to_string(),
            category: "HUBSPOT_DEFINED".to_string(),
        }
    }
}
