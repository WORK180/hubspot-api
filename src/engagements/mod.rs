use std::sync::Arc;

use crate::{api_configs::types::ObjectApi, client::HubspotClient, BasicApi};

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
    pub notes: EngagementManager,
}

impl EngagementsManager {
    pub fn new(client: Arc<HubspotClient>) -> Self {
        Self {
            notes: EngagementManager::new(EngagementType::Notes, Arc::clone(&client)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EngagementManager(EngagementType, Arc<HubspotClient>);

impl EngagementManager {
    fn new(name: EngagementType, client: Arc<HubspotClient>) -> Self {
        Self(name, client)
    }
}

impl ObjectApi<EngagementType> for EngagementManager {
    fn name(&self) -> &EngagementType {
        &self.0
    }

    fn client(&self) -> &Arc<HubspotClient> {
        &self.1
    }

    fn name_str(&self) -> String {
        self.0.to_string()
    }
}

impl BasicApi<EngagementType> for EngagementManager {}
