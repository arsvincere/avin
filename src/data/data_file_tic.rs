/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::PathBuf;

use chrono::{Datelike, NaiveDate};
use polars::prelude::*;

use crate::utils::{self, Cmd};

use super::IID;
use super::error::DataError;
use super::market_data::MarketData;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DataFileTic {
    iid: IID,
    market_data: MarketData,
    df: DataFrame,
}
#[allow(dead_code)]
impl DataFileTic {
    // build
    pub fn new(
        iid: &IID,
        md: &MarketData,
        df: DataFrame,
    ) -> Result<DataFileTic, DataError> {
        // check
        assert_eq!(first_day(&df), last_day(&df));
        assert_eq!(*md, MarketData::TIC);

        let data_file = Self {
            iid: iid.clone(),
            market_data: md.clone(),
            df,
        };

        Ok(data_file)
    }
    pub fn save(data_file: &mut DataFileTic) -> Result<(), &'static str> {
        let file_path = create_path(
            &data_file.iid,
            &data_file.market_data,
            &first_day(&data_file.df),
        );
        Cmd::write_pqt(&mut data_file.df, &file_path).unwrap();

        log::info!("Save tics {}", file_path.display());
        Ok(())
    }
    pub fn load(
        iid: &IID,
        md: &MarketData,
        day: &NaiveDate,
    ) -> Result<DataFileTic, DataError> {
        // get path
        let path = create_path(iid, md, day);

        if !Cmd::is_exist(&path) {
            let msg = format!("{} {}", iid, md);
            return Err(DataError::NotFound(msg.to_string()));
        }

        let df = match Cmd::read_pqt(&path) {
            Ok(df) => df,
            Err(why) => {
                let msg = format!("read {} - {}", path.display(), why);
                return Err(DataError::ReadError(msg.to_string()));
            }
        };

        DataFileTic::new(iid, md, df)
    }
    // pub fn request_all(
    //     iid: &IID,
    //     market_data: &MarketData,
    // ) -> Result<Vec<DataFileTic>, &'static str> {
    //     // dir path
    //     let mut dir_path = iid.path();
    //     dir_path.push(&market_data.name());
    //
    //     // get files
    //     let file_paths = Cmd::get_files(&dir_path).unwrap();
    //
    //     // read parquet files & create DataFileTic objs
    //     let mut all_data_files = Vec::new();
    //     for path in file_paths {
    //         let day: i32 = path
    //             .file_stem()
    //             .unwrap()
    //             .to_str()
    //             .unwrap()
    //             .trim()
    //             .parse()
    //             .unwrap();
    //         let df = Cmd::read_pqt(&path).unwrap();
    //         let data_file =
    //             DataFileTic::new(iid, market_data.clone(), day, df).unwrap();
    //
    //         all_data_files.push(data_file);
    //     }
    //
    //     Ok(all_data_files)
    // }

    // getter
    pub fn iid(&self) -> &IID {
        &self.iid
    }
    pub fn market_data(&self) -> &MarketData {
        &self.market_data
    }
    pub fn df(&self) -> &DataFrame {
        &self.df
    }
}

fn create_path(iid: &IID, md: &MarketData, day: &NaiveDate) -> PathBuf {
    let mut path = iid.path();
    path.push(md.name());
    path.push(day.year().to_string());
    path.push(format!("{}.pqt", day.format("%Y-%m-%d")));

    path
}
fn first_day(df: &DataFrame) -> NaiveDate {
    let ts_nanos = df
        .column("ts_nanos")
        .unwrap()
        .i64()
        .unwrap()
        .first()
        .unwrap();

    let dt = utils::dt(ts_nanos);
    let date = dt.date_naive();

    date
}
fn last_day(df: &DataFrame) -> NaiveDate {
    let ts_nanos = df
        .column("ts_nanos")
        .unwrap()
        .i64()
        .unwrap()
        .last()
        .unwrap();

    let dt = utils::dt(ts_nanos);
    let date = dt.date_naive();

    date
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn first_last() {
        let day_1 = NaiveDate::from_ymd_opt(2025, 6, 9).unwrap();
        let _day_2 = NaiveDate::from_ymd_opt(2025, 6, 10).unwrap();
        let day_3 = NaiveDate::from_ymd_opt(2025, 6, 11).unwrap();

        let ts_1 =
            utils::ts(&Utc.with_ymd_and_hms(2025, 6, 9, 10, 8, 48).unwrap());
        let ts_2 =
            utils::ts(&Utc.with_ymd_and_hms(2025, 6, 10, 10, 8, 48).unwrap());
        let ts_3 =
            utils::ts(&Utc.with_ymd_and_hms(2025, 6, 11, 10, 8, 48).unwrap());

        let df = df!(
            "ts_nanos" => [ts_1, ts_2, ts_3],
        )
        .unwrap();

        assert_eq!(first_day(&df), day_1);
        assert_eq!(last_day(&df), day_3);
    }
    #[test]
    fn load() {
        let path =
            "/home/alex/trading/data/MOEX/SHARE/GAZP/TIC/2025/2025-06-08.pqt";
        let path = std::path::Path::new(path);
        let df = Cmd::read_pqt(path).unwrap();

        let iid = crate::Manager::find("moex_share_gazp").unwrap();
        let md = MarketData::TIC;
        let day = NaiveDate::from_ymd_opt(2025, 6, 8).unwrap();
        let loaded = DataFileTic::load(&iid, &md, &day).unwrap();

        assert_eq!(&df, loaded.df());
    }
}
