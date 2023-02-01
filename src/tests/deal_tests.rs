use serde::Deserialize;
use serde::Serialize;

use crate::api_configs::types::HubspotObjectToCreate;
use crate::AssociationsResults;
use crate::OptionNotDesired;

use super::setup;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DealProperties {
    pub name: String,
    pub hubspot_owner_id: String,
    pub deal_regions: String,
    pub industry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DealPropertiesToUpdate {
    pub industry: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DealAssociations {
    pub companies: AssociationsResults,
}

#[tokio::test]
async fn deal_tests() {
    let hubspot = setup::init_hubspot_for_tests();

    let _existing_deal = hubspot
        .objects
        .deals
        .read::<DealProperties, OptionNotDesired, DealAssociations>("9694916196", true)
        .await
        .expect("Unable to read deal");

    let new_deal = hubspot
        .objects
        .deals
        .create::<DealProperties, OptionNotDesired, DealAssociations>(HubspotObjectToCreate::<
            DealProperties,
            DealAssociations,
        > {
            properties: DealProperties {
                name: "Engineering deal automation test".to_string(),
                hubspot_owner_id: "36743464".to_string(),
                deal_regions: "AU;UK".to_string(),
                industry: "Science & biotechnology".to_string(),
            },
            associations: None,
        })
        .await
        .expect("Unable to create deal");

    let updated_deal = hubspot.objects.deals.update::<DealPropertiesToUpdate, DealProperties>(new_deal.id, DealPropertiesToUpdate {industry: "Transport, shipping & logistics".to_string()} )
}
