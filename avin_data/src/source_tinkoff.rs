/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Datelike, Utc};
use polars::prelude::{Column, DataFrame, DataType, Field, Schema};

use avin_connect::TinkoffClient;
use avin_core::{Bar, Category, Iid, Manager, MarketData, Share, Source};
use avin_utils::{AvinError, CFG, Cmd};

const SERVICE: &str = "https://invest-public-api.tinkoff.ru/history-data";

pub struct SourceTinkoff {}
impl SourceTinkoff {
    pub fn available_market_data() -> Vec<MarketData> {
        vec![MarketData::BAR_1M]
    }
    pub async fn cache() -> Result<(), AvinError> {
        // NOTE: пока кеширует только акции

        // create tinkoff client
        let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();
        let mut tinkoff_client = TinkoffClient::new(event_tx);
        tinkoff_client.connect().await.unwrap();

        // creatre empty df for shares info
        let schema = Share::schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // request shares
        let shares = tinkoff_client.get_shares().await.unwrap();

        // put shares info to dataframe
        for share in shares {
            let row = share.info_df();
            df.extend(&row).unwrap();
        }

        Manager::save_cache(Source::TINKOFF, Category::SHARE, df)
    }
    pub fn download(
        iid: &Iid,
        md: MarketData,
        year: i32,
    ) -> Result<(), AvinError> {
        // check availability market data type
        if !Self::available_market_data().contains(&md) {
            let msg = format!("{md}, Tinkoff provide only 1M bars data");
            let e = AvinError::InvalidValue(msg);
            return Err(e);
        }

        // check year
        if !is_available_year(year) {
            let msg = format!("{md} for {year}");
            let e = AvinError::NotExist(msg);
            return Err(e);
        }

        // create paths
        let tmp_dir = create_tmp_dir();
        let archive_path = create_archive_path(&tmp_dir, iid, year);
        let extract_dir = create_extract_dir(&tmp_dir, iid, year);

        // download
        download_archive(iid, year, &archive_path);

        // extract archive
        extract_archive(&archive_path, &extract_dir);

        // read tinkoff files
        let tinkoff_df = read_extracted_files(&extract_dir);
        if tinkoff_df.is_empty() {
            let msg = format!("{md} for {year}");
            let e = AvinError::NotExist(msg);
            return Err(e);
        }

        // format & save bars data
        let df = format_tinkoff_bars_data(tinkoff_df);
        Manager::save(iid, Source::TINKOFF, md, df).unwrap();

        // delete tmp dir
        Cmd::delete_dir(&tmp_dir).unwrap();

        Ok(())
    }
    pub async fn get_bars(
        iid: &Iid,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        // create tinkoff client
        let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();
        let mut tinkoff_client = TinkoffClient::new(event_tx);
        tinkoff_client.connect().await.unwrap();

        let tf = md.timeframe().unwrap();
        let bars = tinkoff_client.get_bars(iid, tf, begin, end).await.unwrap();
        let df = Bar::to_df(&bars).unwrap();

        Ok(df)
    }
    // pub async fn write_real_time() -> Result<(), AvinError> {
    //     todo!()
    // }
}

