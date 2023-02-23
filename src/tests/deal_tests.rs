use serde::Deserialize;
use serde::Serialize;

use crate::api_configs::types::HubspotObjectToCreate;
use crate::AssociationsResults;
use crate::OptionNotDesired;

use super::setup;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DealProperties {
    pub hubspot_owner_id: String,
    pub deal_regions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DealPropertiesToUpdate {
    pub deal_regions: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DealAssociations {
    pub companies: AssociationsResults,
}

#[tokio::test]
async fn deal_tests() {
    let hubspot = setup::init_hubspot_for_tests();

    //let _existing_deal = hubspot
    //    .objects
    //    .deals
    //    .read::<DealProperties, OptionNotDesired, DealAssociations>("9694916196", true)
    //    .await
    //    .expect("Unable to read deal");
    //
    //let new_deal = hubspot
    //    .objects
    //    .deals
    //    .create::<DealProperties, OptionNotDesired, DealAssociations>(HubspotObjectToCreate::<
    //        DealProperties,
    //        DealAssociations,
    //    > {
    //        properties: DealProperties {
    //            hubspot_owner_id: "36743464".to_string(),
    //            deal_regions: "AU;UK".to_string(),
    //        },
    //        associations: None,
    //    })
    //    .await
    //    .expect("Unable to create deal");
    //
    let updated_deal = hubspot
        .objects
        .deals
        .update::<DealPropertiesToUpdate, DealProperties>(
            "9694916196".to_string(),
            DealPropertiesToUpdate {
                deal_regions: "AU;UK;US".to_string(),
            },
        )
        .await
        .expect("Unable to update deal");

    hubspot
        .objects
        .deals
        .list::<DealProperties, OptionNotDesired, DealAssociations>(Some(10), None, Some(false))
        .await
        .expect("Unable to get list of deals");

    //hubspot
    //    .objects
    //    .deals
    //    .archive(updated_deal.id)
    //    .await
    //    .expect("Unable to archive deal");
}
