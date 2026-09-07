// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod data;
mod instruments;

// ───────────────────────────────────────────────────────────────────────────

use clap::{Parser, Subcommand};

use self::data::DataCommand;
use self::instruments::InstrumentsCommand;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    group: Group,
}

impl Cli {
    pub fn run() {
        let cli = Self::parse();

        match cli.group {
            Group::Instruments { command } => match command {
                InstrumentsCommand::Cache { provider } => {
                    println!("Caching instruments info: {provider:?}");
                }

                InstrumentsCommand::Clear { provider } => {
                    println!("Clear instruments info: {provider:?}");
                }
            },

            Group::Data { command } => match command {
                DataCommand::Sync(options) => {
                    if options.resume {
                        println!("Resume sync");
                        return;
                    }

                    if options.abort {
                        println!("Abort sync");
                        return;
                    }

                    if options.status {
                        println!("Sync status");
                        return;
                    }

                    println!(
                        "Sync: f={}, p={:?}, i={:?}, d={:?}, y={:?}",
                        options.force,
                        options.provider,
                        options.instrument,
                        options.data,
                        options.year,
                    );
                }

                DataCommand::Delete(options) => {
                    println!(
                        "Delete: p={:?}, i={:?}, d={:?}, y={:?}",
                        options.provider,
                        options.instrument,
                        options.data,
                        options.year,
                    );
                }

                DataCommand::Prune => {
                    println!("Prune data");
                }

                DataCommand::Compact => {
                    println!("Compact data");
                }
            },
        }
    }
}

#[derive(Subcommand)]
enum Group {
    Instruments {
        #[command(subcommand)]
        command: InstrumentsCommand,
    },

    Data {
        #[command(subcommand)]
        command: DataCommand,
    },
}
