/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Cmd;
use crate::conf::{DT_FMT, MSK_TIME_DIF};
use crate::core::IID;
use crate::data::market_data::MarketData;
use chrono::prelude::*;
use polars::prelude::*;
use std::path::Path;

pub struct SourceMoex {
    // pub name: String,
    service: String,
    api_key: String,
    client: reqwest::Client,
    candle_schema: Schema,
}
impl SourceMoex {
    pub fn new() -> Self {
        // let name = "MOEX";
        let service = "https://apim.moex.com/iss";
        let key_path =
            Path::new("/home/alex/avin/usr/connect/moex/api_key.txt");
        let api_key = Cmd::read(key_path).unwrap().trim().to_string();
        let client = reqwest::Client::new();

        let candle_schema = Schema::from_iter(vec![
            Field::new("dt".into(), DataType::String),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("volume".into(), DataType::UInt64),
        ]);

        Self {
            // name: name.to_string(),
            service: service.to_string(),
            api_key,
            client,
            candle_schema,
        }
    }
    // pub async fn cache(&self) -> Result<(), &'static str> {
    //     let client = reqwest::Client::new();
    //
    //     let request = client
    //         .get(&self.service)
    //         .bearer_auth(&self.api_key)
    //         .build()?;
    //     // println!("{request:#?}");
    //
    //     let response = client.execute(request).await?;
    //     // println!("{response:#?}");
    //
    //     // dataversion marketdata marketdata_yields securities
    //     let json: serde_json::Value = response.json().await.unwrap();
    //
    //     // let dataversion = &json["dataversion"];
    //     // let marketdata = &json["marketdata"];
    //     // let marketdata_yields = &json["marketdata_yields"];
    //     // let securities = &json["securities"];
    //
    //     // let data = &json["securities"]["data"];
    //
    //     // короче дело такое. Приходит json c разделами
    //     // dataversion marketdata marketdata_yields securities
    //     // внутри securities разделы data & columns
    //     // колонки разобрать еще можно до значений...
    //     // а вот с датой полная засада. Она хранится по строкам
    //     // в векторах. А чтобы собрать датафрейм мне нужны колонки...
    //     // а там: [["ABIO", "TQBR", … "2025-03-2…], ["AFKS", "TQBR", …]
    //     // так что впизду это разбирать... давай лучше посмотрим
    //     // как скачать данные.. с конца в конец тикеры которые мне
    //     // нужны я и так знаю.
    //     let columns = &json["securities"]["columns"];
    //     let columns = columns.as_array().unwrap();
    //     for i in columns {
    //         let s = i.as_str().unwrap();
    //     }
    //
    //     // let json = serde_json::to_string(&json["securities"]).unwrap();
    //     // let file = Cursor::new(json);
    //     // let df = JsonReader::new(file)
    //     //     // .with_json_format(JsonFormat::JsonLines)
    //     //     .with_json_format(JsonFormat::Json)
    //     //     .infer_schema_len(NonZeroUsize::new(3))
    //     //     .with_batch_size(NonZeroUsize::new(3).unwrap())
    //     //     .finish()
    //     //     .unwrap();
    //     // println!("{:?}", df);
    //
    //     // let json = serde_json::to_string(&data).unwrap();
    //     // let cursor = Cursor::new(json);
    //     // let df = JsonReader::new(cursor).finish().unwrap();
    //     // println!("{:?}", df);
    //
    //     // use polars::prelude::*;
    //     // use polars::df;
    //     //
    //     // // use macro
    //     // let df = df! [
    //     //     "names" => ["a", "b", "c"],
    //     //     "values" => [1, 2, 3],
    //     //     "values_nulls" => [Some(1), None, Some(3)]
    //     // ]?;
    //     //
    //     // // from a Vec<Column>
    //     // let c1 = Column::new("names".into(), &["a", "b", "c"]);
    //     // let c2 = Column::new("values".into(), &[Some(1), None, Some(3)]);
    //     // let df = DataFrame::new(vec![c1, c2])?;
    //
    //     return Ok(());
    // }
    pub async fn get_bars(
        &self,
        iid: &IID,
        market_data: &MarketData,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<DataFrame, &'static str> {
        let mut from = Self::utc_to_msk(begin);
        let till = Self::utc_to_msk(end);

        let mut candles = DataFrame::empty_with_schema(&self.candle_schema);
        while from < till {
            println!("   from {from}");
            let response = self
                .try_request(iid, market_data, &from, &till)
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
            candles.extend(&part).unwrap();

            let last = Self::get_last_dt(&part);
            if last < till {
                from = last;
            } else {
                break;
            }
        }

        candles = Self::drop_duplicate(candles);
        candles = Self::dt_to_timestamp(candles);

        return Ok(candles);
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
        iid: &IID,
        market_data: &MarketData,
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
        iid: &IID,
        market_data: &MarketData,
        begin: &NaiveDateTime,
        end: &NaiveDateTime,
    ) -> Result<String, &'static str> {
        let mut url = self.service.clone();

        assert_eq!(iid.category, "SHARE");
        if iid.category == "SHARE" {
            url.push_str(
                "/engines/stock/markets/shares/boards/tqbr/securities/",
            );
        } else {
            panic!("unsupported category");
        }

        let ticker = &iid.ticker;
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

            MarketData::BAR_5M => Err("5M data is not availible at MOEX"),
            // _ => todo!(),
        }
    }
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
        let candles_data = json["candles"]["data"].as_array().unwrap();
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
        let last = candles.column("dt").unwrap().len() - 1;
        let last =
            candles.column("dt").unwrap().get(last).unwrap().str_value();
        let last = NaiveDateTime::parse_from_str(&last, DT_FMT).unwrap();

        return last;
    }
    fn drop_duplicate(candles: DataFrame) -> DataFrame {
        // INFO: во время загузки с мос.биржи в запросе идет
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
    // fn set_tz_utc(candles: DataFrame) -> DataFrame {
    //     candles
    //         .lazy()
    //         .with_column(col("dt").dt().replace_time_zone(
    //             Some("UTC".into()),
    //             lit("raise"),
    //             NonExistent::Raise,
    //         ))
    //         .collect()
    //         .unwrap()
    // }
}
