use reqwest::Client;

use crate::{
    error::{Error, Result},
    types::{FirefoxEmailRelayRequest, FirefoxEmailRelayResponse, FirefoxRelayProfile},
};

pub struct FFRelayApi {
    client: Client,
    token: String,
}

const FFRELAY_ADDRESSED_API: &str = "https://relay.firefox.com/api/v1/relayaddresses";

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
        let token = format!("Token {}", &self.token);
        let url = format!("{FFRELAY_ADDRESSED_API}/");

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

        dbg!(&resp_dict);

        let res: FirefoxEmailRelayResponse = serde_json::from_value(resp_dict)?;

        Ok(res.full_address)
    }

    pub async fn list(&self) -> Result<Vec<FirefoxEmailRelayResponse>> {
        let token = format!("Token {}", &self.token);

        let relay_array = self
            .client
            .get(FFRELAY_ADDRESSED_API)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        //dbg!(&relay_array);

        let email_relays: Vec<FirefoxEmailRelayResponse> = serde_json::from_value(relay_array)?;

        Ok(email_relays)
    }

    pub async fn delete(&self, email_id: u32) -> Result<()> {
        let url = format!("{FFRELAY_ADDRESSED_API}/{email_id}");

        let token = format!("Token {}", &self.token);

        let ret = self
            .client
            .delete(url)
            .header("content-type", "application/json")
            .header("authorization", token)
            .send()
            .await?;

        if ret.status().is_success() {
            println!("success");
            Ok(())
        } else {
            Err(Error::RequestFailure {
                http_status: ret.status().as_u16(),
            })
        }
    }
}
