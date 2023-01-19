use serde::Deserialize;

/// In HubSpot, contacts store information about the individual people that interact with your business.
#[derive(Deserialize, Debug)]
pub struct Contact<P> {
    /// Contact details are stored in contact properties. There are default HubSpot contact properties, but you can also create custom contact properties.
    pub properties: P,
}
