use serde::Deserialize;

/// In HubSpot, deals represent transactions with contacts or companies.
/// Deals are tracked through your sales process in pipeline stages until they're won or lost.
#[derive(Deserialize, Debug)]
pub struct Deal<A, P> {
    pub id: String,
    /// When creating a new deal, you can also associate the deal with existing records or activities.
    pub associations: A,
    /// Deal details are stored in deal properties.
    /// There are default HubSpot deal properties,
    /// but you can also create custom properties.
    pub properties: P,
}

#[derive(Deserialize, Debug)]
pub struct AssociationsResults {
    pub results: Vec<Association>,
}

#[derive(Deserialize, Debug)]
pub struct Association {
    pub id: String,
    #[serde(alias = "type")]
    pub association_type: String,
}
