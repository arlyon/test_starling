//! Command Line Interface functions

use clap::{Parser, Subcommand};

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
