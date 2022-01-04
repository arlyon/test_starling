mod cli;
mod client;
mod persist;
use crate::client::StarlingAccount;
use clap::{Parser};

#[tokio::main]
async fn main() {
    // Get account tokens
    let f = std::fs::File::open(String::from("tokens.yml")).expect("Couldn't open tokens_file");
    let tokens: persist::Tokens = serde_yaml::from_reader(f).expect("Could not deserialise yaml");

    // Get accounts for each token.
    let mut accounts = Vec::new();
    for token in tokens.as_array().iter() {
        // TODO Check this with Alex <----------------------------
        accounts.push(StarlingAccount::new(token.to_string()).await);
    }

    let args = cli::Args::parse();
    match args.command {
        cli::Command::Balances => todo!(),

        cli::Command::Transactions { days } => {
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
    }
}
