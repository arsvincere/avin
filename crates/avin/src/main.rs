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
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Instruments(InstrumentsArgs),
    Data(DataArgs),
}

#[derive(Args)]
struct DataArgs {
    #[command(subcommand)]
    command: DataCommand,
}

#[derive(Subcommand)]
enum DataCommand {
    Sync(SyncArgs),
    Delete(DeleteArgs),
    Prune,
    Compact,
}

// Instruments ---------------------------------------------------------------

#[derive(Args)]
struct InstrumentsArgs {
    #[command(subcommand)]
    command: InstrumentsCommand,
}

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

// Sync ----------------------------------------------------------------------

#[derive(Args)]
struct SyncArgs {
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
struct DeleteArgs {
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
        Command::Instruments(instruments) => match instruments.command {
            InstrumentsCommand::Cache { provider } => {
                println!("Caching instruments info: {provider:?}");
            }
            InstrumentsCommand::Clear { provider } => {
                println!("Clear instruments info: {provider:?}");
            }
        },

        Command::Data(data) => match data.command {
            DataCommand::Sync(args) => {
                if args.resume {
                    println!("Resume sync");
                    return;
                }

                if args.abort {
                    println!("Abort sync");
                    return;
                }

                if args.status {
                    println!("Sync status");
                    return;
                }

                println!(
                    "Sync: f={}, p={:?}, i={:?}, d={:?}, y={:?}",
                    args.force,
                    args.provider,
                    args.instrument,
                    args.data,
                    args.year,
                );
            }

            DataCommand::Delete(args) => {
                println!(
                    "Delete: p={:?}, i={:?}, d={:?}, y={:?}",
                    args.provider, args.instrument, args.data, args.year,
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
