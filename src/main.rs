use serde::{Serialize, Deserialize};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use chrono::{DateTime, Utc};

const BASE_URL: &str = "https://api.starlingbank.com/api/v2";

#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    personal: String,
    business: String,
}

/// Holds the individual results of the "accounts" API call
#[derive(Deserialize, Debug)]
pub struct AccountDetail {
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
pub struct AccountDetails {
    accounts: Vec<AccountDetail>,
}

/// Represents a Starling account
#[derive(Deserialize, Debug)]
struct StarlingAccount {
    key: String,
    base_url: String,
    detail: AccountDetail,
}

impl StarlingAccount {

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
                let account_details = response.json::<AccountDetails>().await.expect("ERROR: Couldn't serialise AccountDetails");
                account_details.accounts.into_iter().next()
            },
            
            reqwest::StatusCode::FORBIDDEN => {
                panic!("ERROR: Need to grab a new token");
            },
            
            _ => {
                panic!("ERROR: Could not get account details");
            }
        }
    }

    async fn new(key: String) -> Self {

        let detail = Self::get_account_details(&key).await.unwrap();

        Self {
            key,
            base_url: BASE_URL.to_string(),
            detail,
        }
    }
}

#[tokio::main]
async fn main() {

    // Get tokens
    let f = std::fs::File::open(String::from("tokens.yml")).expect("Couldn't open tokens_file");
    let tokens: Tokens = serde_yaml::from_reader(f).expect("Could not deserialise yaml");

    // Get accounts
    let account = StarlingAccount::new(tokens.personal).await;

    println!("{:#?}", account);
}
