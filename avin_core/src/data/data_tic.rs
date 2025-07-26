/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Datelike, Days, NaiveDate, Utc};
use polars::prelude::*;

use avin_utils::{self as utils, AvinError, Cmd};

use crate::{Iid, MarketData, Tic};

// TODO: можно это вынести в MarketData, и там уже метод load,
// будет гораздо логичнее.

#[derive(Debug)]
pub struct DataTic {}
impl DataTic {
    pub fn load(
        iid: &Iid,
        market_data: &MarketData,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        // create empty df
        let schema = Tic::schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // load data by days
        let mut day = begin.date_naive();
        let end_day = end.date_naive();
        while day <= end_day {
            match Self::load_file(iid, market_data, &day) {
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
            let msg = format!("{iid} {market_data}");
            return Err(AvinError::NotFound(msg));
        }

        Ok(df)
    }
    pub fn load_file(
        iid: &Iid,
        md: &MarketData,
        day: &NaiveDate,
    ) -> Result<DataFrame, AvinError> {
        // get path
        let mut path = iid.path();
        path.push(md.name());
        path.push(day.year().to_string());
        path.push(format!("{}.parquet", day.format("%Y-%m-%d")));

        if !Cmd::is_exist(&path) {
            let msg = format!("{iid} {md}");
            return Err(AvinError::NotFound(msg.to_string()));
        }

        match Cmd::read_pqt(&path) {
            Ok(df) => Ok(df),
            Err(why) => {
                let msg = format!("read {} - {}", path.display(), why);
                Err(AvinError::ReadError(msg.to_string()))
            }
        }
    }
}
