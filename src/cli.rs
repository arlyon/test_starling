//! Command Line Interface functions

use clap::{Parser, Subcommand};
use futures::future::join_all;
use itertools::Itertools;

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

pub async fn do_transactions(accounts: &[StarlingAccount], days: i64) {
    // Fetch transactions from all Starling accounts and sort by date.
    let transactions = join_all(
        accounts
            .iter()
            .map(|a| a.settled_transactions_between(chrono::Duration::days(days)))
            .collect::<Vec<_>>(),
    )
    .await;
    let transactions: Vec<_> = transactions.into_iter().flatten().sorted().collect();

    // Display.
    for transaction in transactions.iter() {
        println!("{}", transaction.to_string());
    }

    // Save
    persist::write_transactions(&transactions);
}
