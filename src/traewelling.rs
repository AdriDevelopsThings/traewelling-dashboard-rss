use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::Deserialize;

use crate::errors::Error;

pub const BASE_URL: &str = "https://traewelling.de";

#[derive(Clone)]
pub struct Traewelling {
    pub client_id: String,
    client_secret: String
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub expires_in: u64,
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Deserialize)]
pub struct DashboardResponse {
    pub data: Vec<DashboardData>
}

#[derive(Deserialize)]
pub struct DashboardData {
    pub id: u32,
    pub body: String,
    pub username: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub train: DashboardTrain
}

#[derive(Deserialize)]
pub struct DashboardTrain {
    #[serde(rename = "lineName")]
    pub line_name: String,
    pub origin: TrainOriginDestination,
    pub destination: TrainOriginDestination
}

#[derive(Deserialize)]
pub struct TrainOriginDestination {
    pub name: String,
    #[serde(rename = "arrivalReal")]
    pub arrival_real: Option<DateTime<Utc>>,
}


impl Traewelling {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret
        }
    }

    async fn error_handler(response: Response) -> Result<Response, Error> {
        if response.status() != 200 {
            println!("Traewelling error: {}", response.text().await?);
            return Err(Error::TraewellingConnectionError);
        }
        Ok(response.error_for_status()?)
    }

    pub async fn token(&self, code: &str, redirect_uri: &str) -> Result<TokenResponse, Error> {
        let client = reqwest::Client::new();
        let mut form = HashMap::new();
        form.insert("grant_type", "authorization_code");
        form.insert("code", code);
        form.insert("redirect_uri", redirect_uri);
        Ok(
            Self::error_handler(client.post(String::from(BASE_URL) + "/oauth/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&form)
            .send()
            .await?).await?.json().await?
        )
    }

    pub async fn dashboard(&self, token: &str) -> Result<DashboardResponse, Error> {
        let client = reqwest::Client::new();
        Ok(
            Self::error_handler(client.get(String::from(BASE_URL) + "/api/v1/dashboard")
            .bearer_auth(token)
            .send()
            .await?).await?.json().await?
        )
    }
}