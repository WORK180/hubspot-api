use serde::{Deserialize, Serialize};

/// Companies store information about the organizations that interact with your business.
#[derive(Serialize, Deserialize, Debug)]
pub struct Company<P> {
    /// Company details are stored in company properties.
    /// There are default HubSpot company properties,
    /// but you can also create custom company properties.
    pub properties: P,
}
