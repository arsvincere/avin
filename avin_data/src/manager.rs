/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Utc};
use polars::frame::DataFrame;

use crate::{Iid, MarketData};
use avin_utils::AvinError;

use super::data_bar::DataBar;
use super::data_tic::DataTic;
use super::iid_cache::IidCache;

/// Fasade class for operations with market data.
///
/// # ru
/// Фасадный класс для операций с рыночными данными.
pub struct Manager {}
impl Manager {
    pub fn find_iid(s: &str) -> Result<Iid, AvinError> {
        IidCache::find_iid(s)
    }
    pub fn find_figi(s: &str) -> Result<Iid, AvinError> {
        IidCache::find_figi(s)
    }
    pub fn load(
        iid: &Iid,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        match md {
            MarketData::TIC => DataTic::load(iid, md, begin, end),
            MarketData::BAR_1M => DataBar::load(iid, md, begin, end),
            MarketData::BAR_5M => todo!(),
            MarketData::BAR_10M => DataBar::load(iid, md, begin, end),
            MarketData::BAR_1H => DataBar::load(iid, md, begin, end),
            MarketData::BAR_D => DataBar::load(iid, md, begin, end),
            MarketData::BAR_W => DataBar::load(iid, md, begin, end),
            MarketData::BAR_M => DataBar::load(iid, md, begin, end),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::*;
    use avin_utils as utils;

    #[test]
    fn request_1m() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_1M;
        let begin = Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2023, 8, 1, 8, 0, 0).unwrap();

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2023, 8, 1, 7, 59, 0).unwrap()
        );
    }
    #[test]
    fn request_10m() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_10M;
        let begin = utils::str_dt_to_utc("2023-08-01 10:00:00");
        let end = utils::str_dt_to_utc("2023-08-01 11:00:00");

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2023, 8, 1, 7, 50, 0).unwrap()
        );
    }
    #[test]
    fn request_1h() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_1H;
        let begin = utils::str_dt_to_utc("2023-08-01 10:00:00");
        let end = utils::str_dt_to_utc("2023-08-01 13:00:00");

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2023, 8, 1, 9, 0, 0).unwrap()
        );
    }
    #[test]
    fn request_d() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_D;
        let begin = utils::str_date_to_utc("2023-08-01");
        let end = utils::str_date_to_utc("2023-09-01");

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap()
        );
    }
    #[test]
    fn request_w() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_W;
        let begin = utils::str_date_to_utc("2024-01-01");
        let end = utils::str_date_to_utc("2025-01-01");

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2024, 12, 29, 21, 0, 0).unwrap()
        );
    }
    #[test]
    fn request_m() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let md = MarketData::BAR_M;
        let begin = utils::str_date_to_utc("2024-01-01");
        let end = utils::str_date_to_utc("2025-01-01");

        let df = Manager::load(&iid, md, begin, end).unwrap();
        let bars = Bar::from_df(&df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2024, 11, 30, 21, 0, 0).unwrap()
        );
    }
}
