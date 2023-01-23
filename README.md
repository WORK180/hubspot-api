# Hubspot API Rust Library
The unofficial Hubspot API Rust Library provides convenient access to the Hubspot CRM API from applications written in the Rust language.

## Installation
To install the Hubspot API from crates.io. Add the following line to your `Cargo.toml`
```
[dependencies]
hubspot = "0.1.0"
dotenv = "0.15" # Or preferred
```

## Configuring your hubspot settings
This library utilises Hubspot Private App tokens to authenticate your requests. You can set up a Private App by following the instructions here: https://developers.hubspot.com/docs/api/private-apps

### Initializing your Hubspot client
To set up your hubspot client you will need to add the following code.

.env
```
HUBSPOT_API_DOMAIN=api.hubapi.com
HUBSPOT_API_KEY=<Your-private-app-token-here>
HUBSPOT_PORTAL_ID=<Your-hubspot-portal-id-here>

```

main.rs
```
 let hubspot = Hubspot::builder()
        .domain(&env::var("HUBSPOT_API_DOMAIN").expect("HUBSPOT_API_DOMAIN is not set"))
        .key(&env::var("HUBSPOT_API_KEY").expect("HUBSPOT_API_KEY is not set"))
        .portal_id(&env::var("HUBSPOT_PORTAL_ID").expect("HUBSPOT_PORTAL_ID is not set"))
        .build()
        .expect("Unable to create Hubspot configuration");

```

### Usage
Below is an example of how to read a deal by id.

example.rs

```
use std::io::Result;
use hubspot::{
    Deal, DealAssociationsResults as AssociationsResults, Hubspot
};

// This is where you specify the deal properties that will be returned by hubspot
#[derive(Deserialize, Debug)]
pub struct DealProperties {
    pub pipeline: String,
    pub deal_regions: String,
    pub industry: String,
}

// This is where you specify which objects associations you want returned by hubspot
#[derive(Deserialize, Debug)]
pub struct DealAssociations {
    pub companies: AssociationsResults,
    pub contacts: AssociationsResults,
}

async fn get_deal_examples(hubspot: Hubspot, deal_id: &str) {
    let deal = hubspot
        .objects
        .deals
        .read::<DealAssociations, DealProperties>(&deal_id)
        .await?;
}
```

## Contributions
At this stage we are not configured to accept contribtuions, please check back later. In the meantime please open an issue on github, and we will priorities accordingly.

### Local Development
#### Tool Set
##### Cargo

*Install the rust tool chain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.*

##### Make

*Install [cargo-make](https://github.com/sagiegurari/cargo-make) using 
`cargo install cargo-make`*

#### Build

The project has been setup with several tasks in the Makefile.
To run these tasks execute command 
`cargo make taskname`, where taskname is the name of task available
in the Makefile.toml

Running `cargo make ci` command runs the tasks to
format the files, check for lint errors, clean, build(in offline mode) and run tests.