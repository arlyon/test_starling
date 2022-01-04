//! # Handles persisting data to the file system
//!
//! 

use crate::client::Transaction;
use serde::{Deserialize, Serialize};

/// Starling API auth tokens for each account
#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    personal: String,
    business: String,
}

/// Render tokens as an array.
impl Tokens {
    pub fn as_array(&self) -> [&str; 2] {
        [self.personal.as_str(), self.business.as_str()]
    }
}

/// Write transactions to the file system.
pub fn write_transactions(transactions: &Vec<Transaction>) {
    println!("Writing transactios to file system");
}
