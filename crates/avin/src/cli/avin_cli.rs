// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::{Parser, Subcommand};

use super::data::DataCommand;
use super::instruments::InstrumentsCommand;

#[derive(Parser)]
pub struct AvinCli {
    #[command(subcommand)]
    group: Group,
}

impl AvinCli {
    pub fn run() {
        let cli = Self::parse();

        match cli.group {
            Group::Instruments { command } => command.run(),
            Group::Data { command } => command.run(),
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
