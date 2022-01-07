//! # Handles persisting data to the file system
//!
//!

use std::thread::current;

use crate::client::{Transaction, Transactions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TRANSACTION_FILE: &str = "transactions.yml";

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
pub fn save_transactions(transactions: &HashMap<&String, &Transaction>) {
    println!("Writing transactions to file system");

    // Load current transactions if they exist
    let f = std::fs::File::open(String::from(TRANSACTION_FILE)).expect("Fail");
    let current_transactions: Vec<Transaction> = serde_yaml::from_reader(f).unwrap();

    // new_transactions
    //     .iter()
    //     .collect();

    // Merge new transactions

    // Save updated transactions
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TRANSACTION_FILE)
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, transactions).unwrap();
}

fn does_exist_in(t: &Transaction, transactions: &Vec<Transaction>) -> bool {
    true
}
