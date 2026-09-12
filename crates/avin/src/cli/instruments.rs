// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_service::InstrumentService;
use avin_system::Workspace;
use clap::Subcommand;

use avin_domain::DataProvider;

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
    let providers = match provider {
        Some(p) => vec![p],
        None => {
            let ws = Workspace::get().expect("TODO msg");
            ws.data.providers()
        }
    };

    for provider in providers {
        log::info!("Caching instruments info: {provider}");
        InstrumentService::cache(provider).await.map_err(|err| {
            let msg = "TODO msg".to_string();
            // TODO: тип ошибки? или тут вообще уже похую и нужен anyhow?
            AvinError::Cli {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;
    }

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
