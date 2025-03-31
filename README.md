# Hubspot API Rust Library

This unofficial Hubspot API Rust Library provides convenient access to the Hubspot CRM API from applications written in the Rust language.

## Installation

To install the Hubspot API from crates.io, add the following line to your;

_Cargo.toml_

```toml
[dependencies]
hubspot = "0.2.5"
dotenv = "0.15" # Or preferred
```

## Configuring your hubspot settings

This library utilises Hubspot Private App tokens to authenticate your requests. You can set up a Private App by following the instructions here: https://developers.hubspot.com/docs/api/private-apps

### Initializing your Hubspot client

To set up your hubspot client you will need to add the following code.

_.env_

```
HUBSPOT_API_DOMAIN=api.hubapi.com
HUBSPOT_API_KEY=<Your-private-app-token-here>
HUBSPOT_PORTAL_ID=<Your-hubspot-portal-id-here>

```

_main.rs_

```rust
 let hubspot = Hubspot::builder()
        .domain(&env::var("HUBSPOT_API_DOMAIN").expect("HUBSPOT_API_DOMAIN is not set"))
        .key(&env::var("HUBSPOT_API_KEY").expect("HUBSPOT_API_KEY is not set"))
        .portal_id(&env::var("HUBSPOT_PORTAL_ID").expect("HUBSPOT_PORTAL_ID is not set"))
        .build()
        .expect("Unable to create Hubspot configuration");

```

### Usage

Below is an example of how to read a deal by ID.

_example.rs_

```rust
use hubspot::{
    types::{AssociationResults, HubspotRecord, OptionNotDesired},
    Hubspot,
};
use serde::Deserialize;

type Deal = HubspotRecord<DealProperties, OptionNotDesired, DealAssociations>;

// This is where you specify the deal properties that will be returned by hubspot
#[derive(Deserialize, Debug)]
pub struct DealProperties {
    pub pipeline: String,
    pub deal_regions: String,
    pub industry: String,
}

// This is where you specify which objects associations you want returned by hubspot
#[derive(Deserialize, Debug, Default)]
pub struct DealAssociations {
    pub companies: Option<AssociationResults>,
    pub contacts: Option<AssociationResults>,
}

async fn get_deal_examples(hubspot: Hubspot, deal_id: &str) -> Deal {
    hubspot
        .objects
        .deals
        .read::<DealProperties, OptionNotDesired, DealAssociations>(&deal_id, false)
        .await
        .unwrap()
}

```

## Suggestions and Issues

Please open an issue on github, and we will prioritize accordingly.
