// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use clap::{Args, Parser, Subcommand};

use avin_core::{DataProvider, MarketData, Year};
use avin_domain::InstrumentId;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    group: Group,
}

impl Cli {
    pub fn run() {
        let cli = Self::parse();

        match cli.group {
            Group::Instruments { command } => match command {
                InstrumentsCommand::Cache { provider } => {
                    println!("Caching instruments info: {provider:?}");
                }

                InstrumentsCommand::Clear { provider } => {
                    println!("Clear instruments info: {provider:?}");
                }
            },

            Group::Data { command } => match command {
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
            },
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

// Instruments ---------------------------------------------------------------

#[derive(Subcommand)]
enum InstrumentsCommand {
    Cache {
        #[arg(long)]
        provider: Option<DataProvider>,
    },

    Clear {
        #[arg(long)]
        provider: Option<DataProvider>,
    },
}

// Data ----------------------------------------------------------------------

#[derive(Subcommand)]
enum DataCommand {
    Sync(SyncOptions),
    Delete(DeleteOptions),
    Prune,
    Compact,
}

// Sync ----------------------------------------------------------------------

#[derive(Args)]
struct SyncOptions {
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

// Delete --------------------------------------------------------------------

#[derive(Args)]
struct DeleteOptions {
    #[arg(long)]
    provider: Option<DataProvider>,

    #[arg(long, requires = "provider")]
    instrument: Option<InstrumentId>,

    #[arg(long, requires = "instrument")]
    data: Option<MarketData>,

    #[arg(long, requires = "data", value_parser = parse_year)]
    year: Option<Year>,
}

// Parsers -------------------------------------------------------------------

fn parse_year(value: &str) -> Result<Year, String> {
    let year = value
        .parse::<u16>()
        .map_err(|_| format!("invalid year '{value}'"))?;

    Year::new(year).map_err(|err| err.to_string())
}
