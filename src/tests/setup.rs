use crate::{builder::HubspotBuilder, Hubspot};

#[allow(dead_code)]
pub fn init_hubspot_for_tests() -> Hubspot {
    HubspotBuilder::new()
        .domain("api.hubapi.com")
        .token("pat-na1-6946dbd1-a7f5-41f6-8c37-16ad60876ca9")
        .portal_id("1888283")
        .build()
        .expect("Unable to create Hubspot configuration")
}
