use chrono::{DateTime, NaiveDate, NaiveTime, SecondsFormat, TimeDelta, Utc};
use polars::prelude::*;
use std::error::Error;
use std::io::Cursor;
use std::num::NonZeroUsize;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}
impl Command {
    pub fn build(args: &Vec<String>) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        };

        let name: String = args[1].clone();

        let mut copy_args: Vec<String> = Vec::new();
        for i in &args[2..] {
            copy_args.push(i.clone());
        }

        let command = Command {
            name,
            args: copy_args,
        };
        Ok(command)
    }
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        dbg!(&self);

        // Data::cache().await?;
        Data::download().await?;

        Ok(())
    }
}

pub struct Data {}
impl Data {
    pub async fn cache() -> Result<(), Box<dyn Error>> {
        println!(":: Caching instruments info");

        let source = DataMoex::new();
        source.cache().await?;

        Ok(())
    }
    pub async fn download() -> Result<(), Box<dyn Error>> {
        println!(":: Download ____");

        let source = DataMoex::new();
        source.download().await?;

        Ok(())
    }
}

const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
const MSK_TIME_DIF: TimeDelta = TimeDelta::hours(3);

struct DataMoex {
    pub name: String,
    url: String,
    api_key: String,
    user_name: String,
    password: String,
    client: reqwest::Client,
}
impl DataMoex {
    pub fn new() -> DataMoex {
        let name = "MOEX";
        let api_key = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJaVHA2Tjg1ekE4YTBFVDZ5SFBTajJ2V0ZldzNOc2xiSVR2bnVaYWlSNS1NIn0.eyJleHAiOjE3NDUzMDc1MzMsImlhdCI6MTc0MjcxNTUzMywiYXV0aF90aW1lIjoxNzQyNzE1MTExLCJqdGkiOiIxZWVmMmEyYi0wZTYzLTQyNjAtOWViNS1iODkwNDEzYTE2YjIiLCJpc3MiOiJodHRwczovL3NzbzIubW9leC5jb20vYXV0aC9yZWFsbXMvY3JhbWwiLCJhdWQiOlsiYWNjb3VudCIsImlzcyJdLCJzdWIiOiJmOjBiYTZhOGYwLWMzOGEtNDlkNi1iYTBlLTg1NmYxZmU0YmY3ZTo3OWViYzZhNi1iNmNlLTRjZWUtOGNhYi03OTI5NmI1MGYzZjIiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJpc3MiLCJzaWQiOiI4NWZkZWFiZi0zMjExLTRlMDgtYmFiNy0yZGRhZTY2MWE3ZTUiLCJhY3IiOiIxIiwiYWxsb3dlZC1vcmlnaW5zIjpbIi8qIl0sInJlYWxtX2FjY2VzcyI6eyJyb2xlcyI6WyJvZmZsaW5lX2FjY2VzcyIsInVtYV9hdXRob3JpemF0aW9uIl19LCJyZXNvdXJjZV9hY2Nlc3MiOnsiYWNjb3VudCI6eyJyb2xlcyI6WyJtYW5hZ2UtYWNjb3VudCIsInZpZXctcHJvZmlsZSJdfX0sInNjb3BlIjoib3BlbmlkIGlzc19hbGdvcGFjayBwcm9maWxlIG9mZmxpbmVfYWNjZXNzIGVtYWlsIGJhY2t3YXJkc19jb21wYXRpYmxlIiwiZW1haWxfdmVyaWZpZWQiOmZhbHNlLCJpc3NfcGVybWlzc2lvbnMiOiIxMzcsIDEzOCwgMTM5LCAxNDAsIDE2NSwgMTY2LCAxNjcsIDE2OCwgMzI5LCA0MjEiLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiI3OWViYzZhNi1iNmNlLTRjZWUtOGNhYi03OTI5NmI1MGYzZjIiLCJzZXNzaW9uX3N0YXRlIjoiODVmZGVhYmYtMzIxMS00ZTA4LWJhYjctMmRkYWU2NjFhN2U1In0.KSgQ4LnZA-QXwImADKm0xdQYqAxqxpk2YQ3V8ejGOPlV9Gs4JEAmqvWwhrkMylFJHnHf68Qgw11xEltyzF2kqZ9a5Zv5aVjtaE7qr6IdSVuWBp0X6AKIIS2uStKeqmT0BePesecPeY6DGBlnOYznpttnnCtkNGJ1Ax72qgZA8-Cz2LudilJVEQW0-OsBd-FZO4rr1sZ68Qa8JeUdJOHzErxhO7oPha0xHuL_2ypa-G9-KDUQArfc7okVcnetE0_sxuAq80wKEYagR_4Ca82-VQdYF_doE1KSELXudfZO9nKsS35898mraWK1jhUfUKVYTaStvS9eSyyHWY9_52qhnA";
        let url =
            "https://apim.moex.com/iss/engines/stock/markets/shares/boards/tqbr/securities.json";
        let user_name = "mr.alexavin@gmail.com";
        let password = "GRSww23.m";
        let client = reqwest::Client::new();

        DataMoex {
            name: name.to_string(),
            url: url.to_string(),
            api_key: api_key.to_string(),
            user_name: user_name.to_string(),
            password: password.to_string(),
            client: client,
        }
    }
    pub async fn cache(&self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();

        let request =
            client.get(&self.url).bearer_auth(&self.api_key).build()?;
        // println!("{request:#?}");

        let response = client.execute(request).await?;
        // println!("{response:#?}");

        // dataversion marketdata marketdata_yields securities
        let json: serde_json::Value = response.json().await.unwrap();
        // dbg!(&json);

        // let dataversion = &json["dataversion"];
        // let marketdata = &json["marketdata"];
        // let marketdata_yields = &json["marketdata_yields"];
        // let securities = &json["securities"];

        // let data = &json["securities"]["data"];

        // короче дело такое. Приходит json c разделами
        // dataversion marketdata marketdata_yields securities
        // внутри securities разделы data & columns
        // колонки разобрать еще можно до значений...
        // а вот с датой полная засада. Она хранится по строкам
        // в векторах. А чтобы собрать датафрейм мне нужны колонки...
        // а там: [["ABIO", "TQBR", … "2025-03-2…], ["AFKS", "TQBR", …]
        // так что впизду это разбирать... давай лучше посмотрим
        // как скачать данные.. с конца в конец тикеры которые мне
        // нужны я и так знаю.
        let columns = &json["securities"]["columns"];
        dbg!(&columns);
        let columns = columns.as_array().unwrap();
        dbg!(&columns);
        for i in columns {
            let s = i.as_str().unwrap();
            dbg!(&s);
        }

        // let json = serde_json::to_string(&json["securities"]).unwrap();
        // let file = Cursor::new(json);
        // let df = JsonReader::new(file)
        //     // .with_json_format(JsonFormat::JsonLines)
        //     .with_json_format(JsonFormat::Json)
        //     .infer_schema_len(NonZeroUsize::new(3))
        //     .with_batch_size(NonZeroUsize::new(3).unwrap())
        //     .finish()
        //     .unwrap();
        // println!("{:?}", df);

        // let json = serde_json::to_string(&data).unwrap();
        // let cursor = Cursor::new(json);
        // let df = JsonReader::new(cursor).finish().unwrap();
        // println!("{:?}", df);

        // use polars::prelude::*;
        // use polars::df;
        //
        // // use macro
        // let df = df! [
        //     "names" => ["a", "b", "c"],
        //     "values" => [1, 2, 3],
        //     "values_nulls" => [Some(1), None, Some(3)]
        // ]?;
        //
        // // from a Vec<Column>
        // let c1 = Column::new("names".into(), &["a", "b", "c"]);
        // let c2 = Column::new("values".into(), &[Some(1), None, Some(3)]);
        // let df = DataFrame::new(vec![c1, c2])?;

        return Ok(());
    }
    pub async fn download(&self) -> Result<(), Box<dyn Error>> {
        let shares =
            "https://apim.moex.com/iss/engines/stock/markets/shares/boards/tqbr/securities/";
        let ticker = "sber";
        let data_type = "candles.json?";
        let from = "from=2025-01-01&";
        let till = "till=2026-01-01&";
        let interval = "interval=24";

        let url =
            format!("{shares}/{ticker}/{data_type}{from}{till}{interval}");
        let request =
            self.client.get(&url).bearer_auth(&self.api_key).build()?;
        let response = self.client.execute(request).await?;

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
        let json: serde_json::Value = response.json().await.unwrap();
        // dbg!(&json);

        let candles_data = json["candles"]["data"].as_array().unwrap();
        // let mut date_time: Vec<DateTime<Utc>> = Vec::new();
        let mut date_time: Vec<String> = Vec::new();
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
            let dt = array[6].as_str().unwrap().to_string();
            // let dt = DataMoex::msk_to_utc(&dt);

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
        println!("{}", df);

        let df = df
            .lazy()
            .with_column(
                col("dt")
                    .str()
                    .to_datetime(
                        Some(TimeUnit::Milliseconds),
                        None,
                        StrptimeOptions::default(),
                        lit("raise"),
                    )
                    .dt()
                    .replace_time_zone(
                        Some("UTC".into()),
                        lit("raise"),
                        NonExistent::Raise,
                    )
                    .alias("dt"),
            )
            .collect()?;

        println!("{}", &df);

        return Ok(());
    }
    fn msk_to_utc(naive_moex_dt: &String) -> DateTime<Utc> {
        let moex_dt = format!("{naive_moex_dt} +03:00");
        let dt = DateTime::parse_from_str(&moex_dt, "%Y-%m-%d %H:%M:%S %z")
            .unwrap();

        // Для таймфреймов D, W, M - naive_moex_dt имеет время 00:00:00
        // после преобразования методом to_utc(), который меняет таймзону
        // на UTC и дропает offset. Получится не совсем то что нужно...
        // 2025-01-01 00:00:00+03:00  ->  2024-12-31 21:00:00+00:00
        // Поэтому снова к нему прибавляем MSK_TIME_DIF, чтобы получился
        // тот же день в Utc:  2025-01-01 00:00:00+00:00
        if dt.time() == DAY_BEGIN {
            return dt.to_utc() + MSK_TIME_DIF;
        }

        todo!("Преобразование времени младших ТФ из МСК в UTC");
        // Для младших таймфреймов нужно учесть offset еще
        // но вроде метод to_utc как раз его учитывает...
        return dt.to_utc();

        // dt.push_str(" +03:00");
        // let dt = DateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S %z").unwrap();
        // println!("3: {}", dt);
        // let dt = dt.to_utc();
        // println!("utc: {}", dt);
        // let dt = dt + TimeDelta::hours(3);
        // println!("utc+: {}", dt);

        // Для таймфреймов 1M, 10M, 1H у MOEX поле с датой открытия
        // бара имеет и дату и время. Время московское, приводим его в UTC+0
        // if moex_dt.hour != 0:
        //     return (moex_dt - cls.MSK_TIME_DIF).replace(tzinfo=UTC)
        // else:
        // Для таймфреймов 1D, W, M в файлах MOEX поле с датой открытия
        // бара имеет только дату.
        // datetime.fromisoformat возвращает дату со временем 00:00
        // тут остается только заменить timezone на UTC+0
        // return moex_dt.replace(tzinfo=UTC)
    }
}
