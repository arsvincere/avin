/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_utils::AvinError;
use chrono::{DateTime, Utc};
use polars::frame::DataFrame;

use crate::{Category, Iid, MarketData, Source};

use super::data_bar::DataBar;
use super::data_ob::DataOB;
use super::data_sc_ob::DataSCOB;
use super::data_sc_order::DataSCOrder;
use super::data_sc_trade::DataSCTrade;
use super::data_tic::DataTic;
use super::iid_cache::IidCache;

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
    ///
    /// ## Examples
    /// ```
    /// use avin_core::Manager;
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
    /// use avin_core::Manager;
    ///
    /// let iid = Manager::find_figi("BBG004730N88").unwrap();
    /// assert_eq!(iid.name(), "Сбер Банк");
    /// ```
    pub fn find_figi(s: &str) -> Result<Iid, AvinError> {
        IidCache::find_figi(s)
    }
    /// Save market data.
    ///
    /// # ru
    /// Сохранение рыночных данных.
    ///
    /// Раскладывает данные по папкам:
    /// 1. Источник данных
    /// 2. Биржа
    /// 3. Категория инструмента
    /// 4. Тикер
    /// 5. Тип данных
    ///
    /// Далее для тиков идет разбивка датафрейма на файлы по дням
    /// (в одном файле тики за один день). Для баров разбивка по годам.
    pub fn save(
        iid: &Iid,
        source: Source,
        md: MarketData,
        df: &DataFrame,
    ) -> Result<(), AvinError> {
        match md {
            MarketData::BAR_1M => DataBar::save(iid, source, md, df),
            MarketData::BAR_5M => DataBar::save(iid, source, md, df),
            MarketData::BAR_10M => DataBar::save(iid, source, md, df),
            MarketData::BAR_15M => DataBar::save(iid, source, md, df),
            MarketData::BAR_1H => DataBar::save(iid, source, md, df),
            MarketData::BAR_4H => DataBar::save(iid, source, md, df),
            MarketData::BAR_DAY => DataBar::save(iid, source, md, df),
            MarketData::BAR_WEEK => DataBar::save(iid, source, md, df),
            MarketData::BAR_MONTH => DataBar::save(iid, source, md, df),
            MarketData::TIC => DataTic::save(iid, source, md, df),
            MarketData::ORDER_BOOK => DataOB::save(iid, source, md, df),
            MarketData::SC_TRADE => DataSCTrade::save(iid, md, df),
            MarketData::SC_ORDER => DataSCOrder::save(iid, md, df),
            MarketData::SC_OB => DataSCOB::save(iid, md, df),
        }
    }
    /// Load market data.
    ///
    /// # ru
    /// Загрузка рыночных данных, возвращает polars::DataFrame.
    ///
    /// Рыночные данные должна быть предварительно скачаны.
    /// Воспользуйтесь консольной утилитой: "avin-data download --help".
    ///
    /// ## Examples
    /// ```
    /// use avin_core::{Manager, Source, MarketData};
    /// use avin_utils as utils;
    ///
    /// let iid = Manager::find_iid("MOEX_SHARE_SBER").unwrap();
    /// let source = Source::TINKOFF;
    /// let begin = utils::str_date_to_utc("2024-01-01");
    /// let end = utils::str_date_to_utc("2025-01-01");
    /// let md = MarketData::BAR_1H;
    ///
    /// let df = Manager::load(&iid, source, md, begin, end).unwrap();
    /// println!("{}", df);
    /// ```
    pub fn load(
        iid: &Iid,
        source: Source,
        md: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        match md {
            MarketData::BAR_1M => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_5M => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_10M => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_15M => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_1H => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_4H => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_DAY => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_WEEK => DataBar::load(iid, source, md, begin, end),
            MarketData::BAR_MONTH => DataBar::load(iid, source, md, begin, end),
            MarketData::TIC => DataTic::load(iid, source, md, begin, end),
            MarketData::ORDER_BOOK => DataOB::load(iid, source, md, begin, end),
            MarketData::SC_TRADE => DataSCTrade::load(iid, md, begin, end),
            MarketData::SC_ORDER => DataSCOrder::load(iid, md, begin, end),
            MarketData::SC_OB => DataSCOB::load(iid, md, begin, end),
        }
    }
    pub fn load_last(
        iid: &Iid,
        source: Source,
        md: MarketData,
    ) -> Result<DataFrame, AvinError> {
        match md {
            MarketData::BAR_1M => DataBar::load_last(iid, source, md),
            MarketData::BAR_5M => DataBar::load_last(iid, source, md),
            MarketData::BAR_10M => DataBar::load_last(iid, source, md),
            MarketData::BAR_15M => DataBar::load_last(iid, source, md),
            MarketData::BAR_1H => DataBar::load_last(iid, source, md),
            MarketData::BAR_4H => DataBar::load_last(iid, source, md),
            MarketData::BAR_DAY => DataBar::load_last(iid, source, md),
            MarketData::BAR_WEEK => DataBar::load_last(iid, source, md),
            MarketData::BAR_MONTH => DataBar::load_last(iid, source, md),
            MarketData::TIC => todo!(),
            MarketData::ORDER_BOOK => todo!(),
            MarketData::SC_TRADE => todo!(),
            MarketData::SC_ORDER => todo!(),
            MarketData::SC_OB => todo!(),
        }
    }

    /// Save instruments cache.
    ///
    /// # ru
    /// Сохраняет кэш информации об инструментах.
    pub fn save_cache(
        source: Source,
        category: Category,
        iid_df: DataFrame,
    ) -> Result<(), AvinError> {
        IidCache::save(source, category, iid_df)
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::*;

    #[test]
    fn request_1m() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_1M;
        let begin = Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2023, 8, 1, 8, 0, 0).unwrap();

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_10M;
        let begin = avin_utils::str_dt_to_utc("2023-08-01 10:00:00");
        let end = avin_utils::str_dt_to_utc("2023-08-01 11:00:00");

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_1H;
        let begin = avin_utils::str_dt_to_utc("2023-08-01 10:00:00");
        let end = avin_utils::str_dt_to_utc("2023-08-01 13:00:00");

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
    fn request_day() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_DAY;
        let begin = avin_utils::str_date_to_utc("2023-08-01");
        let end = avin_utils::str_date_to_utc("2023-09-01");

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
    fn request_week() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_WEEK;
        let begin = avin_utils::str_date_to_utc("2024-01-01");
        let end = avin_utils::str_date_to_utc("2025-01-01");

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
    fn request_month() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let source = Source::MOEXALGO;
        let md = MarketData::BAR_MONTH;
        let begin = avin_utils::str_date_to_utc("2024-01-01");
        let end = avin_utils::str_date_to_utc("2025-01-01");

        let df = Manager::load(&iid, source, md, begin, end).unwrap();
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
