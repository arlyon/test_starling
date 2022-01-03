use clap::{Parser, Subcommand};
mod client;
use crate::client::StarlingAccount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tokens {
    personal: String,
    business: String,
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
    let args = Args::parse();

    // Get tokens
    let f = std::fs::File::open(String::from("tokens.yml")).expect("Couldn't open tokens_file");
    let tokens: Tokens = serde_yaml::from_reader(f).expect("Could not deserialise yaml");

    // Get accounts
    let account = StarlingAccount::new(tokens.personal).await;

    match args.command {
        Command::Balances => todo!(),

        Command::Transactions { days } => {
            for transaction in account
                .transactions_since(chrono::Duration::days(days))
                .await
            {
                println!("{}", transaction.to_string());
            }
        }
    }
}
