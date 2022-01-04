//! Starling account model 

use chrono::{DateTime, Utc};
use colored::Colorize;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.starlingbank.com/api/v2";

// Holds the individual results of the "accounts" API call
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
struct AccountDetails {
    accounts: Vec<AccountDetail>,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Currency {
    GBP,
    USD,
    EUR,
}

/// Represents available currency values
#[derive(Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CurrencyValue {
    #[serde(rename = "minorUnits")]
    pennies: u32,
    currency: Currency,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    #[serde(rename = "UPCOMING")]
    Upcoming,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "ACCOUNT_CHECK")]
    AccountCheck,
}

/// Represents a transaction returned from the API
#[derive(Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Transaction {
    #[serde(rename = "transactionTime")]
    pub time: DateTime<Utc>,

    #[serde(rename = "feedItemUid")]
    pub uid: String,

    #[serde(rename = "counterPartyName")]
    pub counterparty_name: String,

    pub direction: Direction,

    pub sourceAmount: CurrencyValue,

    pub reference: String,

    pub status: Status,
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        let entry = format!(
            "{} {} {:4}.{:0<2} {} {}",
            format!("{}", self.time.format("%Y-%m-%d")),
            match self.status {
                Status::Settled => " ",
                _ => "*",
            },
            self.sourceAmount.pennies / 100,
            self.sourceAmount.pennies % 100,
            match self.direction {
                Direction::In => "<-",
                Direction::Out => "->",
            },
            format!("{}", self.counterparty_name,).italic(),
        );

        match self.direction {
            Direction::In => format!("{}", format!("{}", entry).green()),
            Direction::Out => format!("{}", format!("{}", entry).red()),
        }
    }
}

/// Represents a single Transaction
#[derive(Deserialize, Debug)]
struct Transactions {
    #[serde(rename = "feedItems")]
    feed_items: Vec<Transaction>,
}

/// Represents a query to the API
#[derive(Serialize)]
struct QueryChangesSince {
    #[serde(rename = "changesSince")]
    changes_since: DateTime<Utc>,
}

/// Represents a query to the API
#[derive(Serialize)]
struct QueryChangesBetween {
    #[serde(rename = "minTransactionTimestamp")]
    min_transaction_timestamp: DateTime<Utc>,
    #[serde(rename = "maxTransactionTimestamp")]
    max_transaction_timestamp: DateTime<Utc>,
}

/// Represents a Starling account
#[derive(Deserialize, Debug)]
pub struct StarlingAccount {
    pub key: String,
    pub detail: AccountDetail,
}

impl StarlingAccount {
    pub async fn new(key: String) -> Self {
        let detail = Self::get_account_details(&key).await.unwrap();

        Self { key, detail }
    }

    pub async fn transactions_since(&self, since: chrono::Duration) -> Vec<Transaction> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "{}/feed/account/{}/category/{}",
                BASE_URL, &self.detail.account_uid, &self.detail.default_category
            ))
            .header(AUTHORIZATION, format!("Bearer {}", &self.key))
            .header(ACCEPT, "application/json")
            .query(&QueryChangesSince {
                changes_since: Utc::now() - since,
            })
            .send()
            .await
            .unwrap();

        response.json::<Transactions>().await.unwrap().feed_items
    }

    pub async fn settled_transactions_between(&self, since: chrono::Duration) -> Vec<Transaction> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "{}/feed/account/{}/settled-transactions-between",
                BASE_URL, &self.detail.account_uid
            ))
            .header(AUTHORIZATION, format!("Bearer {}", &self.key))
            .header(ACCEPT, "application/json")
            .query(&QueryChangesBetween {
                min_transaction_timestamp: Utc::now() - since,
                max_transaction_timestamp: Utc::now(),
            })
            .send()
            .await
            .unwrap();

        response.json::<Transactions>().await.unwrap().feed_items
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
