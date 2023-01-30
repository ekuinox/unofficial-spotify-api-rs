use anyhow::Result;
use serde::Deserialize;

const CLIENT_USER_AGENT: &str = "reqwest";

fn base_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(CLIENT_USER_AGENT)
        .build()
        .expect("client build error")
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenResponse {
    pub client_id: String,
    pub access_token: String,
    pub access_token_expiration_timestamp_ms: u64,
    pub is_anonymous: bool,
}

#[derive(Debug)]
pub struct Client {
    #[allow(unused)]
    client_id: String,
    #[allow(unused)]
    access_token: String,
    #[allow(unused)]
    access_token_expiration_timestamp_ms: u64,
    #[allow(unused)]
    is_anonymous: bool,
}

impl Client {
    pub fn new(
        client_id: String,
        access_token: String,
        access_token_expiration_timestamp_ms: u64,
        is_anonymous: bool,
    ) -> Client {
        Client {
            client_id,
            access_token,
            access_token_expiration_timestamp_ms,
            is_anonymous,
        }
    }
}

impl Client {
    pub async fn init() -> Result<Client> {
        let AccessTokenResponse {
            client_id,
            access_token,
            access_token_expiration_timestamp_ms,
            is_anonymous,
        }: AccessTokenResponse = base_client()
            .get("https://open.spotify.com/get_access_token")
            .send()
            .await?
            .json()
            .await?;
        Ok(Client::new(
            client_id,
            access_token,
            access_token_expiration_timestamp_ms,
            is_anonymous,
        ))
    }
}

pub type Subrole = String;
pub type RoleTitle = String;

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub uri: String,
    pub name: String,
    pub image_uri: String,
    pub subroles: Vec<Subrole>,
    pub weight: f64,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoleCredit {
    pub role_title: RoleTitle,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackCreditsViewResponse {
    pub track_uri: String,
    pub track_title: String,
    pub role_credits: Vec<RoleCredit>,
    pub extended_credits: Vec<serde_json::Value>,
    pub source_names: Vec<String>,
}

impl Client {
    pub async fn track_credits_view(&self, track_id: &str) -> Result<TrackCreditsViewResponse> {
        let view = base_client()
            .get(format!("https://spclient.wg.spotify.com/track-credits-view/v0/experimental/{track_id}/credits"))
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(view)
    }
}
