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

#[derive(Args)]
pub(super) struct SyncOptions {
    #[arg(long, exclusive = true)]
    pub resume: bool,

    #[arg(long, exclusive = true)]
    pub abort: bool,

    #[arg(long, exclusive = true)]
    pub status: bool,

    #[arg(long)]
    pub force: bool,

    #[arg(long, requires = "force")]
    pub provider: Option<DataProvider>,

    #[arg(long, requires = "provider")]
    pub instrument: Option<InstrumentId>,

    #[arg(long, requires = "instrument")]
    pub data: Option<MarketData>,

    #[arg(long, requires = "data", value_parser = parse_year)]
    pub year: Option<Year>,
}

#[derive(Args)]
pub(super) struct DeleteOptions {
    #[arg(long)]
    pub provider: Option<DataProvider>,

    #[arg(long, requires = "provider")]
    pub instrument: Option<InstrumentId>,

    #[arg(long, requires = "instrument")]
    pub data: Option<MarketData>,

    #[arg(long, requires = "data", value_parser = parse_year)]
    pub year: Option<Year>,
}

fn parse_year(value: &str) -> Result<Year, String> {
    let year = value
        .parse::<u16>()
        .map_err(|_| format!("invalid year '{value}'"))?;

    Year::new(year).map_err(|err| err.to_string())
}
