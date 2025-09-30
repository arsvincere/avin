/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;

use chrono::{DateTime, Datelike, TimeZone, Utc};
use polars::prelude::*;

use avin_utils::{self as utils, AvinError, Cmd};

use crate::{Iid, MarketData, schema};

#[derive(Debug)]
pub struct DataOB {}
impl DataOB {
    pub fn save(
        iid: &Iid,
        md: MarketData,
        df: DataFrame,
    ) -> Result<(), AvinError> {
        // NOTE: в датафрейме может быть данные за
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

        let mut year = utils::dt(first).year();
        let end_year = utils::dt(last).year();

        while year <= end_year {
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
            let path = create_file_path(iid, md, year);
            Cmd::write_pqt(&mut year_df, &path).unwrap();

            year += 1;
        }

        Ok(())
    }
    pub fn load(
        iid: &Iid,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        // create empty df
        let schema = schema::trades_schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // load data by years
        let mut year = begin.year();
        let end_year = end.year();
        while year <= end_year {
            match load_file(iid, md, year) {
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
            let msg = format!("{iid} {md}");
            return Err(AvinError::NotFound(msg));
        }

        Ok(df)
    }
}

fn create_file_path(iid: &Iid, md: MarketData, year: i32) -> PathBuf {
    let mut path = iid.path();
    path.push(md.name());
    path.push(format!("{year}.parquet"));

    path
}
fn load_file(
    iid: &Iid,
    md: MarketData,
    year: i32,
) -> Result<DataFrame, AvinError> {
    let path = create_file_path(iid, md, year);

    // check path is exist
    if !Cmd::is_exist(&path) {
        let msg = format!("{iid} {md}");
        return Err(AvinError::NotFound(msg.to_string()));
    }

    // read file
    match Cmd::read_pqt(&path) {
        Ok(df) => Ok(df),
        Err(why) => {
            let msg = format!("read {} - {}", path.display(), why);
            Err(AvinError::IOError(msg.to_string()))
        }
    }
}
