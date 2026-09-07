// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::Subcommand;

use avin::err::AvinError;

#[derive(Subcommand)]
pub(super) enum WorkspaceCommand {
    Init,
    New { name: String },
}

impl WorkspaceCommand {
    pub(super) fn run(self) -> Result<(), AvinError> {
        match self {
            Self::Init => todo!(),
            Self::New { .. } => todo!(),
        }
    }
}
