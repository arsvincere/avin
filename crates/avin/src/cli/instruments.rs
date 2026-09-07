// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::Subcommand;

use avin_core::DataProvider;

use avin::err::AvinError;

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
    pub(super) fn run(self) -> Result<(), AvinError> {
        match self {
            Self::Cache { provider } => cache(provider)?,
            Self::Clear { provider } => clear(provider)?,
        }

        Ok(())
    }
}

fn cache(provider: Option<DataProvider>) -> Result<(), AvinError> {
    println!("Caching instruments info: {provider:?}");

    let _provider = match provider {
        Some(p) => p,
        None => {
            todo!()
        }
    };
    todo!()
}

fn clear(provider: Option<DataProvider>) -> Result<(), AvinError> {
    println!("Clear instruments info: {provider:?}");

    let _provider = match provider {
        Some(p) => p,
        None => {
            todo!()
        }
    };
    todo!()
}
