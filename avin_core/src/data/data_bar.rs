/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, TimeZone, Utc};
use polars::prelude::*;

use avin_utils::{self as utils, AvinError, Cmd};

use crate::{Bar, Iid, MarketData, Source};

#[derive(Debug)]
pub struct DataBar {}
impl DataBar {
    pub fn save(
        iid: &Iid,
        source: Source,
        md: MarketData,
        df: &DataFrame,
    ) -> Result<(), AvinError> {
        // NOTE: в датафрейме могут быть данные за
        // два разных года. Например при обновление в первых
        // числах января. Перед сохранением нужно проверить состав
        // датафрейма и сохранить кусками по годам.
        let first = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .first()
            .unwrap();
        let last = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .last()
            .unwrap();

        // define current year and end year
        let mut year = utils::dt(first).year();
        let end_year = utils::dt(last).year();

        // filter and save by years
        while year <= end_year {
            // begin/end timestamps
            let b = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            let e = Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap();
            let begin_ts = utils::ts(b);
            let end_ts = utils::ts(e);

            // filter rows of single year
            let mut year_df = df
                .clone()
                .lazy()
                .filter(col("ts_nanos").gt_eq(begin_ts))
                .filter(col("ts_nanos").lt(end_ts))
                .collect()
                .unwrap();

            // save
            let path = create_file_path(iid, source, md, year);
            Cmd::write_pqt(&mut year_df, &path).unwrap();

            year += 1;
        }

        Ok(())
    }
    pub fn load(
        iid: &Iid,
        source: Source,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        // create empty df
        let schema = Bar::schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // load data by years
        let mut year = begin.year();
        let end_year = end.year();
        while year <= end_year {
            let path = create_file_path(iid, source, md, year);
            match load_file(&path) {
                Ok(data) => {
                    df.extend(&data).unwrap();
                    year += 1;
                }
                Err(AvinError::NotFound(_)) => {
                    year += 1;
                }
                Err(other) => {
                    log::error!("{other}");
                    panic!();
                }
            }
        }

        // filter & check empty
        let df = utils::filter_dt(begin, end, df);
        if df.is_empty() {
            let msg = format!("market data {md} for {iid}/{source}");
            let e = AvinError::NotFound(msg);
            return Err(e);
        }

        Ok(df)
    }
    /// Load last market data file.
    ///
    /// # ru
    /// Загружает один самый новый файл с рыночными данными.
    pub fn load_last(
        iid: &Iid,
        source: Source,
        md: MarketData,
    ) -> Result<DataFrame, AvinError> {
        // check dir with data
        let dir_path = create_dir_path(iid, source, md);
        if !dir_path.exists() {
            let msg = format!("{iid}/{source} at {}", dir_path.display());
            let e = AvinError::NotFound(msg);
            return Err(e);
        }

        // check files in dir
        let files = Cmd::get_files(&dir_path).unwrap(); // it's sorted
        if files.is_empty() {
            let msg = format!("{iid}/{source} at {}", dir_path.display());
            let e = AvinError::NotFound(msg);
            return Err(e);
        }

        // read last file
        let last_file = files.last().unwrap();
        let df = Cmd::read_pqt(last_file).unwrap();

        Ok(df)
    }
}

fn create_dir_path(iid: &Iid, source: Source, md: MarketData) -> PathBuf {
    let mut path = iid.path();
    path.push(source.name());
    path.push(md.name());

    path
}
fn create_file_path(
    iid: &Iid,
    source: Source,
    md: MarketData,
    year: i32,
) -> PathBuf {
    let mut path = create_dir_path(iid, source, md);
    path.push(format!("{year}.parquet"));

    path
}
fn load_file(path: &Path) -> Result<DataFrame, AvinError> {
    // check path is exist, else AvinError::NotFound
    if !Cmd::is_exist(path) {
        let msg = format!("{}", path.display());
        let e = AvinError::NotFound(msg);
        return Err(e);
    }

    // read file, else AvinError::IOError
    match Cmd::read_pqt(path) {
        Ok(df) => Ok(df),
        Err(why) => {
            let msg = format!("read {} - {}", path.display(), why);
            let e = AvinError::IOError(msg);
            Err(e)
        }
    }
}
