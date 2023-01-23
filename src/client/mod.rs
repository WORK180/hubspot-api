use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;

use self::error::{HubspotError, HubspotErrorResponse, HubspotResult};

pub mod error;

#[derive(Debug)]
pub struct HubspotClient {
    client: Client,
    /// Your private app access token
    token: String,
    domain: String,
    pub portal_id: String,
}

impl HubspotClient {
    /// Create HubspotClient
    pub fn new(client: Client, domain: &str, token: &str, portal_id: &str) -> Self {
        Self {
            client,
            domain: domain.to_owned(),
            token: token.to_owned(),
            portal_id: portal_id.to_owned(),
        }
    }

    pub async fn send<R>(&self, req: RequestBuilder) -> HubspotResult<R>
    where
        R: DeserializeOwned,
    {
        let res = req.bearer_auth(&self.token).send().await?;

        if res.status().is_success() {
            let body = res.bytes().await?;
            let body = body.to_vec();
            let body = std::str::from_utf8(&body).unwrap();

            if body.is_empty() {
                Ok(serde_json::from_str::<R>("null")?)
            } else {
                Ok(serde_json::from_str::<R>(body)?)
            }
        } else {
            let body = res.bytes().await?;
            let body = body.to_vec();
            let body = std::str::from_utf8(&body).unwrap();

            let err = serde_json::from_str::<HubspotErrorResponse>(body);
            if let Ok(err) = err {
                Err(HubspotError::from(err))
            } else {
                Err(HubspotError::Hubspot(body.to_owned()))
            }
        }
    }

    /// Create hubspot request builder.
    /// # Arguments
    /// * `method` = The HTTP request method.
    /// * `path` - The HTTP request path.
    pub fn begin(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, format!("https://{}/{}", self.domain, path))
    }
}
