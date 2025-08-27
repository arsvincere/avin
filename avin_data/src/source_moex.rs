/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

use avin_utils::AvinError;
use avin_utils::CFG;
use avin_utils::Cmd;
use chrono::prelude::*;
use chrono::Utc;
use chrono::TimeDelta;
use polars::prelude::*;
use avin_core::{Iid, Manager};
use serde_json;


use crate::MarketData;

const MSK_TIME_DIF: TimeDelta = TimeDelta::new(10800, 0).unwrap();
const DT_FMT: &'static str = "%Y-%m-%d %H:%M:%S";


pub struct SourceMoex {
    service: String,
    api_key: String,
    client: reqwest::Client,
    bar_schema: Schema,
}
impl SourceMoex {
    pub fn new() -> Self {
        let service = "https://apim.moex.com/iss".to_string(); // todo: должно лежать в константах и выбор так же через енамы (?)
        let key_path = CFG.connect.moex_api_key();
        let api_key = Cmd::read(&key_path).unwrap().trim().to_string();
        let client = reqwest::Client::new();
        let bar_schema = Schema::from_iter(vec![Field::new("dt".into(), DataType::String),
             Field::new("open".into(), DataType::Float64),
             Field::new("high".into(), DataType::Float64),
             Field::new("low".into(), DataType::Float64),
             Field::new("close".into(), DataType::Float64),
             Field::new("volume".into(), DataType::UInt64),]);
        Self {
            service,
            api_key,
            client,
            bar_schema,
        }
    }

    #[warn(unused_variables)]
    pub async fn get_bars(&self,
        begin: &DateTime<Utc>, end: &DateTime<Utc>) -> Result<DataFrame, AvinError> {
            let mut from = Self::utc_to_msk(begin);
            let till = Self::utc_to_msk(end);
            let iid = Manager::find_iid("MOEX_SHARE_GAZP").unwrap();    // убрать
            let mut bars = DataFrame::empty_with_schema(&self.bar_schema);

            while from < till {
                let response = self.try_request(&iid, MarketData::BAR_1H, &from, &till)
                    .await
                    .unwrap();
                let json: serde_json::Value = match response.json().await {
                Err(e) => {
                    eprintln!("Error parsing response: {e}");
                    eprintln!("Try request again");
                    continue;
                }
                Ok(json) => json,
            };
            let part = Self::parse_json_candles(json);

            if part.height() <= 1 {
                break;
            }
            bars.extend(&part).unwrap();

            let last = Self::get_last_dt(&part);
            if last < till {
                from = last;
            } else {
                break;
            }
        }

        bars = Self::drop_duplicate(bars);
        bars = Self::dt_to_timestamp(bars);

        return Ok(bars);
    }

    fn utc_to_msk(dt: &DateTime<Utc>) -> NaiveDateTime {
        dt.naive_utc() + MSK_TIME_DIF
    }

    fn msk_to_utc(moex_dt: &str) -> DateTime<Utc> {
        let dt = format!("{}+03:00", moex_dt);
        let dt = DateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S%z");

        dt.unwrap().to_utc()
    }

    async fn try_request(
        &self,
        iid: &Iid,
        market_data: MarketData,
        from: &NaiveDateTime,
        till: &NaiveDateTime,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.get_url(iid, market_data, from, till).unwrap();
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        return Ok(response);
    }

    fn get_url(
        &self,
        iid: &Iid,
        market_data: MarketData,
        begin: &NaiveDateTime,
        end: &NaiveDateTime,
    ) -> Result<String, &'static str> {
        let mut url = self.service.clone();

        assert_eq!(iid.category(), "SHARE");
        url.push_str(
            "/engines/stock/markets/shares/boards/tqbr/securities/",
        );

        let ticker = &iid.ticker();
        let data = "/candles.json?";
        let from = format!("from={begin}&"); // "from=2025-01-01 00:00&"
        let till = format!("till={end}&"); // "till=2025-03-27 14:35&"
        let interval = Self::interval_from(&market_data)?;

