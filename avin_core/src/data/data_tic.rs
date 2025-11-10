/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Days, NaiveDate, TimeZone, Utc};
use polars::prelude::*;

use avin_utils::{self as utils, AvinError, Cmd};

use crate::{Iid, MarketData, Source, Tic};

#[derive(Debug)]
pub struct DataTic {}
impl DataTic {
    pub fn save(
        iid: &Iid,
        source: Source,
        md: MarketData,
        df: &DataFrame,
    ) -> Result<(), AvinError> {
        // NOTE: в датафрейме могут быть данные за
        // два разных дня... Ну на всякий случай
        // перед сохранением нужно проверить состав
        // датафрейма и сохранить кусками по дням.
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

        // define current day and end day
        let mut day = utils::dt(first).date_naive();
        let end_day = utils::dt(last).date_naive();

        // filter and save by days
        while day <= end_day {
            // begin/end timestamps
            let b = Utc
                .with_ymd_and_hms(day.year(), day.month(), day.day(), 0, 0, 0)
                .unwrap();
            let e = b.checked_add_days(Days::new(1)).unwrap();
            let begin_ts = utils::ts(b);
            let end_ts = utils::ts(e);

            // filter rows of single day
            let mut day_df = df
                .clone()
                .lazy()
                .filter(col("ts_nanos").gt_eq(begin_ts))
                .filter(col("ts_nanos").lt(end_ts))
                .collect()
                .unwrap();

            // save
            let path = create_file_path(iid, source, md, day);
            Cmd::write_pqt(&mut day_df, &path).unwrap();

            day = day.checked_add_days(Days::new(1)).unwrap();
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
        let schema = Tic::schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // load data by days
        let mut day = begin.date_naive();
        let end_day = end.date_naive();
        while day <= end_day {
            let path = create_file_path(iid, source, md, day);
            match load_file(&path) {
                Ok(file_df) => {
                    df.extend(&file_df).unwrap();
                    day = day.checked_add_days(Days::new(1)).unwrap();
                }
                Err(AvinError::NotFound(_)) => {
                    day = day.checked_add_days(Days::new(1)).unwrap();
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
            let msg = format!("{iid} {md}");
            return Err(AvinError::NotFound(msg));
        }

        Ok(df)
    }
}

fn create_dir_path(
    iid: &Iid,
    source: Source,
    md: MarketData,
    day: NaiveDate,
) -> PathBuf {
    let mut path = iid.path();
    path.push(source.name());
    path.push(md.name());
    path.push(day.year().to_string());

    path
}
fn create_file_path(
    iid: &Iid,
    source: Source,
    md: MarketData,
    day: NaiveDate,
) -> PathBuf {
    let mut path = create_dir_path(iid, source, md, day);
    path.push(format!("{}.parquet", day.format("%Y-%m-%d")));

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
