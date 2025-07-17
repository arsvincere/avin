/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Datelike, Utc};
use polars::prelude::*;

use avin_utils::{self as utils, AvinError, Cmd};

use crate::{Bar, Iid, MarketData};

// TODO: можно это вынести в MarketData, и там уже метод load,
// будет гораздо логичнее.

#[derive(Debug)]
pub struct DataBar {}
impl DataBar {
    pub fn load(
        iid: &Iid,
        market_data: &MarketData,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        // create empty df
        let schema = Bar::schema();
        let mut df = DataFrame::empty_with_schema(&schema);

        // load data by years
        let mut year = begin.year();
        let end_year = end.year();
        while year <= end_year {
            match load_file(iid, market_data, year) {
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
            let msg = format!("{iid} {market_data}");
            return Err(AvinError::NotFound(msg));
        }

        Ok(df)
    }
}

fn load_file(
    iid: &Iid,
    market_data: &MarketData,
    year: i32,
) -> Result<DataFrame, AvinError> {
    // get path
    let mut path = iid.path();
    path.push(market_data.name());
    path.push(format!("{year}.pqt"));

    if !Cmd::is_exist(&path) {
        let msg = format!("{iid} {market_data}");
        return Err(AvinError::NotFound(msg.to_string()));
    }

    match Cmd::read_pqt(&path) {
        Ok(df) => Ok(df),
        Err(why) => {
            let msg = format!("read {} - {}", path.display(), why);
            Err(AvinError::ReadError(msg.to_string()))
        }
    }

    // let data_file = DataBar::new(
    //     iid.clone(),
    //     market_data.clone(),
    //     df,
    //     year,
    // )
    // .unwrap();

    // Ok(data_file)
}
