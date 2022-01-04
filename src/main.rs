mod cli;
mod client;
mod persist;
use crate::client::StarlingAccount;
use clap::Parser;

#[tokio::main]
async fn main() {
    // Get accounts for each token.
    let tokens = persist::Tokens::new("tokens.yml");
    let mut accounts = Vec::new();
    for token in tokens.as_array().iter() {
        // TODO Check this with Alex <----------------------------
        accounts.push(StarlingAccount::new(token.to_string()).await);
    }

    let args = cli::Args::parse();
    match args.command {
        cli::Command::Balances => todo!(),
        cli::Command::Transactions { days } => cli::do_transactions(&accounts, days).await,
    }
}
