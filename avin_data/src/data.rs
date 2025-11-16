/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{TimeZone, Utc};
use polars::frame::UniqueKeepStrategy;
use strum::IntoEnumIterator;

use avin_core::{Bar, Category, Exchange, Iid, Manager, MarketData, Source};
use avin_utils::{AvinError, CFG, Cmd};

use super::source_tinkoff::SourceTinkoff;

/// Fasade class for operations with market data.
///
/// # ru
/// Фасадный класс для операций с рыночными данными от разных
/// источников. Скачивание, преобразование, обновление.
pub struct Data {}
impl Data {
    /// Return all available Iid.
    ///
    /// # ru
    /// Возвращает список всех доступных инструментов (для которых
    /// имеются скачанные данные).
    pub fn all_iid() -> Vec<Iid> {
        let mut all_iid = Vec::new();

        for exchange in Exchange::iter() {
            for category in Category::iter() {
                // TODO: блять да че же делать с этими ебучими индексами
                if category == Category::INDEX {
                    continue;
                }
                // TODO: сделать кэширование фьючей с Т
                if category == Category::FUTURE {
                    continue;
                }

                let mut category_path = CFG.dir.data();
                category_path.push(exchange.name());
                category_path.push(category.name());
                if !category_path.exists() {
                    continue;
                }

                let tickers = Cmd::get_dirs(&category_path).unwrap();

                for ticker in tickers {
                    let s = format!(
                        "{}_{}_{}",
                        exchange.name(),
                        category.name(),
                        Cmd::name(&ticker).unwrap()
                    );
                    let iid = Manager::find_iid(&s).unwrap();

                    all_iid.push(iid);
                }
            }
        }

        all_iid
    }
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
    /// Find instrument ID by str (case insensitive),
    /// format: "exchange_category_ticker"
    ///
    /// # ru
    /// Поиск идентификатора инструмента по строке (не чувствительно
    /// к регистру). Формат строки: "exchange_category_ticker"
    pub fn find(s: &str) -> Result<Iid, AvinError> {
        Manager::find_iid(s)
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
        let mut output_bars = convert(input, output, &bars);

        // NOTE: после конвертации последний бар может быть незавершенным
        // тупо отбрасываем его
        output_bars.pop();

        // save
        let output_df = Bar::to_df(&output_bars).unwrap();
        Manager::save(iid, source, output, &output_df)?;

        Ok(())
    }
    /// Update market data.
    ///
    /// # ru
    /// Обновить данные по одному инструменту.
    pub async fn update(
        iid: &Iid,
        source: Source,
        md: MarketData,
    ) -> Result<(), AvinError> {
        log::info!("Update {iid} {md} from {source}");

        // load last
        let mut df = Manager::load_last(iid, source, md)?;

        // get last timestamp
        let ts = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .last()
            .unwrap();

        // begin/end DateTime
        let begin = avin_utils::dt(ts);
        let end = Utc::now();

        // request new bars, tinkoff return only historical, completed bars
        let new = SourceTinkoff::get_bars(iid, md, begin, end).await.unwrap();

        // join df
        df.extend(&new).unwrap();

        // drop duplicated rows
        let col_name = String::from("ts_nanos");
        df = df
            .unique_stable(Some(&[col_name]), UniqueKeepStrategy::Last, None)
            .unwrap();

        Manager::save(iid, source, md, &df).unwrap();

        Ok(())
    }
    /// Update all available market data.
    ///
    /// # ru
    /// Обновить все скаченные данные.
    pub async fn update_all() -> Result<(), AvinError> {
        log::info!("Update all market data");

        // check data dir
        let data_dir = CFG.dir.data();
        if !data_dir.exists() {
            let msg = format!("data dir {}", data_dir.display());
            let e = AvinError::NotFound(msg);
            return Err(e);
        }

        let all_iid = Data::all_iid();
        for iid in all_iid.iter() {
            for source in Source::iter() {
                for md in MarketData::iter() {
                    // BUG: если MOEXALGO или тики или стаканы хз как лучше...
                    if source == Source::MOEXALGO
                        || md == MarketData::TIC
                        || md == MarketData::ORDER_BOOK
                    {
                        continue;
                    }

                    let mut path = iid.path();
                    path.push(source.name());
                    path.push(md.name());

                    if path.exists() {
                        Data::update(iid, source, md).await?;
                    }
                }
            }
        }

        Ok(())
    }
    /// Write real time tics and order book data.
    ///
    /// # ru
    /// Реал-тайм запись тиков и данных по стакану.
    pub async fn record() -> Result<(), AvinError> {
        SourceTinkoff::record().await
    }
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
        },
        MarketData::ORDER_BOOK => match output {
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
        },
        MarketData::SC_TRADE => match output {
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
        },
        MarketData::SC_ORDER => match output {
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
        },
        MarketData::SC_OB => match output {
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
            MarketData::ORDER_BOOK => false,
            MarketData::SC_TRADE => false,
            MarketData::SC_ORDER => false,
            MarketData::SC_OB => false,
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
