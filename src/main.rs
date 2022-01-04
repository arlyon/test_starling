use clap::{Parser, Subcommand};
mod client;
use crate::client::StarlingAccount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    personal: String,
    business: String,
}

impl Tokens {
    pub fn as_array(&self) -> [&str; 2] {
        [self.personal.as_str(), self.business.as_str()]
    }
}

/// CLI arguments
#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

/// CLI Commands
#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Account balances
    Balances,

    /// Account Transactions
    Transactions {
        //// Days to get
        #[clap(short, long, default_value_t = 7)]
        days: i64,
    },
}

#[tokio::main]
async fn main() {
    // Get account tokens
    let f = std::fs::File::open(String::from("tokens.yml")).expect("Couldn't open tokens_file");
    let tokens: Tokens = serde_yaml::from_reader(f).expect("Could not deserialise yaml");

    // Get accounts for each token
    let mut accounts = Vec::new();
    for token in tokens.as_array().iter() {
        // TODO Check this with Alex <----------------------------
        accounts.push(StarlingAccount::new(token.to_string()).await);
    }

    let args = Args::parse();
    match args.command {
        Command::Balances => todo!(),

        Command::Transactions { days } => {
            // Fetch transactions from all Starling accounts and sort by date
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

            // Sort by date
            for transaction in transactions.iter() {
                println!("{}", transaction.to_string());
            }
            println!("Fetched {} transactions", transactions.len());
        }
    }
}
