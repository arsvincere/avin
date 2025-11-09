/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::str::FromStr;

use chrono::{Datelike, Utc};
use clap::{Args, Parser, Subcommand};
use strum::IntoEnumIterator;

use avin_core::{MarketData, Source};
use avin_utils::AvinError;

use crate::Data;

#[derive(Parser)]
#[command(name = "avin-data")]
#[command(version)]
#[command(about = "Utility for operations with market data")]
#[command(long_about = "Utility for operations with market data

# ru
Утилита для операций с рыночными данными от разных источников:
скачивание, преобразование, обновление.
")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
impl Cli {
    pub async fn run() {
        let cli = Cli::parse();

        let result = match cli.command {
            Commands::Cache(args) => cache(args).await,
            Commands::Find(args) => find(args),
            Commands::Download(args) => download(args).await,
            Commands::Convert(args) => convert(args),
            Commands::Update => update(),
            Commands::Write => write_real_time(),
        };

        match result {
            Ok(()) => (),
            Err(e) => log::error!("{e}"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Cache instruments info
    ///
    /// # Examples
    /// avin-data cache TINKOFF
    /// avin-data cache MOEXALGO
    ///
    /// # ru
    /// Кеширование информации об инструментах
    Cache(CacheArgs),
    /// Find instrument
    ///
    /// # Examples
    /// avin-data find MOEX_SHARE_SBER
    ///
    /// # ru
    /// Найти инструмент
    Find(FindArgs),
    /// Download market data
    ///
    /// # Examples
    /// avin-data download MOEX_SHARE_SBER TINKOFF
    ///
    /// # ru
    /// Скачивание рыночных данных
    Download(DownloadArgs),
    /// Convert market data
    ///
    /// # ru
    /// Преобразование таймфреймов
    Convert(ConvertArgs),
    /// Update all available market data
    ///
    /// # ru
    /// Обновить все имеющиеся рыночные данные
    Update,
    /// Write real time tics and order book data
    ///
    /// # ru
    /// Реал-тайм запись тиков и данных по стакану
    Write,
}

#[derive(Args, Debug)]
struct CacheArgs {
    /// Available values: [TINKOFF, MOEXALGO]
    ///
    /// # ru
    /// Допустимые значения: [TINKOFF, MOEXALGO]
    ///
    /// Если источник не указан будут кешированы все доступные источники.
    source: Option<String>,
}
#[derive(Args, Debug)]
struct FindArgs {
    /// Instrument ID
    ///
    /// # ru
    /// Идентификатор инструмента
    instrument: String,
    // /// Source of market data: [TINKOFF, MOEXALGO]
    // ///
    // /// # ru
    // /// Источник рыночных данных.
    // source: String,
}
#[derive(Args, Debug)]
struct DownloadArgs {
    /// Instrument ID
    ///
    /// # ru
    /// Идентификатор инструмента
    instrument: String,
    /// Source of market data: [TINKOFF, MOEXALGO]
    ///
    /// # ru
    /// Источник рыночных данных.
    source: String,
    /// Market data kind
    ///
    /// # ru
    /// Тип данных.
    data: String,
}
#[derive(Args, Debug)]
struct ConvertArgs {
    /// Instrument ID
    ///
    /// # ru
    /// Идентификатор инструмента
    instrument: String,
    /// Source of market data: [TINKOFF, MOEXALGO]
    ///
    /// # ru
    /// Источник рыночных данных.
    source: String,
    /// Input market data kind: [BAR_1M, BAR_10M, ...]
    ///
    /// # ru
    /// Входной тип данных.
    input: String,
    /// Output market data kind: [BAR_5M, BAR_10M, ...]
    ///
    /// # ru
    /// Выходной данных.
    output: String,
}

async fn cache(args: CacheArgs) -> Result<(), AvinError> {
    if let Some(value) = args.source {
        let source = Source::from_str(value.as_str())?;
        Data::cache(source).await?;
    } else {
        for source in Source::iter() {
            Data::cache(source).await?;
        }
    };

    Ok(())
}

fn find(args: FindArgs) -> Result<(), AvinError> {
    let iid = Data::find(&args.instrument)?;

    let exchange = iid.exchange();
    let category = iid.category();
    let ticker = iid.ticker();
    let figi = iid.figi();
    let name = iid.name();
    let lot = iid.lot();
    let step = iid.step();

    let table = format!(
        "┌───────────┬─────────────────┐
│ Name      ┆ {name:>15} │
│ Exchange  ┆ {exchange:>15} │
│ Category  ┆ {category:>15} │
│ Ticker    ┆ {ticker:>15} │
│ FIGI      ┆ {figi:>15} │
│ Lot       ┆ {lot:>15} │
│ Step      ┆ {step:>15} │
└───────────┴─────────────────┘"
    );
    println!("{table}");

    Ok(())
}

async fn download(args: DownloadArgs) -> Result<(), AvinError> {
    let iid = Data::find(&args.instrument)?;
    let source = Source::from_str(&args.source)?;
    let md = MarketData::from_str(&args.data)?;

    let mut year = Utc::now().year();

    loop {
        let result = Data::download(&iid, source, md, year).await;

        if result.is_ok() {
            year -= 1;
        } else {
            return result;
        }
    }
}

fn convert(args: ConvertArgs) -> Result<(), AvinError> {
    let iid = Data::find(&args.instrument)?;
    let source = Source::from_str(&args.source)?;
    let input = MarketData::from_str(&args.input)?;
    let output = MarketData::from_str(&args.output)?;

    Data::convert(&iid, source, input, output)
}

fn update() -> Result<(), AvinError> {
    log::info!("Updating...");
    Ok(())
}

fn write_real_time() -> Result<(), AvinError> {
    log::info!("Writing...");
    Ok(())
}
