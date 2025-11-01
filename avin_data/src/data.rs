/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::{Bar, Iid, Manager, MarketData, Source};
use avin_utils::AvinError;
use chrono::{TimeZone, Utc};

use crate::SourceTinkoff;

/// Fasade class for operations with market data.
///
/// # ru
/// Фасадный класс для операций с рыночными данными от разных
/// источников. Скачивание, преобразование, обновление.
pub struct Data {}
impl Data {
    /// Make cache of instruments info.
    ///
    /// # ru
    /// Кешировать информацию об инструментах.
    pub async fn cache(source: Source) -> Result<(), AvinError> {
        log::info!("Cache instruments info from {source}");

        match source {
            Source::MOEXALGO => todo!(),
            Source::TINKOFF => SourceTinkoff::cache().await,
        }
    }
    /// Download and save market data.
    ///
    /// # ru
    /// Загрузка рыночных данных и сохранение на диске.
    pub async fn download(
        iid: &Iid,
        source: Source,
        md: MarketData,
        year: i32,
    ) -> Result<(), AvinError> {
        log::info!("Download {iid} {md} {year} from {source}");

        match source {
            Source::MOEXALGO => todo!(),
            Source::TINKOFF => SourceTinkoff::download(iid, md, year),
        }
    }
    /// Convert market data.
    ///
    /// # ru
    /// Преобразование рыночных данных.
    ///
    /// Можно преобразовывать бары младшиих таймфреймов в старшие.
    /// Тики в кластеры.
    pub fn convert(
        iid: &Iid,
        source: Source,
        input: MarketData,
        output: MarketData,
    ) -> Result<(), AvinError> {
        log::info!("Convert {iid}/{source} {input} -> {output}");

        // check convertation is possible
        if !is_convertation_possibe(input, output) {
            let msg = format!("Impossible convert from {input} -> {output}");
            let e = AvinError::InvalidValue(msg);
            return Err(e);
        }

        // load input data
        let end = Utc::now();
        let begin = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
        let input_df = Manager::load(iid, source, input, begin, end)?;

        // convert
        let bars = Bar::from_df(&input_df).unwrap();
        let output_bars = convert(input, output, &bars);
        let output_df = Bar::to_df(&output_bars).unwrap();

        // save
        Manager::save(iid, source, output, output_df)?;

        Ok(())
    }

    pub async fn update(
        iid: &Iid,
        source: Source,
        md: MarketData,
    ) -> Result<(), AvinError> {
        log::info!("Update {iid} {md} from {source}");
        todo!()
    }
    pub async fn update_all() -> Result<(), AvinError> {
        todo!()
    }

    // /// Load market data
    // ///
    // /// # ru
    // /// Загрузка рыночных данных, возвращает polars::DataFrame.
    // ///
    // /// Рыночные данные должна быть предварительно загружены.
    // /// Воспользуйтесь консольной утилитой: "avin-data download --help".
    // ///
    // /// ## Examples
    // /// ```
    // /// use avin_core::MarketData;
    // /// use avin_data::Manager;
    // /// use avin_utils as utils;
    // ///
    // /// let iid = Manager::find_iid("MOEX_SHARE_SBER").unwrap();
    // /// let md = MarketData::BAR_1H;
    // /// let begin = utils::str_date_to_utc("2024-01-01");
    // /// let end = utils::str_date_to_utc("2025-01-01");
    // ///
    // /// let df = Manager::load(&iid, md, begin, end).unwrap();
    // /// println!("{}", df);
    // /// ```
    // pub fn load(
    //     _source: Source,
    //     iid: &Iid,
    //     md: MarketData,
    //     begin: DateTime<Utc>,
    //     end: DateTime<Utc>,
    // ) -> Result<DataFrame, AvinError> {
    //     match md {
    //         MarketData::BAR_1M => DataBar::load(iid, md, begin, end),
    //         MarketData::BAR_10M => DataBar::load(iid, md, begin, end),
    //         MarketData::BAR_1H => DataBar::load(iid, md, begin, end),
    //         MarketData::BAR_DAY => DataBar::load(iid, md, begin, end),
    //         MarketData::BAR_WEEK => DataBar::load(iid, md, begin, end),
    //         MarketData::BAR_MONTH => DataBar::load(iid, md, begin, end),
    //         MarketData::TIC => DataTic::load(iid, md, begin, end),
    //         MarketData::TRADE_STATS => DataTrades::load(iid, md, begin, end),
    //         MarketData::ORDER_STATS => DataOrders::load(iid, md, begin, end),
    //         MarketData::OB_STATS => DataOB::load(iid, md, begin, end),
    //     }
    // }
}

