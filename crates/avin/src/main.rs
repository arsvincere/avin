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
struct Cli {
    #[command(subcommand)]
    command: RootCommand,
}

#[derive(Subcommand)]
enum RootCommand {
    Instruments {
        #[command(subcommand)]
        action: InstrumentsAction,
    },

    Data {
        #[command(subcommand)]
        action: DataAction,
    },
}

// Instruments ---------------------------------------------------------------

#[derive(Subcommand)]
enum InstrumentsAction {
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
enum DataAction {
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

// Main ----------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();

    match cli.command {
        RootCommand::Instruments { action } => match action {
            InstrumentsAction::Cache { provider } => {
                println!("Caching instruments info: {provider:?}");
            }

            InstrumentsAction::Clear { provider } => {
                println!("Clear instruments info: {provider:?}");
            }
        },

        RootCommand::Data { action } => match action {
            DataAction::Sync(options) => {
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

            DataAction::Delete(options) => {
                println!(
                    "Delete: p={:?}, i={:?}, d={:?}, y={:?}",
                    options.provider,
                    options.instrument,
                    options.data,
                    options.year,
                );
            }

            DataAction::Prune => {
                println!("Prune data");
            }

            DataAction::Compact => {
                println!("Compact data");
            }
        },
    }
}
