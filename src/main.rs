mod client;
use crate::client::StarlingAccount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    personal: String,
    business: String,
}

#[tokio::main]
async fn main() {
    // Get tokens
    let f = std::fs::File::open(String::from("tokens.yml")).expect("Couldn't open tokens_file");
    let tokens: Tokens = serde_yaml::from_reader(f).expect("Could not deserialise yaml");

    // Get accounts
    let account = StarlingAccount::new(tokens.personal).await;
    
    for transaction in account.transactions_since(chrono::Duration::days(7)).await {
        println!("{}", transaction.to_string());
    }
}
