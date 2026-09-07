// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::Subcommand;

use avin_core::DataProvider;

#[derive(Subcommand)]
pub(super) enum InstrumentsCommand {
    Cache {
        #[arg(long)]
        provider: Option<DataProvider>,
    },

    Clear {
        #[arg(long)]
        provider: Option<DataProvider>,
    },
}

impl InstrumentsCommand {
    pub(super) fn run(self) {
        match self {
            Self::Cache { provider } => {
                println!("Caching instruments info: {provider:?}");
            }

            Self::Clear { provider } => {
                println!("Clear instruments info: {provider:?}");
            }
        }
    }
}
