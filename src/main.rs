#![deny(unsafe_code)]
#![warn(clippy::unwrap_used)]

mod cli;
mod client;
mod persist;
use crate::client::StarlingAccount;
use clap::Parser;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    // Get accounts for each token.
    let tokens = persist::Tokens::new("tokens.yml");

    let accounts = join_all(
        tokens
            .into_iter()
            .map(|t| StarlingAccount::new(t.key))
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let args = cli::Args::parse();
    match args.command {
        cli::Command::Balances => todo!(),
        cli::Command::Transactions { days } => cli::do_transactions(&accounts, days).await,
    }
}