fn is_convertation_possibe(input: MarketData, output: MarketData) -> bool {
    match input {
        MarketData::BAR_1M => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => true,
            MarketData::BAR_10M => true,
            MarketData::BAR_15M => true,
            MarketData::BAR_1H => true,
            MarketData::BAR_4H => true,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_5M => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => true,
            MarketData::BAR_15M => true,
            MarketData::BAR_1H => true,
            MarketData::BAR_4H => true,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_10M => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => true,
            MarketData::BAR_4H => true,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_15M => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => true,
            MarketData::BAR_4H => true,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_1H => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => true,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_4H => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => true,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_DAY => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => true,
            MarketData::BAR_MONTH => true,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_WEEK => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::BAR_MONTH => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::TIC => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::TRADE_STATS => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::ORDER_STATS => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
        MarketData::OB_STATS => match output {
            MarketData::BAR_1M => false,
            MarketData::BAR_5M => false,
            MarketData::BAR_10M => false,
            MarketData::BAR_15M => false,
            MarketData::BAR_1H => false,
            MarketData::BAR_4H => false,
            MarketData::BAR_DAY => false,
            MarketData::BAR_WEEK => false,
            MarketData::BAR_MONTH => false,
            MarketData::TIC => false,
            MarketData::TRADE_STATS => false,
            MarketData::ORDER_STATS => false,
            MarketData::OB_STATS => false,
        },
    }
}
fn convert(_input: MarketData, output: MarketData, bars: &[Bar]) -> Vec<Bar> {
    assert!(!bars.is_empty());

    // выходные склеенные бары
    let mut out_bars = Vec::new();

    // Конвертируем output MarketData в таймфрейм
    let tf = output.timeframe().unwrap();

    // Проходим по всему вектору баров
    let mut i = 0;
    while i < bars.len() {
        // joined_bar - выходной склеенный бар
        let mut joined_bar = *bars.get(i).unwrap();

        // Таймштампы начала и конца выходного склеенного бара
        let begin = tf.prev_ts(joined_bar.ts);
        let end = tf.next_ts(joined_bar.ts);

        // NOTE: допустим конвертим 1М -> D
        // тогда у joined_bar будет время 06:59, а должно быть 00:00,
        // поэтому присваиваем ему рассчитанный prev_ts
        // prev_ts - корректное начальное время склееного бара
        joined_bar.ts = begin;

        let mut j = i + 1;
        while j < bars.len() {
            let bar = *bars.get(j).unwrap();
            if bar.ts < end {
                joined_bar = Bar::join(joined_bar, bar);
                j += 1;
            } else {
                break;
            }
        }

        // сохраняем склеенный бар в векторе
        out_bars.push(joined_bar);

        i = j;
    }

    out_bars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_bars() {
        // Возьмем для примера 3 бара 5М и сконвертим их в 10М
        let dt1 = Utc.with_ymd_and_hms(2025, 1, 1, 10, 0, 0).unwrap();
        let dt2 = Utc.with_ymd_and_hms(2025, 1, 1, 10, 5, 0).unwrap();
        let dt3 = Utc.with_ymd_and_hms(2025, 1, 1, 10, 10, 0).unwrap();

        let ts1 = avin_utils::ts(dt1);
        let ts2 = avin_utils::ts(dt2);
        let ts3 = avin_utils::ts(dt3);

        let b1 = Bar::new(ts1, 100.0, 110.0, 90.0, 105.0, 5000);
        let b2 = Bar::new(ts2, 101.0, 120.0, 80.0, 106.0, 6000);
        let b3 = Bar::new(ts3, 102.0, 130.0, 70.0, 107.0, 7000);

        let bars = [b1, b2, b3];
        let input = MarketData::BAR_5M;
        let output = MarketData::BAR_10M;

        // сконвертим
        let out_bars = convert(input, output, &bars);

        // На выходе должно получиться два бара
        assert_eq!(out_bars.len(), 2);
        let out1 = *out_bars.first().unwrap();
        let out2 = *out_bars.get(1).unwrap();

        assert_eq!(out1.ts, b1.ts);
        assert_eq!(out1.o, b1.o);
        assert_eq!(out1.h, b2.h);
        assert_eq!(out1.l, b2.l);
        assert_eq!(out1.c, b2.c);
        assert_eq!(out1.v, b1.v + b2.v);

        assert_eq!(out2.ts, b3.ts);
        assert_eq!(out2.o, b3.o);
        assert_eq!(out2.h, b3.h);
        assert_eq!(out2.l, b3.l);
        assert_eq!(out2.c, b3.c);
        assert_eq!(out2.v, b3.v);
    }
}
