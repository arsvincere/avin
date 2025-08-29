/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::prelude::*;
use polars::frame::DataFrame;

use crate::data_trades::DataTrades;
use crate::{Iid, MarketData, Source, SourceMoex};
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
    /// Find instrument id by str (case insensitive),
    /// format: "exchange_category_ticker"
    ///
    /// # ru
    /// Поиск идентификатора инструмента по строке (не чувствительно
    /// к регистру). Формат строки: "exchange_category_ticker"
    ///
    /// Информация о доступных инструментах должна быть предварительно
    /// кэширована. Воспользуйтесь консольной утилитой:
    /// "avin-data cache --help".
    /// Подробнее: <https://github.com/arsvincere/avin>
    ///
    /// ## Examples
    /// ```
    /// use avin_data::Manager;
    ///
    /// let iid = Manager::find_iid("moex_share_sber").unwrap();
    /// assert_eq!(iid.name(), "Сбер Банк");
    /// ```
    pub fn find_iid(s: &str) -> Result<Iid, AvinError> {
        IidCache::find_iid(s)
    }
    /// Find instrument id by FIGI - Financial Instrument Global Identifier.
    ///
    /// # ru
    /// Поиск идентификатора инструмента по FIGI - глобальный финансовый
    /// идентификатор инструмента.
    ///
    /// Информация о доступных инструментах должна быть предварительно
    /// кэширована. Воспользуйтесь консольной утилитой:
    /// "avin-data cache --help".
    /// Подробнее: <https://github.com/arsvincere/avin>
    ///
    /// ## Examples
    /// ```
    /// use avin_data::Manager;
    ///
    /// let iid = Manager::find_figi("BBG004730N88").unwrap();
    /// assert_eq!(iid.name(), "Сбер Банк");
    /// ```
    pub fn find_figi(s: &str) -> Result<Iid, AvinError> {
        IidCache::find_figi(s)
    }
    /// Download and save market data.
    ///
    /// # ru
    /// Загрузка рыночных данных и сохранение на диске.
    pub async fn download(
        _source: Source,
        iid: &Iid,
        md: MarketData,
        year: Option<i32>,
    ) -> Result<(), AvinError> {
        if year.is_some() {
            log::info!("Download {iid} {md} for {}", year.unwrap());
        } else {
            log::info!("Download {iid} {md}");
        }

        // download market data
        let result = match md {
            MarketData::BAR_1M => Self::download_bar(iid, md, year).await,
            MarketData::BAR_10M => Self::download_bar(iid, md, year).await,
            MarketData::BAR_1H => Self::download_bar(iid, md, year).await,
            MarketData::BAR_D => Self::download_bar(iid, md, year).await,
            MarketData::BAR_W => Self::download_bar(iid, md, year).await,
            MarketData::BAR_M => Self::download_bar(iid, md, year).await,
            MarketData::TIC => Self::download_tic(iid).await,
            MarketData::TRADE_STATS => Self::download_trades(iid, year).await,
            MarketData::ORDER_STATS => Self::download_orders(iid, year).await,
            MarketData::OB_STATS => Self::download_ob(iid, year).await,
        };

        // check is empty
        let df = result?;
        if df.is_empty() {
            log::info!("No data");
            return Ok(());
        }

        // save data
        match md {
            MarketData::BAR_1M => DataBar::save(iid, md, df),
            MarketData::BAR_10M => DataBar::save(iid, md, df),
            MarketData::BAR_1H => DataBar::save(iid, md, df),
            MarketData::BAR_D => DataBar::save(iid, md, df),
            MarketData::BAR_W => DataBar::save(iid, md, df),
            MarketData::BAR_M => DataBar::save(iid, md, df),
            MarketData::TIC => DataTic::save(iid, md, df),
            MarketData::TRADE_STATS => DataTrades::save(iid, md, df),
            MarketData::ORDER_STATS => todo!(),
            MarketData::OB_STATS => todo!(),
        }
    }
    /// Load market data
    ///
    /// # ru
    /// Загрузка рыночных данных, возвращает polars::DataFrame.
    ///
    /// Рыночные данные должна быть предварительно загружены.
    /// Воспользуйтесь консольной утилитой: "avin-data download --help".
    /// Подробнее: <https://github.com/arsvincere/avin>
    ///
    /// ## Examples
    /// ```
    /// use avin_data::{Manager, MarketData};
    /// use avin_utils as utils;
    ///
    /// let iid = Manager::find_iid("MOEX_SHARE_SBER").unwrap();
    /// let md = MarketData::BAR_1H;
    /// let begin = utils::str_date_to_utc("2024-01-01");
    /// let end = utils::str_date_to_utc("2025-01-01");
    ///
    /// let df = Manager::load(&iid, md, begin, end).unwrap();
    /// println!("{}", df);
    /// ```
    pub fn load(
        iid: &Iid,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        match md {
            MarketData::BAR_1M => DataBar::load(iid, md, begin, end),
            MarketData::BAR_10M => DataBar::load(iid, md, begin, end),
            MarketData::BAR_1H => DataBar::load(iid, md, begin, end),
            MarketData::BAR_D => DataBar::load(iid, md, begin, end),
            MarketData::BAR_W => DataBar::load(iid, md, begin, end),
            MarketData::BAR_M => DataBar::load(iid, md, begin, end),
            MarketData::TIC => DataTic::load(iid, md, begin, end),
            MarketData::TRADE_STATS => DataTrades::load(iid, md, begin, end),
            MarketData::ORDER_STATS => todo!(),
            MarketData::OB_STATS => todo!(),
        }
    }

    async fn download_bar(
        _iid: &Iid,
        _md: MarketData,
        _year: Option<i32>,
    ) -> Result<DataFrame, AvinError> {
        todo!()
    }
    async fn download_tic(_iid: &Iid) -> Result<DataFrame, AvinError> {
        todo!()
    }
    async fn download_trades(
        iid: &Iid,
        year: Option<i32>,
    ) -> Result<DataFrame, AvinError> {
        // choose source
        let source = SourceMoex::new();
        let md = MarketData::TRADE_STATS;
        let year = year.unwrap();
        let from = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let till = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();

        source.get(iid, md, from, till).await
    }
    async fn download_orders(
        _iid: &Iid,
        _year: Option<i32>,
    ) -> Result<DataFrame, AvinError> {
        todo!()
    }
    async fn download_ob(
        _iid: &Iid,
        _year: Option<i32>,
    ) -> Result<DataFrame, AvinError> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use chrono::TimeZone;
