pub mod notes;

use std::sync::Arc;

use strum_macros::Display;

use crate::{
    api_configs::{types::ToPath, ApiCollection},
    client::HubspotClient,
};

#[derive(Clone, Debug, Display)]
pub enum EngagementType {
    Notes,
}

impl ToPath for EngagementType {
    fn to_path(&self) -> String {
        self.to_string().to_lowercase()
    }
}

/// Engagements, also called activities, store data from interactions with records.
/// For example, if you call a prospect, you can log a call to the contact record,
/// and also associate the call with their associated company.
/// Possible activities include notes, tasks, meetings, emails, calls, postal mail,
/// SMS, LinkedIn messages, and WhatsApp messaged.
#[derive(Clone, Debug)]
pub struct EngagementsManager {
    /// Notes add information to the record timeline.
    pub notes: ApiCollection<EngagementType>,
}

impl EngagementsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            notes: ApiCollection::<EngagementType>::new(EngagementType::Notes, Arc::clone(&client)),
        }
    }
}
