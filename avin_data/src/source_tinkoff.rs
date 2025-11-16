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
        let mut downloader = TDownloader::new(iid, md, year);

        downloader.check_availability_market_data()?;
        downloader.create_tmp_dir();
        downloader.create_archive_path();
        downloader.create_extract_dir();

        downloader.download_archive();
        downloader.check_archive()?;
        downloader.extract_archive();

        downloader.read_extracted_files();
        downloader.format_tinkoff_bars_data();
        downloader.save();

        downloader.delete_tmp_dir();

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
        let mut recoder = TRecoder::new();
        recoder.connect().await;
        recoder.create_marketdata_stream().await;

        recoder.create_tic_storage();
        recoder.create_ob_storage();

        recoder.subscribe_tic().await;
        recoder.subscribe_ob().await;

        recoder.start();

        Ok(())
    }
}

// Download archives with 1M bars
struct TDownloader {
    iid: Iid,
    md: MarketData,
    year: i32,
    tmp_dir: PathBuf,
    archive_path: PathBuf,
    extract_dir: PathBuf,
    df: DataFrame,
}
impl TDownloader {
    // call it step by step
    fn new(iid: &Iid, md: MarketData, year: i32) -> Self {
        let schema = Self::tinkoff_csv_schema();
        let df = DataFrame::empty_with_schema(&schema);

        Self {
            iid: iid.clone(),
            md,
            year,
            tmp_dir: PathBuf::new(),
            archive_path: PathBuf::new(),
            extract_dir: PathBuf::new(),
            df,
        }
    }
    fn check_availability_market_data(&self) -> Result<(), AvinError> {
        let md = self.md;
        if !SourceTinkoff::available_market_data().contains(&md) {
            let msg = format!("{md}, Tinkoff provide only 1M bars data");
            let e = AvinError::InvalidValue(msg);
            return Err(e);
        }

        Ok(())
    }
    fn create_tmp_dir(&mut self) {
        let mut path = CFG.dir.tmp();
        path.push("tinkoff");

        // delete if exist
        if path.exists() {
            Cmd::delete_dir(&path).unwrap();
        }

        // make dir
        Cmd::make_dirs(&path).unwrap();

        self.tmp_dir = path;
    }
    fn create_archive_path(&mut self) {
        // archive name
        let e = self.iid.exchange();
        let c = self.iid.category();
        let t = self.iid.ticker();
        let y = self.year;
        let file_name = format!("{e}-{c}-{t}-1M-{y}.zip");

        // archive path
        let mut path = self.tmp_dir.clone();
        path.push("download");
        if !Cmd::is_exist(&path) {
            Cmd::make_dirs(&path).unwrap();
        }
        path.push(file_name);

        self.archive_path = path;
    }
    fn create_extract_dir(&mut self) {
        // create dir name
        let e = self.iid.exchange();
        let c = self.iid.category();
        let t = self.iid.ticker();
        let y = self.year;
        let dir_name = format!("{e}-{c}-{t}-1M-{y}");

        // create dir path for extract files
        let mut path = self.tmp_dir.clone();
        path.push("extract");
        path.push(dir_name);

        // make dir if not exist
        if !Cmd::is_exist(&path) {
            Cmd::make_dirs(&path).unwrap();
        }

        self.extract_dir = path;
    }
    fn download_archive(&self) {
        // read tinkoff token
        let token_path = CFG.connect.tinkoff();
        let token = Cmd::read(&token_path).unwrap().trim().to_string();

        // create curl bash command
        let figi = self.iid.figi();
        let year = self.year;
        let mut command = Command::new("/bin/curl");
        command.arg("-s"); // silent
        command.arg("--location");
        command.arg(format!("{SERVICE}?figi={figi}&year={year}"));
        command.arg("-H");
        command.arg(format!("Authorization: Bearer {token}"));
        command.arg("-o");
        command.arg(&self.archive_path);

        // execute
        command.spawn().unwrap().wait().unwrap();
    }
    fn check_archive(&self) -> Result<(), AvinError> {
        // if archive size == 0
        if Cmd::size(&self.archive_path).unwrap() == 0 {
            Cmd::delete_dir(&self.tmp_dir).unwrap();
            let msg = format!("{}", self.archive_path.display());
            let e = AvinError::NotExist(msg);
            return Err(e);
        }

        Ok(())
    }
    fn extract_archive(&self) {
        // create unzip bash command
        let mut command = Command::new("/bin/unzip");
        command.arg("-q"); // silent
        command.arg("-o"); // overwrite
        command.arg(&self.archive_path); // src archive
        command.arg("-d"); // output dir
        command.arg(&self.extract_dir);

        // execute
        command.spawn().unwrap().wait().unwrap();
    }
    fn read_extracted_files(&mut self) {
        // get files list (Cmd return sorting files list)
        let files = Cmd::get_files(&self.extract_dir).unwrap();

        // read all extracted files
        for file in files.iter() {
            let part = Self::read_tinkoff_csv_file(file).unwrap();
            self.df.extend(&part).unwrap();
        }
    }
    fn format_tinkoff_bars_data(&mut self) {
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
        let datetimes = self
            .df
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
        let open = self.df.column("open").unwrap().clone();
        let high = self.df.column("high").unwrap().clone();
        let low = self.df.column("low").unwrap().clone();
        let close = self.df.column("close").unwrap().clone();
        let volume = self.df.column("volume").unwrap().clone();

        // create output df
        let mut formated = DataFrame::empty();
        formated.with_column(timestamps).unwrap();
        formated.with_column(open).unwrap();
        formated.with_column(high).unwrap();
        formated.with_column(low).unwrap();
        formated.with_column(close).unwrap();
        formated.with_column(volume).unwrap();

        self.df = formated
    }
    fn save(&self) {
        Manager::save(&self.iid, Source::TINKOFF, self.md, &self.df).unwrap();
    }
    fn delete_tmp_dir(&self) {
        Cmd::delete_dir(&self.tmp_dir).unwrap();
    }

