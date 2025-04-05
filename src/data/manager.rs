/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::Asset;
use crate::data::data_file_bar::DataFileBar;
use crate::data::market_data::MarketData;
use crate::data::source::Source;
use crate::data::source_moex::SourceMoex;
use chrono::prelude::*;
use polars::prelude::*;

pub struct Manager {}
impl Manager {
    pub async fn download(
        source: &Source,
        asset: &Asset,
        market_data: &MarketData,
        year: Option<i32>,
    ) -> Result<(), &'static str> {
        let source = match source {
            Source::MOEX => SourceMoex::new(),
            Source::TINKOFF => panic!("Нахер с Тинькофф качать?"),
            Source::CONVERTER => panic!(),
        };
        println!(":: Download {} {}", asset.ticker, market_data.name());

        match year {
            Some(year) => {
                Self::download_one_year(&source, &asset, &market_data, year)
                    .await
            }
            None => {
                Self::download_all_availible(&source, &asset, &market_data)
                    .await
            }
        }
    }
    pub fn convert(
        asset: &Asset,
        in_t: &MarketData,
        out_t: &MarketData,
    ) -> Result<(), &'static str> {
        println!(
            ":: Convert {} {} -> {}",
            asset.ticker,
            in_t.name(),
            out_t.name(),
        );

        // load data files
        let data = DataFileBar::request_all(asset, in_t)?;
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
        asset: &Asset,
        market_data: &MarketData,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<DataFrame, &'static str> {
        let mut year = begin.year();
        let end_year = end.year();

        let mut df = DataFileBar::load(asset, market_data, year).unwrap();
        year = year + 1;
        while year <= end_year {
            let file_df =
                DataFileBar::load(asset, market_data, year).unwrap();
            df.extend(&file_df).unwrap();
            year += 1;
        }

        let begin = begin.timestamp_nanos_opt().unwrap();
        let end = end.timestamp_nanos_opt().unwrap();
        let df = df
            .clone()
            .lazy()
            .filter(col("ts_nanos").is_between(
                begin,
                end,
                ClosedInterval::Left,
            ))
            .collect()
            .unwrap();

        Ok(df)
    }

    async fn download_one_year(
        source: &SourceMoex,
        asset: &Asset,
        market_data: &MarketData,
        year: i32,
    ) -> Result<(), &'static str> {
        let begin = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();
        let df = source.get_bars(&asset, &market_data, &begin, &end).await?;

        if df.is_empty() {
            return Err("   - no data for {year}");
        }

        // INFO: ParquetWriter требует &mut df для сохранения...
        // по факту никто data_file не меняет перед записью
        let mut data_file =
            DataFileBar::new(asset, market_data.clone(), df, year).unwrap();
        DataFileBar::save(&mut data_file)?;

        println!("Download complete!");
        Ok(())
    }
    async fn download_all_availible(
        source: &SourceMoex,
        asset: &Asset,
        market_data: &MarketData,
    ) -> Result<(), &'static str> {
        let mut year: i32 = 1990; // суть - более старых данных точно нет
        let now_year = Utc::now().year();

        while year <= now_year {
            let begin = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59).unwrap();
            let df =
                source.get_bars(&asset, &market_data, &begin, &end).await?;

            if df.is_empty() {
                println!("   - no data for {year}");
                year += 1;
                continue;
            }

            // INFO: ParquetWriter требует &mut df для сохранения...
            // по факту никто data_file не меняет перед записью
            let mut data_file =
                DataFileBar::new(asset, market_data.clone(), df, year)
                    .unwrap();
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
    //     let instr = Asset::from("moex_share_sber").unwrap();
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
        let instr = Asset::from("moex_share_sber").unwrap();
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
        let instr = Asset::from("moex_share_sber").unwrap();
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
        let instr = Asset::from("moex_share_sber").unwrap();
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
        let instr = Asset::from("moex_share_sber").unwrap();
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
        let instr = Asset::from("moex_share_sber").unwrap();
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