//
//     use super::*;
//     use crate::*;
//     use avin_utils as utils;
//
//     #[test]
//     fn request_1m() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_1M;
//         let begin = Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap();
//         let end = Utc.with_ymd_and_hms(2023, 8, 1, 8, 0, 0).unwrap();
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2023, 8, 1, 7, 59, 0).unwrap()
//         );
//     }
//     #[test]
//     fn request_10m() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_10M;
//         let begin = utils::str_dt_to_utc("2023-08-01 10:00:00");
//         let end = utils::str_dt_to_utc("2023-08-01 11:00:00");
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2023, 8, 1, 7, 50, 0).unwrap()
//         );
//     }
//     #[test]
//     fn request_1h() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_1H;
//         let begin = utils::str_dt_to_utc("2023-08-01 10:00:00");
//         let end = utils::str_dt_to_utc("2023-08-01 13:00:00");
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2023, 8, 1, 9, 0, 0).unwrap()
//         );
//     }
//     #[test]
//     fn request_d() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_D;
//         let begin = utils::str_date_to_utc("2023-08-01");
//         let end = utils::str_date_to_utc("2023-09-01");
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap()
//         );
//     }
//     #[test]
//     fn request_w() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_W;
//         let begin = utils::str_date_to_utc("2024-01-01");
//         let end = utils::str_date_to_utc("2025-01-01");
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2024, 12, 29, 21, 0, 0).unwrap()
//         );
//     }
//     #[test]
//     fn request_m() {
//         let iid = Manager::find_iid("moex_share_sber").unwrap();
//         let md = MarketData::BAR_M;
//         let begin = utils::str_date_to_utc("2024-01-01");
//         let end = utils::str_date_to_utc("2025-01-01");
//
//         let df = Manager::load(&iid, md, begin, end).unwrap();
//         let bars = Bar::from_df(&df).unwrap();
//         let first = bars.first().unwrap();
//         let last = bars.last().unwrap();
//
//         assert_eq!(first.dt(), begin);
//         assert_eq!(
//             last.dt(),
//             Utc.with_ymd_and_hms(2024, 11, 30, 21, 0, 0).unwrap()
//         );
//     }
// }
