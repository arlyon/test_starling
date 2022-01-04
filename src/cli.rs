//! Command Line Interface functions

use clap::{Parser, Subcommand};
use crate::client::StarlingAccount;
use crate::persist;

/// CLI arguments
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

/// CLI Commands
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Account balances
    Balances,

    /// Account Transactions
    Transactions {
        //// Days to get
        #[clap(short, long, default_value_t = 7)]
        days: i64,
    },
}

pub async fn do_transactions(accounts: &Vec<StarlingAccount>, days:i64) {
    // Fetch transactions from all Starling accounts and sort by date.
    let mut transactions = Vec::new();
    for account in accounts.iter() {
        dbg!(&account.detail.name);
        for transaction in account
            .settled_transactions_between(chrono::Duration::days(days))
            .await
        {
            transactions.push(transaction);
        }
    }
    transactions.sort();

    // Display.
    for transaction in transactions.iter() {
        println!("{}", transaction.to_string());
    }
    println!("Fetched {} transactions", transactions.len());

    // Save
    persist::write_transactions(&transactions);
}