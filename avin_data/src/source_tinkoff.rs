/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::Duration,
};

use chrono::{DateTime, NaiveTime, TimeDelta, Utc};
use polars::{
    io::SerReader,
    prelude::{
        Column, CsvParseOptions, CsvReadOptions, CsvReader, DataFrame,
        DataType, Field, Schema,
    },
};

use avin_connect::TinkoffClient;
use avin_core::{
    Bar, Category, Event, Iid, Manager, MarketData, OrderBook, Share, Source,
    Tic,
};
use avin_utils::{AvinError, CFG, Cmd};

const SERVICE: &str = "https://invest-public-api.tinkoff.ru/history-data";
const SAVE_PERIOD: TimeDelta = TimeDelta::new(10 * 60, 0).unwrap(); // 10 min
const ONE_SECOND: Duration = Duration::from_secs(1);
const SHUTDOWN_TIME: NaiveTime = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

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

        // create paths
        let tmp_dir = create_tmp_dir();
        let archive_path = create_archive_path(&tmp_dir, iid, year);
        let extract_dir = create_extract_dir(&tmp_dir, iid, year);

        // download
        download_archive(iid, year, &archive_path);

        // check archive
        if Cmd::size(&archive_path).unwrap() == 0 {
            Cmd::delete_dir(&tmp_dir).unwrap();
            let msg = format!("{md} for {year}");
            let e = AvinError::NotExist(msg);
            return Err(e);
        }

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
        Manager::save(iid, Source::TINKOFF, md, &df).unwrap();

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
    pub async fn record() -> Result<(), AvinError> {
        // create tinkoff client & start stream
        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
        let mut tinkoff_client = create_tinkoff_client(event_tx).await;

        let mut tic_data = create_tic_storage();
        let mut ob_data = create_ob_storage();
        subscribe_tic(&mut tinkoff_client).await;
        subscribe_ob(&mut tinkoff_client).await;

        // dt variables for manage the loop
        let mut last_save = Utc::now();
        let shutdown_time = Utc::now().with_time(SHUTDOWN_TIME).unwrap();

        loop {
            // receive events from broker
            while let Ok(e) = event_rx.try_recv() {
                println!("{e}");
                match e {
                    Event::Tic(e) => {
                        tic_data
                            .get_mut(&e.figi)
                            .unwrap()
                            .extend(&e.tic.df())
                            .unwrap();
                    }
                    Event::OrderBook(e) => {
                        ob_data
                            .get_mut(&e.figi)
                            .unwrap()
                            .extend(&e.ob.df())
                            .unwrap();
                    }
                    _ => panic!(),
                }
            }

            // check dt -> save
            let dt = Utc::now();
            if dt - last_save > SAVE_PERIOD {
                last_save = dt;
                save_tics(&tic_data);
                save_ob(&ob_data);
            }

            // check dt -> shutdown
            if dt > shutdown_time {
                save_tics(&tic_data);
                save_ob(&ob_data);
                break;
            }

            std::thread::sleep(ONE_SECOND);
        }

        log::info!("Record finished at {shutdown_time}");

        Ok(())
    }
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
fn tinkoff_csv_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("uuid".into(), DataType::String),
        Field::new("datetime".into(), DataType::String),
        Field::new("open".into(), DataType::Float64),
        Field::new("close".into(), DataType::Float64),
        Field::new("high".into(), DataType::Float64),
        Field::new("low".into(), DataType::Float64),
        Field::new("volume".into(), DataType::UInt64),
        Field::new("__empty__".into(), DataType::String),
    ])
}
fn read_tinkoff_csv(path: &Path) -> Result<DataFrame, AvinError> {
    let file = std::fs::File::open(path).unwrap();
    let schema = tinkoff_csv_schema();

    let parse_options = CsvParseOptions::default().with_separator(b';');
    let options = CsvReadOptions::default()
        .with_parse_options(parse_options)
        .with_schema(Some(Arc::new(schema)))
        .with_has_header(false);

    let df = CsvReader::new(file).with_options(options).finish().unwrap();

    Ok(df)
}
fn read_extracted_files(extract_dir: &Path) -> DataFrame {
    // create empty df for read all tinkoff data
    let schema = tinkoff_csv_schema();
    let mut df = DataFrame::empty_with_schema(&schema);

    // get files list (Cmd return sorting files list)
    let files = Cmd::get_files(extract_dir).unwrap();

    // read all extracted files
    for file in files.iter() {
        let part = read_tinkoff_csv(file).unwrap();
        df.extend(&part).unwrap();
    }

    df
}
fn format_tinkoff_bars_data(tinkoff_df: DataFrame) -> DataFrame {
    // Tinkoff csv market data
    // ┌─────────────────────┬─────────────────────┬─────────┬─────────┬─────────┬─────────┬────────┬───────────┐
    // │ uuid                ┆ datetime            ┆ open    ┆ close   ┆ high    ┆ low     ┆ volume ┆ __empty__ │
    // │ ---                 ┆ ---                 ┆ ---     ┆ ---     ┆ ---     ┆ ---     ┆ ---    ┆ ---       │
    // │ str                 ┆ str                 ┆ f64     ┆ f64     ┆ f64     ┆ f64     ┆ u64    ┆ str       │
    // ╞═════════════════════╪═════════════════════╪═════════╪═════════╪═════════╪═════════╪════════╪═══════════╡
    // │ 509edd0c-129c-4ee2- ┆ 2021-01-04T07:00:00 ┆ 23820.0 ┆ 23900.0 ┆ 23904.0 ┆ 23812.0 ┆ 3132   ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-01-04T07:01:00 ┆ 23898.0 ┆ 23910.0 ┆ 23946.0 ┆ 23894.0 ┆ 2208   ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-01-04T07:02:00 ┆ 23910.0 ┆ 23908.0 ┆ 23914.0 ┆ 23860.0 ┆ 1084   ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-01-04T07:03:00 ┆ 23908.0 ┆ 23892.0 ┆ 23946.0 ┆ 23890.0 ┆ 1558   ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-01-04T07:04:00 ┆ 23890.0 ┆ 23904.0 ┆ 23920.0 ┆ 23890.0 ┆ 399    ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ …                   ┆ …                   ┆ …       ┆ …       ┆ …       ┆ …       ┆ …      ┆ …         │
    // │ 509edd0c-129c-4ee2- ┆ 2021-12-30T20:45:00 ┆ 22912.0 ┆ 22916.0 ┆ 22916.0 ┆ 22908.0 ┆ 140    ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-12-30T20:46:00 ┆ 22914.0 ┆ 22910.0 ┆ 22918.0 ┆ 22910.0 ┆ 124    ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-12-30T20:47:00 ┆ 22910.0 ┆ 22916.0 ┆ 22916.0 ┆ 22908.0 ┆ 119    ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-12-30T20:48:00 ┆ 22910.0 ┆ 22916.0 ┆ 22916.0 ┆ 22906.0 ┆ 31     ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // │ 509edd0c-129c-4ee2- ┆ 2021-12-30T20:49:00 ┆ 22916.0 ┆ 22900.0 ┆ 22916.0 ┆ 22900.0 ┆ 121    ┆ null      │
    // │ 934d-7f6246…        ┆ Z                   ┆         ┆         ┆         ┆         ┆        ┆           │
    // └─────────────────────┴─────────────────────┴─────────┴─────────┴─────────┴─────────┴────────┴───────────┘
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
        .column("datetime")
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

    // select columns OHLCV
    let open = tinkoff_df.column("open").unwrap().clone();
    let high = tinkoff_df.column("high").unwrap().clone();
    let low = tinkoff_df.column("low").unwrap().clone();
    let close = tinkoff_df.column("close").unwrap().clone();
    let volume = tinkoff_df.column("volume").unwrap().clone();

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

async fn create_tinkoff_client(
    event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
) -> TinkoffClient {
    let mut tinkoff_client = TinkoffClient::new(event_tx);
    tinkoff_client.connect().await.unwrap();
    tinkoff_client.create_marketdata_stream().await.unwrap();

    tinkoff_client
}
fn create_tic_storage() -> HashMap<String, DataFrame> {
    let tic_schema = Tic::schema();
    let mut tic_data = HashMap::new();
    for record in CFG.data.record_tics.iter() {
        let iid = Manager::find_iid(&record.iid).unwrap();
        let df = DataFrame::empty_with_schema(&tic_schema);
        tic_data.insert(iid.figi().clone(), df);
    }

    tic_data
}
fn create_ob_storage() -> HashMap<String, DataFrame> {
    let ob_schema = OrderBook::schema();
    let mut ob_data = HashMap::new();
    for record in CFG.data.record_ob.iter() {
        let iid = Manager::find_iid(&record.iid).unwrap();
        let df = DataFrame::empty_with_schema(&ob_schema);
        ob_data.insert(iid.figi().clone(), df);
    }

    ob_data
}
async fn subscribe_tic(tinkoff_client: &mut TinkoffClient) {
    for record in CFG.data.record_tics.iter() {
        let iid = Manager::find_iid(&record.iid).unwrap();
        tinkoff_client.subscribe_tic(&iid).await.unwrap();
    }
}
async fn subscribe_ob(tinkoff_client: &mut TinkoffClient) {
    for record in CFG.data.record_ob.iter() {
        let iid = Manager::find_iid(&record.iid).unwrap();
        tinkoff_client.subscribe_ob(&iid).await.unwrap();
    }
}
fn save_tics(tic_data: &HashMap<String, DataFrame>) {
    for (figi, df) in tic_data.iter() {
        let iid = Manager::find_figi(figi).unwrap();
        Manager::save(&iid, Source::TINKOFF, MarketData::TIC, df).unwrap();
        log::info!("Save {iid} tics");
    }
}
fn save_ob(ob_data: &HashMap<String, DataFrame>) {
    for (figi, df) in ob_data.iter() {
        let iid = Manager::find_figi(figi).unwrap();
        Manager::save(&iid, Source::TINKOFF, MarketData::ORDER_BOOK, df)
            .unwrap();
        log::info!("Save {iid} ob");
    }
}

#[allow(unused)]
struct TicStorage {
    client: TinkoffClient,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    tic_data: HashMap<String, DataFrame>,
}
#[allow(unused)]
impl TicStorage {
    fn new() -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let client = TinkoffClient::new(event_tx);

        Self {
            client,
            event_rx,
            tic_data: HashMap::new(),
        }
    }
    async fn init(&mut self) {
        self.client.connect().await.unwrap();
        self.client.create_marketdata_stream().await.unwrap();

        self.create_tic_storage();
        self.subscribe_tic();
    }

    fn create_tic_storage(&mut self) {
        let tic_schema = Tic::schema();
        for record in CFG.data.record_tics.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            let df = DataFrame::empty_with_schema(&tic_schema);
            self.tic_data.insert(iid.figi().clone(), df);
        }
    }
    async fn subscribe_tic(&mut self) {
        for record in CFG.data.record_tics.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            self.client.subscribe_tic(&iid).await.unwrap();
        }
    }
    fn save_tics(&self) {
        for (figi, df) in self.tic_data.iter() {
            let iid = Manager::find_figi(figi).unwrap();
            Manager::save(&iid, Source::TINKOFF, MarketData::TIC, df).unwrap();
            log::info!("Save {iid} tics");
        }
    }
}
