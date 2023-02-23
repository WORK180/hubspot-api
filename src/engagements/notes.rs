use serde::Serialize;
use time::OffsetDateTime;

/// Notes add information to the record timeline or associate an attachment with an object.
/// For example, if you need to keep track of an offline conversation you had with a contact,
/// you can add a note to their contact record with details and documents related to the conversation.
/// Other users in the account will then be able to view and reference that note.
#[derive(Serialize, Debug)]
pub struct Note {
    pub associations: Vec<Association>,
    pub properties: NoteProperties,
}

#[derive(Serialize, Debug)]
pub struct NoteProperties {
    /// The note's text content, limited to 65,536 characters.
    #[serde(rename = "hs_note_body")]
    pub body: String,
    /// This field marks the note's time of creation and
    /// determines where the note sits on the record timeline.
    #[serde(rename = "hs_timestamp", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}

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
    Contact,
    Company,
    Deal,
}

impl NoteProperties {
    pub fn new(body: String) -> Self {
        Self {
            body,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}

impl Note {
    pub fn new(properties: NoteProperties) -> Self {
        Self {
            properties,
            associations: Vec::new(),
        }
    }

    /// Attach multiple associations of the same ObjectType
    pub fn attach_associations(
        mut self,
        object_type: AvailableNoteAssociation,
        ids: Vec<String>,
    ) -> Self {
        for id in ids {
            self.associations.push(Association::new(id, &object_type))
        }
        self
    }
}

impl Association {
    pub fn new(id: String, object_type: &AvailableNoteAssociation) -> Association {
        Association {
            to: AssociationTo { id },
            types: vec![match object_type {
                AvailableNoteAssociation::Contact => AssociationType::hubspot_default("202"),
                AvailableNoteAssociation::Company => AssociationType::hubspot_default("190"),
                AvailableNoteAssociation::Deal => AssociationType::hubspot_default("214"),
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