        url = format!("{url}{ticker}{data}{from}{till}{interval}");
        Ok(url)
    }

    fn interval_from(market_data: &MarketData) -> Result<&str, &'static str> {
        match market_data {
            MarketData::BAR_1M => Ok("interval=1"),
            MarketData::BAR_10M => Ok("interval=10"),
            MarketData::BAR_1H => Ok("interval=60"),
            MarketData::BAR_D => Ok("interval=24"),
            MarketData::BAR_W => Ok("interval=7"),
            MarketData::BAR_M => Ok("interval=31"),

            MarketData::TIC => todo!(),
            MarketData::BAR_5M => Err("5M data is not availible at MOEX"),
        }
    }

    #[warn(unused_variables)]
    fn parse_json_candles(json: serde_json::Value) -> DataFrame {
        // "candles": Object {
        //     "columns": Array [
        //         String("open"),
        //         String("close"),
        //         String("high"),
        //         String("low"),
        //         String("value"),
        //         String("volume"),
        //         String("begin"),
        //         String("end"),
        //     ],
        //     "data": Array [
        //         Array [
        //             Number(280),
        //             Number(272.25),
        //             Number(280.41),
        //             Number(271.8),
        //             Number(11853565984.9),
        //             Number(43086870),
        //             String("2025-01-03 00:00:00"),
        //             String("2025-01-03 23:59:59"),
        //         ],
        //         Array [
        //             Number(270.88),
        //             Number(274.37),
        //             Number(274.41),
        //             Number(270.07),
        //             Number(7737094495.2),
        //             Number(28454750),
        //             String("2025-01-06 00:00:00"),
        //             String("2025-01-06 23:59:59"),
        //         ],
        let candles_data: &Vec<serde_json::Value> = json["candles"]["data"].as_array().unwrap();
        let mut date_time: Vec<&str> = Vec::new();
        let mut open: Vec<f64> = Vec::new();
        let mut close: Vec<f64> = Vec::new();
        let mut high: Vec<f64> = Vec::new();
        let mut low: Vec<f64> = Vec::new();
        let mut vol: Vec<u64> = Vec::new();
        // let val: Vec<f64> = Vec::new();
        for candle in candles_data {
            let array = candle.as_array().unwrap();

            let o = array[0].as_f64().unwrap();
            let c = array[1].as_f64().unwrap();
            let h = array[2].as_f64().unwrap();
            let l = array[3].as_f64().unwrap();
            // let val = array[4].as_f64().unwrap();
            let v = array[5].as_u64().unwrap();
            let dt = array[6].as_str().unwrap();

            date_time.push(dt);
            open.push(o);
            high.push(h);
            low.push(l);
            close.push(c);
            vol.push(v);
        }

        let df: DataFrame = df!(
            "dt" => date_time,
            "open" => open,
            "high" => high,
            "low" => low,
            "close" => close,
            "volume" => vol,
        )
        .unwrap();

        return df;
    }

    fn get_last_dt(candles: &DataFrame) -> NaiveDateTime {
        let last: usize = candles.column("dt").unwrap().len() - 1;
        let last =
            candles.column("dt").unwrap().get(last).unwrap().str_value();
        let last = NaiveDateTime::parse_from_str(&last, DT_FMT).unwrap();

        return last;
    }

    fn drop_duplicate(candles: DataFrame) -> DataFrame {
        // NOTE: во время загузки с мос.биржи в запросе идет
        // from-till и на каждой итерации цикла получается дублируется
        // последняя свеча: сначала она идет последняя, а на следующем
        // шаге цикла она первая. Все потому что долбаная мосбиржа
        // выдает свечи в закрытом диапазоне [from, till]. Было бы
        // меньше боли если бы выдавала как обычно в программировании
        // полуоткрытый диапазон [from, till).
        // Ну самый простой вариант - переложить работу по удаленю
        // дублей на DataFrame.
        let col_name = String::from("dt");

        candles
            .unique_stable(Some(&[col_name]), UniqueKeepStrategy::Any, None)
            .unwrap()
    }

        fn dt_to_timestamp(mut candles: DataFrame) -> DataFrame {
        let mut timestamp: Vec<i64> = Vec::new();
        for naive_opt in candles.column("dt").unwrap().str().unwrap().iter() {
            let utc_dt = Self::msk_to_utc(naive_opt.unwrap());
            let ts = utc_dt.timestamp_nanos_opt().unwrap();
            timestamp.push(ts);
        }

        candles
            .insert_column(0, Column::new("ts_nanos".into(), &timestamp))
            .unwrap()
            .drop_in_place("dt")
            .unwrap();

        candles
    }


}
//     fn get_last_dt(candles: &DataFrame) -> NaiveDateTime {
//         let last = candles.column("dt").unwrap().len() - 1;
//         let last =
//             candles.column("dt").unwrap().get(last).unwrap().str_value();
//         let last = NaiveDateTime::parse_from_str(&last, DT_FMT).unwrap();
//
//         return last;
//     }
//     fn drop_duplicate(candles: DataFrame) -> DataFrame {
//         // NOTE: во время загузки с мос.биржи в запросе идет
//         // from-till и на каждой итерации цикла получается дублируется
//         // последняя свеча: сначала она идет последняя, а на следующем
//         // шаге цикла она первая. Все потому что долбаная мосбиржа
//         // выдает свечи в закрытом диапазоне [from, till]. Было бы
//         // меньше боли если бы выдавала как обычно в программировании
//         // полуоткрытый диапазон [from, till).
//         // Ну самый простой вариант - переложить работу по удаленю
//         // дублей на DataFrame.
//         let col_name = String::from("dt");
//
//         candles
//             .unique_stable(Some(&[col_name]), UniqueKeepStrategy::Any, None)
//             .unwrap()
//     }
//     fn dt_to_timestamp(mut candles: DataFrame) -> DataFrame {
//         let mut timestamp: Vec<i64> = Vec::new();
//         for naive_opt in candles.column("dt").unwrap().str().unwrap().iter() {
//             let utc_dt = Self::msk_to_utc(naive_opt.unwrap());
//             let ts = utc_dt.timestamp_nanos_opt().unwrap();
//             timestamp.push(ts);
//         }
//
//         candles
//             .insert_column(0, Column::new("ts_nanos".into(), &timestamp))
//             .unwrap()
//             .drop_in_place("dt")
//             .unwrap();
//
//         candles
//     }
//     // fn set_tz_utc(candles: DataFrame) -> DataFrame {
//     //     candles
//     //         .lazy()
//     //         .with_column(col("dt").dt().replace_time_zone(
//     //             Some("UTC".into()),
//     //             lit("raise"),
//     //             NonExistent::Raise,
//     //         ))
//     //         .collect()
//     //         .unwrap()
//     // }
// }
