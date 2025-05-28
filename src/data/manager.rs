/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::data::category::Category;
use crate::data::data_file_bar::DataFileBar;
use crate::data::error::DataError;
use crate::data::iid::IID;
use crate::data::iid_cache::IidCache;
use crate::data::market_data::MarketData;
use crate::data::source::Source;
use crate::data::source_moex::SourceMoex;
use crate::tinkoff::Tinkoff;
use chrono::prelude::*;
use polars::prelude::{
    DataFrame, DataType, Duration, Field, IntoLazy, NamedFrom, Schema,
    Series, col, df,
};
use polars::time::ClosedWindow;
// use polars::prelude::*;

pub struct Manager {}
impl Manager {
    pub async fn cache(source: &Source) -> Result<(), &'static str> {
        println!(":: Caching {}", source.to_string());

        match source {
            Source::TINKOFF => Manager::cache_tinkoff().await,
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
                Self::download_one_year(&source, &iid, &market_data, year)
                    .await
            }
            None => {
                Self::download_all_availible(&source, &iid, &market_data)
                    .await
            }
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
            Manager::convert_timeframe(&i, in_t, out_t)?;
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
        // create empty df
        let bar_schema = Schema::from_iter(vec![
            Field::new("ts_nanos".into(), DataType::Int64),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("volume".into(), DataType::UInt64),
        ]);
        let mut df = DataFrame::empty_with_schema(&bar_schema);

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

        // filter begin end datetime
        let begin = begin.timestamp_nanos_opt().unwrap_or(0);
        let end = end.timestamp_nanos_opt().unwrap();
        let df = df
            .lazy()
            .filter(col("ts_nanos").gt_eq(begin))
            .filter(col("ts_nanos").lt(end))
            .collect()
            .unwrap();

        // check empty
        if df.is_empty() {
            let msg = format!("{} {}", iid, market_data);
            return Err(DataError::NotFound(msg));
        }

        Ok(df)
    }

    async fn cache_tinkoff() -> Result<(), &'static str> {
        let mut source = Tinkoff::new().await;
        let shares = source.get_shares().await.unwrap();

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
    async fn download_one_year(
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
            let df =
                source.get_bars(&iid, &market_data, &begin, &end).await?;

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
        data: &DataFileBar,
        in_t: &MarketData,
        out_t: &MarketData,
    ) -> Result<(), &'static str> {
        let b = NaiveDate::from_ymd_opt(data.year, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let e = NaiveDate::from_ymd_opt(data.year, 12, 31)
            .unwrap()
            .and_hms_opt(23, 59, 0)
            .unwrap();
        let r = polars::prelude::date_range(
            "dt".into(),
            b,
            e,
            Duration::new(
                time_unit::TimeUnit::Minutes.get_unit_nanoseconds() as i64,
            ),
            ClosedWindow::Both, // Both=[b,e], None=(b,e)...
            polars::prelude::TimeUnit::Milliseconds,
            None,
        )
        .unwrap();
        let c = Series::new("name".into(), r);
        let df = df!(
            "dt" => c,
        );
        // TODO:
        // пока получилось только создать датафрейм типо "__fillVoid" как
        // раньше делал. Без пробелов по датам. Теперь его еще timezone
        // Utc поставить. Потом надо объединить с реальными барама.
        // Потом как то селектить по группам и сливать в один бар.
        // Сейчас это слишком сложно для меня... нифига еще не понимаю
        // как работать с датафреймами на расте, на питоне блин все было
        // просто.
        dbg!(&in_t);
        dbg!(&out_t);
        dbg!(&df);

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Usr;
    use crate::core::Bar;

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
        let begin = Usr::dt("2023-08-01 10:00:00");
        let end = Usr::dt("2023-08-01 11:00:00");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
        let bars = Bar::from_df(df).unwrap();
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
        let begin = Usr::dt("2023-08-01 10:00:00");
        let end = Usr::dt("2023-08-01 13:00:00");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
        dbg!(&df);
        let bars = Bar::from_df(df).unwrap();
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
        let begin = Usr::date("2023-08-01");
        let end = Usr::date("2023-09-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
        let bars = Bar::from_df(df).unwrap();
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
        let begin = Usr::date("2024-01-01");
        let end = Usr::date("2025-01-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
        let bars = Bar::from_df(df).unwrap();
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
        let begin = Usr::date("2024-01-01");
        let end = Usr::date("2025-01-01");

        let df = Manager::request(&instr, &data, &begin, &end).unwrap();
        let bars = Bar::from_df(df).unwrap();
        let first = bars.first().unwrap();
        let last = bars.last().unwrap();

        assert_eq!(first.dt(), begin);
        assert_eq!(
            last.dt(),
            Utc.with_ymd_and_hms(2024, 11, 30, 21, 0, 0).unwrap()
        );
    }
}
