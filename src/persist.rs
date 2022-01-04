//! # Handles persisting data to the file system
//!
//!

use crate::client::Transaction;
use serde::{Deserialize, Serialize};

/// Starling API auth tokens for each account.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub key: ApiKey,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiKey(pub String);

// newtype pattern
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tokens(pub Vec<Token>);

/// Render tokens as an array.
impl Tokens {
    pub fn new(filename: &str) -> Self {
        let f = std::fs::File::open(String::from(filename)).expect("Couldn't open tokens_file");
        serde_yaml::from_reader(f).expect("Could not deserialise yaml")
    }
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Write transactions to the file system.
pub fn write_transactions(transactions: &[Transaction]) {
    println!("Writing transactios to file system");
}
