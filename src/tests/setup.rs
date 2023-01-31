use std::env;

use crate::{builder::HubspotBuilder, Hubspot};

pub fn init_hubspot_for_tests() -> Hubspot {
    HubspotBuilder::new()
        .domain(&env::var("HUBSPOT_API_DOMAIN").expect("HUBSPOT_API_DOMAIN is not set"))
        .token(&env::var("HUBSPOT_API_KEY").expect("HUBSPOT_API_KEY is not set"))
        .portal_id(&env::var("HUBSPOT_PORTAL_ID").expect("HUBSPOT_PORTAL_ID is not set"))
        .build()
        .expect("Unable to create Hubspot configuration")
}
