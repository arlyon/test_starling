//! Implements the functionality to model a Starling account

use chrono::{DateTime, Utc};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::Deserialize;

const BASE_URL: &str = "https://api.starlingbank.com/api/v2";

// Holds the individual results of the "accounts" API call
#[derive(Deserialize, Debug)]
struct AccountDetail {
    pub name: String,

    #[serde(rename = "accountUid")]
    pub account_uid: String,

    #[serde(rename = "defaultCategory")]
    pub default_category: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

/// Holds the results of the "accounts" API call
#[derive(Deserialize, Debug)]
struct AccountDetails {
    accounts: Vec<AccountDetail>,
}

/// Represents a Starling account
#[derive(Deserialize, Debug)]
pub struct StarlingAccount {
    key: String,
    detail: AccountDetail,
}

impl StarlingAccount {
    pub async fn new(key: String) -> Self {
        let detail = Self::get_account_details(&key).await.unwrap();

        Self {
            key,
            detail,
        }
    }

    /// Get details for Starling account with api_key
    async fn get_account_details(api_key: &String) -> Option<AccountDetail> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/accounts", BASE_URL))
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                let account_details = response
                    .json::<AccountDetails>()
                    .await
                    .expect("ERROR: Couldn't serialise AccountDetails");
                account_details.accounts.into_iter().next()
            }

            reqwest::StatusCode::FORBIDDEN => {
                panic!("ERROR: Need to grab a new token");
            }

            _ => {
                panic!("ERROR: Could not get account details");
            }
        }
    }
}
