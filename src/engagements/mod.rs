use std::sync::Arc;

use crate::{api_configs::ApiCollection, client::HubspotClient};

#[derive(Clone, Debug)]
pub enum EngagementType {
    Notes,
}

impl ToString for EngagementType {
    fn to_string(&self) -> String {
        match self {
            EngagementType::Notes => "Notes".to_string(),
        }
    }
}

/// Engagements, also called activities, store data from interactions with records.
/// For example, if you call a prospect, you can log a call to the contact record,
/// and also associate the call with their associated company.
/// Possible activities include notes, tasks, meetings, emails, calls, postal mail,
/// SMS, LinkedIn messages, and WhatsApp messaged.
#[derive(Clone)]
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