fn is_available_year(year: i32) -> bool {
    let max_year = Utc::now().year();
    let min_year = 2018;

    min_year <= year && year <= max_year
}
fn create_tmp_dir() -> PathBuf {
    let mut dir_path = CFG.dir.tmp();
    dir_path.push("tinkoff");

    // clear if exist
    if dir_path.exists() {
        Cmd::delete_dir(&dir_path).unwrap();
    }

    // make dir
    Cmd::make_dirs(&dir_path).unwrap();

    dir_path
}
fn create_archive_path(tmp_dir: &Path, iid: &Iid, year: i32) -> PathBuf {
    // create archive file name
    let e = iid.exchange();
    let c = iid.category();
    let t = iid.ticker();
    let file_name = format!("{e}-{c}-{t}-1M-{year}.zip");

    // create archive dir path
    let mut archive_path = PathBuf::new();
    archive_path.push(tmp_dir);
    archive_path.push("download");

    // make dir if not exist
    if !Cmd::is_exist(&archive_path) {
        Cmd::make_dirs(&archive_path).unwrap();
    }

    // join dir path + file name
    archive_path.push(file_name);

    archive_path
}
fn create_extract_dir(tmp_dir: &Path, iid: &Iid, year: i32) -> PathBuf {
    // create dir name
    let e = iid.exchange();
    let c = iid.category();
    let t = iid.ticker();
    let dir_name = format!("{e}-{c}-{t}-1M-{year}");

    // create dir path for extract files
    let mut extract_dir_path = PathBuf::new();
    extract_dir_path.push(tmp_dir);
    extract_dir_path.push("extract");
    extract_dir_path.push(dir_name);

    // make dir if not exist
    if !Cmd::is_exist(&extract_dir_path) {
        Cmd::make_dirs(&extract_dir_path).unwrap();
    }

    extract_dir_path
}
fn download_archive(iid: &Iid, year: i32, archive_path: &Path) {
    // read tinkoff token
    let token_path = CFG.connect.tinkoff();
    let token = Cmd::read(&token_path).unwrap().trim().to_string();

    // create curl bash command
    let figi = iid.figi();
    let mut command = Command::new("/bin/curl");
    command.arg("-s"); // silent
    command.arg("--location");
    command.arg(format!("{SERVICE}?figi={figi}&year={year}"));
    command.arg("-H");
    command.arg(format!("Authorization: Bearer {token}"));
    command.arg("-o");
    command.arg(archive_path);

    // execute
    command.spawn().unwrap().wait().unwrap();
}
fn extract_archive(archive_path: &Path, extract_dir: &Path) {
    // create unzip bash command
    let mut command = Command::new("/bin/unzip");
    command.arg("-q"); // silent
    command.arg("-o"); // overwrite
    command.arg(archive_path); // src archive
    command.arg("-d"); // output dir
    command.arg(extract_dir);

    // execute
    command.spawn().unwrap().wait().unwrap();
}
fn read_extracted_files(extract_dir: &Path) -> DataFrame {
    // create empty df for read all tinkoff data
    let schema = Schema::from_iter(vec![
        Field::new("column_1".into(), DataType::String),
        Field::new("column_2".into(), DataType::String),
        Field::new("column_3".into(), DataType::Float64), // open
        Field::new("column_4".into(), DataType::Float64), // close
        Field::new("column_5".into(), DataType::Float64), // high
        Field::new("column_6".into(), DataType::Float64), // low
        Field::new("column_7".into(), DataType::Int64),   // volume
        Field::new("column_8".into(), DataType::String),
    ]);
    let mut df = DataFrame::empty_with_schema(&schema);

    // get files list (Cmd return sorting files list)
    let files = Cmd::get_files(extract_dir).unwrap();

    // read all extracted files
    for file in files.iter() {
        let part = Cmd::read_csv(file).unwrap();
        df.extend(&part).unwrap();
    }

    df
}
fn format_tinkoff_bars_data(tinkoff_df: DataFrame) -> DataFrame {
    // Tinkoff csv market data
    //   uuid                               datetime               open       close      high       low       volume
    // ┌─────────────────────────────────┬──────────────────────┬──────────┬──────────┬──────────┬──────────┬──────────┬──────────┐
    // │ column_1                        ┆ column_2             ┆ column_3 ┆ column_4 ┆ column_5 ┆ column_6 ┆ column_7 ┆ column_8 │
    // │ ---                             ┆ ---                  ┆ ---      ┆ ---      ┆ ---      ┆ ---      ┆ ---      ┆ ---      │
    // │ str                             ┆ str                  ┆ f64      ┆ f64      ┆ f64      ┆ f64      ┆ i64      ┆ str      │
    // ╞═════════════════════════════════╪══════════════════════╪══════════╪══════════╪══════════╪══════════╪══════════╪══════════╡
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-01-02T07:16:00Z ┆ 268.87   ┆ 271.04   ┆ 271.04   ┆ 268.87   ┆ 17       ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-01-02T07:17:00Z ┆ 268.87   ┆ 271.0    ┆ 271.07   ┆ 268.87   ┆ 194      ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-01-02T07:18:00Z ┆ 269.0    ┆ 270.83   ┆ 271.0    ┆ 269.0    ┆ 94       ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-01-02T07:19:00Z ┆ 269.91   ┆ 269.91   ┆ 270.83   ┆ 269.91   ┆ 66       ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-01-02T07:20:00Z ┆ 269.91   ┆ 270.72   ┆ 270.76   ┆ 269.91   ┆ 102      ┆ null     │
    // │ …                               ┆ …                    ┆ …        ┆ …        ┆ …        ┆ …        ┆ …        ┆ …        │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-12-31T20:44:00Z ┆ 280.91   ┆ 280.91   ┆ 280.91   ┆ 280.91   ┆ 1        ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-12-31T20:45:00Z ┆ 280.92   ┆ 281.26   ┆ 281.26   ┆ 280.91   ┆ 6        ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-12-31T20:46:00Z ┆ 280.91   ┆ 280.9    ┆ 281.26   ┆ 280.9    ┆ 17       ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-12-31T20:48:00Z ┆ 280.91   ┆ 281.26   ┆ 281.26   ┆ 280.9    ┆ 37       ┆ null     │
    // │ e6123145-9665-43e0-8413-cd61b8… ┆ 2024-12-31T20:49:00Z ┆ 280.9    ┆ 280.9    ┆ 280.9    ┆ 280.9    ┆ 41       ┆ null     │
    // └─────────────────────────────────┴──────────────────────┴──────────┴──────────┴──────────┴──────────┴──────────┴──────────┘
    //
    // output df
    // ┌─────────────────────┬────────┬────────┬────────┬────────┬────────┐
    // │ ts_nanos            ┆ open   ┆ high   ┆ low    ┆ close  ┆ volume │
    // │ ---                 ┆ ---    ┆ ---    ┆ ---    ┆ ---    ┆ ---    │
    // │ i64                 ┆ f64    ┆ f64    ┆ f64    ┆ f64    ┆ i64    │
    // ╞═════════════════════╪════════╪════════╪════════╪════════╪════════╡
    // │ 1704179760000000000 ┆ 268.87 ┆ 271.04 ┆ 268.87 ┆ 271.04 ┆ 17     │
    // │ 1704179820000000000 ┆ 268.87 ┆ 271.07 ┆ 268.87 ┆ 271.0  ┆ 194    │
    // │ 1704179880000000000 ┆ 269.0  ┆ 271.0  ┆ 269.0  ┆ 270.83 ┆ 94     │
    // │ 1704179940000000000 ┆ 269.91 ┆ 270.83 ┆ 269.91 ┆ 269.91 ┆ 66     │
    // │ 1704180000000000000 ┆ 269.91 ┆ 270.76 ┆ 269.91 ┆ 270.72 ┆ 102    │
    // │ …                   ┆ …      ┆ …      ┆ …      ┆ …      ┆ …      │
    // │ 1735677840000000000 ┆ 280.91 ┆ 280.91 ┆ 280.91 ┆ 280.91 ┆ 1      │
    // │ 1735677900000000000 ┆ 280.92 ┆ 281.26 ┆ 280.91 ┆ 281.26 ┆ 6      │
    // │ 1735677960000000000 ┆ 280.91 ┆ 281.26 ┆ 280.9  ┆ 280.9  ┆ 17     │
    // │ 1735678080000000000 ┆ 280.91 ┆ 281.26 ┆ 280.9  ┆ 281.26 ┆ 37     │
    // │ 1735678140000000000 ┆ 280.9  ┆ 280.9  ┆ 280.9  ┆ 280.9  ┆ 41     │
    // └─────────────────────┴────────┴────────┴────────┴────────┴────────┘

    // convert str datetime -> timestamp nanos
    let mut timestamps = Vec::new();
    let datetimes = tinkoff_df
        .column("column_2")
        .unwrap()
        .str()
        .unwrap()
        .into_iter();
    for str_opt in datetimes {
        let dt = str_opt.unwrap();
        let dt = DateTime::parse_from_rfc3339(dt).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        timestamps.push(ts);
    }
    let timestamps = Column::new("ts_nanos".into(), timestamps);

    // rename columns OHLCV
    let mut open = tinkoff_df.column("column_3").unwrap().clone();
    let mut high = tinkoff_df.column("column_5").unwrap().clone();
    let mut low = tinkoff_df.column("column_6").unwrap().clone();
    let mut close = tinkoff_df.column("column_4").unwrap().clone();
    let mut volume = tinkoff_df
        .column("column_7")
        .unwrap()
        .clone()
        .cast(&DataType::UInt64)
        .unwrap();

    open.rename("open".into());
    high.rename("high".into());
    low.rename("low".into());
    close.rename("close".into());
    volume.rename("volume".into());

    // create output df
    let mut df = DataFrame::empty();
    df.with_column(timestamps).unwrap();
    df.with_column(open).unwrap();
    df.with_column(high).unwrap();
    df.with_column(low).unwrap();
    df.with_column(close).unwrap();
    df.with_column(volume).unwrap();

    df
}
