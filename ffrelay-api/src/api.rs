use log::info;
use reqwest::Client;

use crate::{
    error::{Error, Result},
    types::{FirefoxEmailRelay, FirefoxEmailRelayRequest, FirefoxRelayProfile},
};

pub struct FFRelayApi {
    client: Client,
    token: String,
}

const FFRELAY_API_ENDPOINT: &str = "https://relay.firefox.com/api";
//const FFRELAY_API_ENDPOINT: &str = "http://localhost:1234";

const FFRELAY_EMAIL_ENDPOINT: &str = "v1/relayaddresses";
const FFRELAY_EMAIL_DOMAIN_ENDPOINT: &str = "v1/domainaddresses";

impl FFRelayApi {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let client = Client::new();

        Self {
            client,
            token: token.into(),
        }
    }

    async fn create_with_endpoint(
        &self,
        endpoint: &str,
        request: FirefoxEmailRelayRequest,
    ) -> Result<String> {
        let token = format!("Token {}", &self.token);
        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}/");

        info!("url: {url}");

        let resp_dict = self
            .client
            .post(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .json(&request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&resp_dict);

        let res: FirefoxEmailRelay = serde_json::from_value(resp_dict)?;

        Ok(res.full_address)
    }

    async fn list_with_endpoint(&self, endpoint: &str) -> Result<Vec<FirefoxEmailRelay>> {
        let token = format!("Token {}", &self.token);

        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}");

        let relay_array = self
            .client
            .get(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&relay_array);

        let email_relays: Vec<FirefoxEmailRelay> = serde_json::from_value(relay_array)?;

        Ok(email_relays)
    }

    async fn delete_with_endpoint(&self, endpoint: &str, email_id: u64) -> Result<()> {
        let url = format!("{FFRELAY_API_ENDPOINT}/{endpoint}/{email_id}");

        let token = format!("Token {}", &self.token);

        let ret = self
            .client
            .delete(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?;

        if ret.status().is_success() {
            Ok(())
        } else {
            Err(Error::EmailDeletionFailure {
                http_status: ret.status().as_u16(),
            })
        }
    }

    async fn find_email_relay(&self, email_id: u64) -> Result<FirefoxEmailRelay> {
        let relays = self.list().await?;

        for r in relays {
            if r.id == email_id {
                return Ok(r);
            }
        }

        Err(Error::RelayIdNotFound)
    }

    ////////////////////////////////////////////////////////////////////////////
    // PUBLIC
    ////////////////////////////////////////////////////////////////////////////

    pub async fn profiles(&self) -> Result<Vec<FirefoxRelayProfile>> {
        let url = "https://relay.firefox.com/api/v1/profiles/";
        let token = format!("Token {}", &self.token);

        let profiles_dict = self
            .client
            .get(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&profiles_dict);

        let profiles: Vec<FirefoxRelayProfile> = serde_json::from_value(profiles_dict)?;

        Ok(profiles)
    }

    pub async fn create(&self, request: FirefoxEmailRelayRequest) -> Result<String> {
        let endpoint = if request.address.is_some() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.create_with_endpoint(endpoint, request).await
    }

    pub async fn list(&self) -> Result<Vec<FirefoxEmailRelay>> {
        let mut relays = vec![];

        if let Ok(email_relays) = self.list_with_endpoint(FFRELAY_EMAIL_ENDPOINT).await {
            relays.extend(email_relays);
        }

        if let Ok(domain_relays) = self.list_with_endpoint(FFRELAY_EMAIL_DOMAIN_ENDPOINT).await {
            relays.extend(domain_relays);
        }

        Ok(relays)
    }

    pub async fn delete(&self, email_id: u64) -> Result<()> {
        let relay = self.find_email_relay(email_id).await?;

        let endpoint = if relay.is_domain() {
            FFRELAY_EMAIL_DOMAIN_ENDPOINT
        } else {
            FFRELAY_EMAIL_ENDPOINT
        };

        self.delete_with_endpoint(endpoint, email_id).await
    }
}