    // private
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
    fn read_tinkoff_csv_file(path: &Path) -> Result<DataFrame, AvinError> {
        let file = std::fs::File::open(path).unwrap();
        let schema = Self::tinkoff_csv_schema();

        let parse_options = CsvParseOptions::default().with_separator(b';');
        let options = CsvReadOptions::default()
            .with_parse_options(parse_options)
            .with_schema(Some(Arc::new(schema)))
            .with_has_header(false);

        let df = CsvReader::new(file).with_options(options).finish().unwrap();

        Ok(df)
    }
}

// Saving market data from stream: tic & order book
struct TRecoder {
    client: TinkoffClient,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    tic_data: HashMap<String, DataFrame>,
    ob_data: HashMap<String, DataFrame>,
}
impl TRecoder {
    const SAVE_PERIOD: TimeDelta = TimeDelta::new(60 * 60, 0).unwrap(); // 60min
    const ONE_SECOND: Duration = Duration::from_secs(1);
    const SHUTDOWN_TIME: NaiveTime = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

    // call it step by step
    fn new() -> Self {
        // create tinkoff client & start stream
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let client = TinkoffClient::new(event_tx);

        Self {
            client,
            event_rx,
            tic_data: HashMap::new(),
            ob_data: HashMap::new(),
        }
    }
    async fn connect(&mut self) {
        self.client.connect().await.unwrap();
    }
    async fn create_marketdata_stream(&mut self) {
        self.client.create_marketdata_stream().await.unwrap();
    }
    fn create_tic_storage(&mut self) {
        let tic_schema = Tic::schema();
        for record in CFG.data.record_tics.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            let df = DataFrame::empty_with_schema(&tic_schema);
            self.tic_data.insert(iid.figi().clone(), df);
        }
    }
    fn create_ob_storage(&mut self) {
        let ob_schema = OrderBook::schema();
        for record in CFG.data.record_ob.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            let df = DataFrame::empty_with_schema(&ob_schema);
            self.ob_data.insert(iid.figi().clone(), df);
        }
    }
    async fn subscribe_tic(&mut self) {
        for record in CFG.data.record_tics.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            self.client.subscribe_tic(&iid).await.unwrap();
        }
    }
    async fn subscribe_ob(&mut self) {
        for record in CFG.data.record_ob.iter() {
            let iid = Manager::find_iid(&record.iid).unwrap();
            self.client.subscribe_ob(&iid).await.unwrap();
        }
    }
    fn start(&mut self) {
        // dt variables for manage the loop
        let mut last_save = Utc::now();
        let shutdown_time = Utc::now().with_time(Self::SHUTDOWN_TIME).unwrap();

        loop {
            // receive events from broker
            while let Ok(e) = self.event_rx.try_recv() {
                println!("{e}");
                self.receive(e);
            }

            // check dt -> save
            let dt = Utc::now();
            if dt - last_save > Self::SAVE_PERIOD {
                last_save = dt;
                self.save();
            }

            // check dt -> shutdown
            if dt > shutdown_time {
                self.save();
                break;
            }

            std::thread::sleep(Self::ONE_SECOND);
        }
    }

    // private
    fn receive(&mut self, e: Event) {
        match e {
            Event::Tic(e) => {
                self.tic_data
                    .get_mut(&e.figi)
                    .unwrap()
                    .extend(&e.tic.df())
                    .unwrap();
            }
            Event::OrderBook(e) => {
                self.ob_data
                    .get_mut(&e.figi)
                    .unwrap()
                    .extend(&e.ob.df())
                    .unwrap();
            }
            _ => panic!(),
        }
    }
    fn save(&self) {
        for (figi, df) in self.tic_data.iter() {
            if df.is_empty() {
                continue;
            }

            let iid = Manager::find_figi(figi).unwrap();
            Manager::save(&iid, Source::TINKOFF, MarketData::TIC, df).unwrap();
            log::info!("Save {iid} tics");
        }

        for (figi, df) in self.ob_data.iter() {
            if df.is_empty() {
                continue;
            }

            let iid = Manager::find_figi(figi).unwrap();
            Manager::save(&iid, Source::TINKOFF, MarketData::ORDER_BOOK, df)
                .unwrap();
            log::info!("Save {iid} ob");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available() {
        let available_md = SourceTinkoff::available_market_data();
        assert_eq!(available_md.len(), 1);
    }
    #[tokio::test]
    async fn cache() {
        SourceTinkoff::cache().await.unwrap();
    }
    #[tokio::test]
    async fn download() {
        let iid = Manager::find_iid("moex_share_abio").unwrap();
        let md = MarketData::BAR_1M;
        let year = 2025;

        // download ABIO 2025 bars 1M
        SourceTinkoff::download(&iid, md, year).unwrap();

        // delete ABIO data
        let path = iid.path();
        Cmd::delete_dir(&path).unwrap();
    }
    #[tokio::test]
    async fn get_bars() {
        let iid = Manager::find_iid("moex_share_abio").unwrap();
        let md = MarketData::BAR_DAY;
        let end = Utc::now();
        let begin = end - TimeDelta::days(365);

        let df = SourceTinkoff::get_bars(&iid, md, begin, end).await.unwrap();
        assert!(!df.is_empty());
    }
}
