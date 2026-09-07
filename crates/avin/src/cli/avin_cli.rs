// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::{Parser, Subcommand};

use avin_system::Workspace;

use avin::err::AvinError;

use super::data::DataCommand;
use super::instruments::InstrumentsCommand;
use super::workspace::WorkspaceCommand;

#[derive(Parser)]
pub struct AvinCli {
    #[command(subcommand)]
    group: Group,
}

impl AvinCli {
    pub fn run() -> Result<(), AvinError> {
        let cli = Self::parse();

        if cli.group.requires_workspace() {
            Workspace::get().map_err(|err| AvinError::Cli {
                message: "failed to open AVIN workspace".to_string(),
                source: Some(Box::new(err)),
            })?;
        }

        match cli.group {
            Group::Workspace(command) => command.run(),
            Group::Instruments { command } => command.run(),
            Group::Data { command } => command.run(),
        }
    }
}

#[derive(Subcommand)]
enum Group {
    #[command(flatten)]
    Workspace(WorkspaceCommand),
    Instruments {
        #[command(subcommand)]
        command: InstrumentsCommand,
    },
    Data {
        #[command(subcommand)]
        command: DataCommand,
    },
}

impl Group {
    fn requires_workspace(&self) -> bool {
        !matches!(self, Self::Workspace(_))
    }
}
