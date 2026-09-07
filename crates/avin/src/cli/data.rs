// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::{Args, Subcommand};

use avin_core::{DataProvider, MarketData, Year};
use avin_domain::InstrumentId;

#[derive(Subcommand)]
pub(super) enum DataCommand {
    Sync(SyncOptions),
    Delete(DeleteOptions),
    Prune,
    Compact,
}

impl DataCommand {
    pub(super) fn run(self) {
        match self {
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
        };
    }
}

#[derive(Args)]
pub(super) struct SyncOptions {
    #[arg(long, exclusive = true)]
    resume: bool,

    #[arg(long, exclusive = true)]
    abort: bool,

    #[arg(long, exclusive = true)]
    status: bool,

    #[arg(long)]
    force: bool,

    #[arg(long, requires = "force")]
    provider: Option<DataProvider>,

    #[arg(long, requires = "provider")]
    instrument: Option<InstrumentId>,

    #[arg(long, requires = "instrument")]
    data: Option<MarketData>,

    #[arg(long, requires = "data", value_parser = parse_year)]
    year: Option<Year>,
}

#[derive(Args)]
pub(super) struct DeleteOptions {
    #[arg(long)]
    provider: Option<DataProvider>,

    #[arg(long, requires = "provider")]
    instrument: Option<InstrumentId>,

    #[arg(long, requires = "instrument")]
    data: Option<MarketData>,

    #[arg(long, requires = "data", value_parser = parse_year)]
    year: Option<Year>,
}

fn parse_year(value: &str) -> Result<Year, String> {
    let year = value
        .parse::<u16>()
        .map_err(|_| format!("invalid year '{value}'"))?;

    Year::new(year).map_err(|err| err.to_string())
}
