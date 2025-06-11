/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{Days, prelude::*};
use polars::prelude::{DataFrame, DataType, Field, IntoLazy, Schema, col};

use crate::Tinkoff;

use super::category::Category;
use super::data_file_bar::DataFileBar;
use super::data_file_tic::DataFileTic;
use super::error::DataError;
use super::iid::IID;
use super::iid_cache::IidCache;
use super::market_data::MarketData;
use super::source::Source;
use super::source_moex::SourceMoex;

pub struct Manager {}
impl Manager {
    pub async fn cache(source: &Source) -> Result<(), &'static str> {
        println!(":: Caching {}", source.to_string());

        match source {
            Source::TINKOFF => cache_tinkoff().await,
            Source::MOEX => todo!(),
            Source::CONVERTER => panic!(),
        }
    }
    pub async fn download(
        source: &Source,
        iid: &IID,
        market_data: &MarketData,
        year: Option<i32>,
    ) -> Result<(), &'static str> {
        let source = match source {
            Source::MOEX => SourceMoex::new(),
            Source::TINKOFF => panic!("Нахер с Тинькофф качать?"),
            Source::CONVERTER => panic!(),
        };
        println!(":: Download {} {}", iid.ticker(), market_data.name());

        match year {
            Some(year) => {
                download_year(&source, &iid, &market_data, year).await
            }
            None => download_all_availible(&source, &iid, &market_data).await,
        }
    }
    pub fn find(s: &str) -> Result<IID, &'static str> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 3 {
            eprintln!("Fail to create IID from str: {s}");
            return Err("Invalid IID");
        };

        // TODO: пока работает только биржа MOEX
        let exchange = parts[0].to_uppercase();
        assert_eq!(exchange, "MOEX");

        // TODO: пока работает только тип инструмента SHARE
        let category = parts[1].to_uppercase();
        assert_eq!(category, "SHARE");

        let ticker = parts[2].to_uppercase();

        // loading instruments cache
        let iid = IidCache::find(&exchange, &category, &ticker);

        match iid {
            Some(iid) => Ok(iid),
            None => Err("instrument not found"),
        }
    }
    pub fn find_figi(s: &str) -> Result<IID, &'static str> {
        // loading instruments cache
        let iid = IidCache::find_figi(s);

        match iid {
            Some(iid) => Ok(iid),
            None => Err("instrument not found"),
        }
    }
    pub fn convert(
        iid: &IID,
        in_t: &MarketData,
        out_t: &MarketData,
    ) -> Result<(), &'static str> {
        println!(
            ":: Convert {} {} -> {}",
            iid.ticker(),
            in_t.name(),
            out_t.name(),
        );

        // load data files
        let data = DataFileBar::request_all(iid, in_t)?;
        if data.len() == 0 {
            return Err("   - no data files");
        }

        // convert timeframe
        for i in data {
            convert_timeframe(&i, in_t, out_t)?;
        }

        // сохранить

        println!("Convert complete!");
        Ok(())
    }
    pub fn request(
        iid: &IID,
        market_data: &MarketData,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<DataFrame, DataError> {
        match market_data {
            MarketData::TIC => request_tic(iid, market_data, begin, end),
            MarketData::BAR_1M => request_bar(iid, market_data, begin, end),
            MarketData::BAR_5M => todo!(),
            MarketData::BAR_10M => request_bar(iid, market_data, begin, end),
            MarketData::BAR_1H => request_bar(iid, market_data, begin, end),
            MarketData::BAR_D => request_bar(iid, market_data, begin, end),
            MarketData::BAR_W => request_bar(iid, market_data, begin, end),
            MarketData::BAR_M => request_bar(iid, market_data, begin, end),
        }
    }
}

// private
// TODO: move func schema in core struct
fn bar_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("open".into(), DataType::Float64),
        Field::new("high".into(), DataType::Float64),
        Field::new("low".into(), DataType::Float64),
        Field::new("close".into(), DataType::Float64),
        Field::new("volume".into(), DataType::Int64),
        Field::new("value".into(), DataType::Float64),
    ])
}
fn tic_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("direction".into(), DataType::String),
        Field::new("lots".into(), DataType::Int64),
        Field::new("price".into(), DataType::Float64),
        Field::new("value".into(), DataType::Float64),
        Field::new("session".into(), DataType::Int8),
        Field::new("tradeno".into(), DataType::Int64),
    ])
}
fn filter_dt(
    begin: &DateTime<Utc>,
    end: &DateTime<Utc>,
    df: DataFrame,
) -> DataFrame {
    // filter begin end datetime by timestamp
    let b = begin.timestamp_nanos_opt().unwrap_or(0);
    let e = end.timestamp_nanos_opt().unwrap();

    let df = df
        .lazy()
        .filter(col("ts_nanos").gt_eq(b))
        .filter(col("ts_nanos").lt(e))
        .collect()
        .unwrap();

    df
}

