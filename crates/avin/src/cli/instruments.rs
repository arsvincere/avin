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

    List {
        #[arg(long)]
        provider: Option<DataProvider>,
    },
}

impl InstrumentsCommand {
    pub(super) async fn run(self) -> Result<(), AvinError> {
        match self {
            Self::Cache { provider } => cache(provider).await,
            Self::Clear { provider } => clear(provider),
            Self::List { provider } => list(provider),
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
            AvinError::cli("caching failed", Some(err.into()))
        })?;
    }

    log::info!("Instrument reference data cached successfully");

    Ok(())
}

fn clear(provider: Option<DataProvider>) -> Result<(), AvinError> {
    // If provider is None, clear all.
    let providers = match provider {
        Some(p) => vec![p],
        None => DataProvider::all().to_vec(),
    };

    // clearing
    for provider in providers {
        log::info!("Clearing instrument cache for {provider}");

        InstrumentService::clear(provider).map_err(|err| {
            AvinError::cli("clearing failed", Some(err.into()))
        })?;
    }

    log::info!("Instrument reference data cleared successfully");

    Ok(())
}

fn list(provider: Option<DataProvider>) -> Result<(), AvinError> {
    // get inventory instrument cache
    let mut inventory = InstrumentService::inventory().map_err(|err| {
        AvinError::cli("failed to get instrument inventory", Some(err.into()))
    })?;

    // filter by provider
    if let Some(provider) = provider {
        inventory.retain(|(p, _, _)| *p == provider);
    }

    // header
    let mut report = String::new();
    report.push_str(&format!("{}\n", "-".repeat(59)));
    report.push_str(&format!(
        "{:<16} {:<12} {:<16} {:>12}\n",
        "Provider", "Exchange", "Category", "Instruments"
    ));
    report.push_str(&format!("{}\n", "-".repeat(59)));

    // report rows
    let mut total = 0;
    for (p, e, c) in inventory {
        let list = InstrumentService::list(p, e, c).map_err(|err| {
            let msg = format!("failed to get instruments for {p} {e} {c}");
            AvinError::cli(msg, Some(err.into()))
        })?;

        let count = list.len();
        total += count;

        report.push_str(&format!(
            "{:<16} {:<12} {:<16} {:>12}\n",
            p.to_string(),
            e.to_string(),
            c.to_string(),
            count
        ));
    }

    // summary
    report.push_str(&format!("{}\n", "-".repeat(59)));
    report.push_str(&format!(
        "{:<16} {:<12} {:<16} {total:>12}\n",
        "Total", "", ""
    ));
    report.push_str(&format!("{}\n", "-".repeat(59)));

    // print
    print!("{report}");

    Ok(())
}
