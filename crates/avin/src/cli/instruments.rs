// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::Subcommand;

use avin_domain::DataProvider;
use avin_service::InstrumentService;
use avin_system::Workspace;

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
    pub(super) async fn run(self) -> Result<(), AvinError> {
        match self {
            Self::Cache { provider } => cache(provider).await,
            Self::Clear { provider } => clear(provider),
        }
    }
}

async fn cache(provider: Option<DataProvider>) -> Result<(), AvinError> {
    // If provider is None, use all providers from the workspace data manifest.
    let providers = match provider {
        Some(p) => vec![p],
        None => {
            let ws = Workspace::get().expect(
                "workspace must be initialized before \
                running instrument commands",
            );
            ws.data.providers()
        }
    };

    // warn if nothing to cache
    if providers.is_empty() {
        log::warn!(
            "No data provider specified and no providers configured \
            in the workspace data manifest. Nothing to cache."
        );
        return Ok(());
    }

    // caching
    for provider in providers {
        log::info!("Caching instruments from {provider}");

        InstrumentService::cache(provider).await.map_err(|err| {
            let msg = "instrument caching failed".to_string();
            AvinError::Cli {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;
    }

    log::info!("Instrument reference data cached successfully");

    Ok(())
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