async fn cache_tinkoff() -> Result<(), &'static str> {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let mut b = Tinkoff::new(tx);
    b.connect().await.unwrap();

    let shares = b.get_shares().await.unwrap();

    let mut iids = Vec::new();
    for share in shares {
        iids.push(share.iid().clone());
    }

    let source = Source::TINKOFF;
    let category = Category::SHARE;
    let cache = IidCache::new(source, category, iids);

    IidCache::save(&cache)?;

    Ok(())
}
async fn download_year(
    source: &SourceMoex,
    iid: &IID,
    market_data: &MarketData,
    year: i32,
) -> Result<(), &'static str> {
    let begin = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();
    let df = source.get_bars(&iid, &market_data, &begin, &end).await?;

    if df.is_empty() {
        return Err("   - no data for {year}");
    }

    // NOTE: ParquetWriter требует &mut df для сохранения...
    // по факту никто data_file не меняет перед записью
    let mut data_file =
        DataFileBar::new(iid, market_data.clone(), df, year).unwrap();
    DataFileBar::save(&mut data_file)?;

    println!("Download complete!");
    Ok(())
}
async fn download_all_availible(
    source: &SourceMoex,
    iid: &IID,
    market_data: &MarketData,
) -> Result<(), &'static str> {
    let mut year: i32 = 1990; // суть - более старых данных точно нет
    let now_year = Utc::now().year();

    while year <= now_year {
        let begin = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();
        let df = source.get_bars(&iid, &market_data, &begin, &end).await?;

        if df.is_empty() {
            println!("   - no data for {year}");
            year += 1;
            continue;
        }

        // NOTE: ParquetWriter требует &mut df для сохранения...
        // по факту никто data_file не меняет перед записью
        let mut data_file =
            DataFileBar::new(iid, market_data.clone(), df, year).unwrap();
        DataFileBar::save(&mut data_file)?;
        year += 1;
    }

    println!("Download complete!");
    Ok(())
}
fn convert_timeframe(
    _data: &DataFileBar,
    _in_t: &MarketData,
    _out_t: &MarketData,
) -> Result<(), &'static str> {
    todo!();

    // NOTE: old python code convert timeframe
    //
    // bars = cls.__fillVoid(bars, in_type)
    // period = out_type.toTimeDelta()
    //
    // converted = list()
    // i = 0
    // while i < len(bars):
    //     first = i
    //     last = i
    //     while last < len(bars):
    //         time_dif = bars[last].dt - bars[first].dt
    //         if time_dif < period:
    //             last += 1
    //         else:
    //             break
    //
    //     new_bar = cls.__join(bars[first:last])
    //     if new_bar is not None:
    //         converted.append(new_bar)
    //
    //     i = last
    //
    // return converted
}
fn request_bar(
    iid: &IID,
    market_data: &MarketData,
    begin: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> Result<DataFrame, DataError> {
    // create empty df
    let schema = bar_schema();
    let mut df = DataFrame::empty_with_schema(&schema);

    // load data by years
    let mut year = begin.year();
    let end_year = end.year();
    while year <= end_year {
        match DataFileBar::load(iid, market_data, year) {
            Ok(data) => {
                df.extend(&data).unwrap();
                year += 1;
            }
            Err(e) => match e {
                DataError::NotFound(_) => {
                    year += 1;
                }
                DataError::ReadError(e) => {
                    log::error!("{}", e);
                    panic!();
                }
            },
        }
    }

    // filter & check empty
    let df = filter_dt(begin, end, df);
    if df.is_empty() {
        let msg = format!("{} {}", iid, market_data);
        return Err(DataError::NotFound(msg));
    }

    Ok(df)
}
fn request_tic(
    iid: &IID,
    market_data: &MarketData,
    begin: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> Result<DataFrame, DataError> {
    // create empty df
    let schema = tic_schema();
    let mut df = DataFrame::empty_with_schema(&schema);

    // load data by days
    let mut day = begin.date_naive();
    let end_day = end.date_naive();
    while day <= end_day {
        match DataFileTic::load(iid, market_data, &day) {
            Ok(data) => {
                df.extend(&data.df()).unwrap();
                day = day.checked_add_days(Days::new(1)).unwrap();
            }
            Err(e) => match e {
                DataError::NotFound(_) => {
                    day = day.checked_add_days(Days::new(1)).unwrap();
                }
                DataError::ReadError(e) => {
                    log::error!("{}", e);
                    panic!();
                }
            },
        }
    }

    // filter & check empty
    let df = filter_dt(begin, end, df);
    if df.is_empty() {
        let msg = format!("{} {}", iid, market_data);
        return Err(DataError::NotFound(msg));
    }

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // #[test]
    // fn request_1m() {
    //     let instr = IID::from("moex_share_sber").unwrap();
    //     let data = MarketData::BAR_1M;
    //     let begin = Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap();
    //     let end = Utc.with_ymd_and_hms(2023, 8, 1, 8, 0, 0).unwrap();
    //
    //     let df = Manager::request(&instr, &data, &begin, &end).unwrap();
    //     let bars = Bar::from_df(df).unwrap();
    //     let first = bars.first().unwrap();
    //     let last = bars.last().unwrap();
    //
    //     assert_eq!(first.dt(), begin);
    //     assert_eq!(
    //         last.dt(),
    //         Utc.with_ymd_and_hms(2023, 8, 1, 7, 59, 0).unwrap()
    //     );
    // }
    #[test]
    fn request_10m() {
        let instr = Manager::find("moex_share_sber").unwrap();
        let data = MarketData::BAR_10M;
        let begin = utils::datetime("2023-08-01 10:00:00");
        let end = utils::datetime("2023-08-01 11:00:00");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
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
        let instr = Manager::find("moex_share_sber").unwrap();
        let data = MarketData::BAR_1H;
        let begin = utils::datetime("2023-08-01 10:00:00");
        let end = utils::datetime("2023-08-01 13:00:00");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
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
        let instr = Manager::find("moex_share_sber").unwrap();
        let data = MarketData::BAR_D;
        let begin = utils::date("2023-08-01");
        let end = utils::date("2023-09-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
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
        let instr = Manager::find("moex_share_sber").unwrap();
        let data = MarketData::BAR_W;
        let begin = utils::date("2024-01-01");
        let end = utils::date("2025-01-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
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
        let instr = Manager::find("moex_share_sber").unwrap();
        let data = MarketData::BAR_M;
        let begin = utils::date("2024-01-01");
        let end = utils::date("2025-01-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
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
