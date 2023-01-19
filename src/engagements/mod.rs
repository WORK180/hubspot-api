mod notes;

use std::sync::Arc;

use crate::client::HubspotClient;

use self::notes::NotesManager;
pub use self::notes::{Note, Properties};

/// Engagements, also called activities, store data from interactions with records.
/// For example, if you call a prospect, you can log a call to the contact record, and also associate the call with their associated company. Possible activities include notes, tasks, meetings, emails, calls, postal mail, SMS, LinkedIn messages, and WhatsApp messaged.
#[derive(Clone)]
pub struct EngagementsManager {
    /// Notes add information to the record timeline.
    pub notes: NotesManager,
}

impl EngagementsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            notes: NotesManager::new(Arc::clone(&client)),
        }
    }
}
