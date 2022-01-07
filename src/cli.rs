//! Command Line Interface functions

use clap::{Parser, Subcommand};
use futures::future::join_all;
use itertools::Itertools;
use std::collections::HashMap;

use crate::client::{StarlingAccount, Transaction};
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

    /// Update Transactions
    Update {
        //// Days to get
        #[clap(short, long, default_value_t = 7)]
        days: i64,
    },
}

pub async fn do_update(accounts: &[StarlingAccount], days: i64) {
    // Fetch transactions from all Starling accounts and sort by date.
    let transactions = join_all(
        accounts
            .iter()
            .map(|a| a.settled_transactions_between(chrono::Duration::days(days)))
            .collect::<Vec<_>>(),
    )
    .await;
    let transactions: Vec<_> = transactions.into_iter().flatten().sorted().collect();

    let mut transactions_dict = HashMap::new();
    for t in transactions.iter() {
        transactions_dict.insert(&t.uid, t);
    }

    // Display.
    for transaction in transactions.iter() {
        println!("{}", transaction.to_string());
    }

    persist::save_transactions(&transactions_dict);
    println!("Done")
}
