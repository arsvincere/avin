// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_service::DataService;
use clap::{Args, Subcommand};

use avin_core::Year;
use avin_domain::{DataProvider, InstrumentId, MarketData};

use avin::err::AvinError;

#[derive(Subcommand)]
pub(super) enum DataCommand {
    Download(DownloadOptions),
    Delete(DeleteOptions),
    Sync(SyncOptions),
    Prune,
    Compact,
}

impl DataCommand {
    pub(super) fn run(self) -> Result<(), AvinError> {
        match self {
            DataCommand::Download(options) => download(options),

            DataCommand::Sync(options) => {
                if options.resume {
                    println!("Resume sync");
                    return Ok(());
                }

                if options.abort {
                    println!("Abort sync");
                    return Ok(());
                }

                if options.status {
                    println!("Sync status");
                    return Ok(());
                }

                println!(
                    "Sync: f={}, p={:?}, i={:?}, d={:?}, y={:?}",
                    options.force,
                    options.provider,
                    options.instrument,
                    options.data,
                    options.year,
                );
                Ok(())
            }

            DataCommand::Delete(options) => {
                println!(
                    "Delete: p={:?}, i={:?}, d={:?}, y={:?}",
                    options.provider,
                    options.instrument,
                    options.data,
                    options.year,
                );
                Ok(())
            }

            DataCommand::Prune => {
                println!("Prune data");
                Ok(())
            }

            DataCommand::Compact => {
                println!("Compact data");
                Ok(())
            }
        }
    }
}

#[derive(Args)]
pub(super) struct DownloadOptions {
    #[arg(long)]
    provider: DataProvider,

    #[arg(long, requires = "provider")]
    instrument: InstrumentId,

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

fn download(opt: DownloadOptions) -> Result<(), AvinError> {
    DataService::download(
        opt.provider,
        &opt.instrument,
        opt.data.unwrap(),
        opt.year.unwrap(),
    )
    .unwrap();

    Ok(())
}

// TODO: добавить парсер в сам тип Year чтобы clap им мог пользоваться
fn parse_year(value: &str) -> Result<Year, String> {
    let year = value
        .parse::<u16>()
        .map_err(|_| format!("invalid year '{value}'"))?;

    Year::new(year).map_err(|err| err.to_string())
}
