use serde::Deserialize;

use crate::AssociationsResults;
use crate::OptionNotDesired;

use super::setup;

#[derive(Deserialize, Debug)]
pub struct DealProperties {
    pub pipeline: String,
    pub deal_regions: String,
    pub industry: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct DealAssociations {
    pub companies: AssociationsResults,
    pub contacts: AssociationsResults,
}

#[tokio::test]
async fn deal_tests() {
    let hubspot = setup::init_hubspot_for_tests();

    let deal = hubspot
        .objects
        .deals
        .read::<DealProperties, OptionNotDesired, DealAssociations>("9694916196", true)
        .await
        .expect("Unable to read deal");
}
