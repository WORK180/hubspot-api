use std::sync::Arc;

use reqwest::Method;

use crate::client::{error::HubspotResult, HubspotClient};

use self::note::NoteResponse;
pub use self::note::{Note, Properties};

mod note;

/// You can log notes on CRM objects to add information to the record timeline or associate an attachment with an object.
/// For example, if you need to keep track of an offline conversation you had with a contact, you can add a note to their contact record with details and documents related to the conversation.
/// Other users in the account will then be able to view and reference that note.
///
/// You can manage notes either in Hubspot or through the Notes Manager
#[derive(Clone)]
pub struct NotesManager(Arc<HubspotClient>);

impl NotesManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self(client)
    }

    /// Creates a new note
    pub async fn create(&self, note: Note) -> HubspotResult<NoteResponse> {
        let req = self
            .0
            .begin(Method::POST, "crm/v4/objects/notes")
            .json::<Note>(&note);

        self.0.send::<NoteResponse>(req).await
    }
}
