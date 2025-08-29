/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::Days;
use chrono::TimeDelta;
use chrono::Utc;
use chrono::prelude::*;
use polars::prelude::*;

use avin_utils::AvinError;
use avin_utils::CFG;
use avin_utils::Cmd;

use crate::Iid;
use crate::Manager;

use super::MarketData;
use super::schema;

const SERVICE: &str = "https://apim.moex.com/iss";
// const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";
const MSK_TIME_DIF: TimeDelta = TimeDelta::new(10800, 0).unwrap();

pub struct SourceMoex {
    token: String,
    client: reqwest::Client,
}
impl Default for SourceMoex {
    fn default() -> Self {
        let token_path = CFG.connect.moex_token();
        let token = Cmd::read(&token_path).unwrap().trim().to_string();

        Self {
            token,
            client: reqwest::Client::new(),
        }
    }
}
impl SourceMoex {
    // build
    pub fn new() -> Self {
        SourceMoex::default()
    }

    // public
    pub async fn get(
        &self,
        iid: &Iid,
        md: MarketData,
        from: DateTime<Utc>, // closed range [from, till]
        till: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        assert!(from < till);
        let from = utc_to_msk(from);
        let till = utc_to_msk(till);

        match md {
            MarketData::BAR_1M => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_10M => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_1H => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_D => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_W => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_M => self.get_bars(iid, md, from, till).await,
            MarketData::TIC => todo!(),
            MarketData::TRADE_STATS => self.get_trades(iid, from, till).await,
            MarketData::ORDER_STATS => todo!(),
            MarketData::OB_STATS => todo!(),
        }
    }

    // get bars
    pub async fn get_bars(
        &self,
        _iid: &Iid,
        _md: MarketData,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        let iid = Manager::find_iid("MOEX_SHARE_GAZP").unwrap(); // убрать
        let mut bars = DataFrame::empty_with_schema(&schema::bar_schema());

        let mut dt = from;
        while dt < till {
            let response = self
                .try_request_bars(&iid, MarketData::BAR_1M, &dt, &till)
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
            let part = parse_json_bars(json);

            if part.height() <= 1 {
                break;
            }
            bars.extend(&part).unwrap();

            let last = get_last_dt(&part);
            if last < till {
                dt = last;
            } else {
                break;
            }
        }

        bars = dt_to_timestamp(bars);
        bars = drop_duplicate_timestamp(bars);

        Ok(bars)
    }
    async fn try_request_bars(
        &self,
        iid: &Iid,
        market_data: MarketData,
        from: &NaiveDateTime,
        till: &NaiveDateTime,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.get_url_bar(iid, market_data, from, till).unwrap();
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        Ok(response)
    }
    fn get_url_bar(
        &self,
        iid: &Iid,
        market_data: MarketData,
        begin: &NaiveDateTime,
        end: &NaiveDateTime,
    ) -> Result<String, &'static str> {
        let mut url = String::from(SERVICE);

        assert_eq!(iid.category(), "SHARE");
        url.push_str("/engines/stock/markets/shares/boards/tqbr/securities/");

        let ticker = &iid.ticker();
        let data = "/candles.json?";
        let from = format!("from={begin}&"); // "from=2025-01-01 00:00&"
        let till = format!("till={end}&"); // "till=2025-03-27 14:35&"
        let interval = interval_from(&market_data);

        url = format!("{url}{ticker}{data}{from}{till}{interval}");

        Ok(url)
    }

    // get trades
    async fn get_trades(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        let mut trades =
            DataFrame::empty_with_schema(&schema::trades_schema());

        let mut f = from;
        while f < till {
            // NOTE: данные отдает не более 1000шт за раз, поэтому
            // качаем по 3 дня. Время игнорится, from-till учитывается
            // только дата дня, время отбрасывается.
            let t = f.checked_add_days(Days::new(3)).unwrap();
            let response = self.try_request_trades(iid, f, t).await.unwrap();

            let json: serde_json::Value = match response.json().await {
                Err(e) => {
                    log::error!("Error parsing response to json: {e}");
                    panic!();
                }
                Ok(json) => json,
            };

            let part = parse_json_trades_stat(json);
            trades.extend(&part).unwrap();

            if f < till {
                f = t;
            } else {
                break;
            }
        }

        // trades = dt_to_timestamp(trades);
        trades = drop_duplicate_timestamp(trades);

        Ok(trades)
    }
    async fn try_request_trades(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<reqwest::Response, AvinError> {
        let url = self.get_url_trades_stat(iid, from, till);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        Ok(response)
    }
    fn get_url_trades_stat(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> String {
        let from = from.date();
        let till = till.date();

        let mut url = String::from(SERVICE);
        url += "/datashop/algopack/eq/tradestats";
        url += format!("/{}.json?", iid.ticker()).as_str();
        url += format!("from={from}&").as_str();
        url += format!("till={till}").as_str();

        url
    }
}

fn utc_to_msk(dt: DateTime<Utc>) -> NaiveDateTime {
    dt.naive_utc() + MSK_TIME_DIF
}
fn msk_to_utc(moex_dt: &str) -> DateTime<Utc> {
    let dt = format!("{}+03:00", moex_dt);
    let dt = DateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S%z");

    dt.unwrap().to_utc()
}
fn interval_from(market_data: &MarketData) -> &str {
    match market_data {
        MarketData::BAR_1M => "interval=1",
        MarketData::BAR_10M => "interval=10",
        MarketData::BAR_1H => "interval=60",
        MarketData::BAR_D => "interval=24",
        MarketData::BAR_W => "interval=7",
        MarketData::BAR_M => "interval=31",
        _ => panic!(),
    }
}
fn dt_to_timestamp(mut candles: DataFrame) -> DataFrame {
    let mut timestamp: Vec<i64> = Vec::new();
    for naive_opt in candles.column("dt").unwrap().str().unwrap().iter() {
        let utc_dt = msk_to_utc(naive_opt.unwrap());
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
fn drop_duplicate_timestamp(df: DataFrame) -> DataFrame {
    // NOTE: во время загузки с мос.биржи в запросе идет
    // from-till и на каждой итерации цикла получается дублируется
    // последняя свеча: сначала она идет последняя, а на следующем
    // шаге цикла она первая. Все потому что долбаная мосбиржа
    // выдает свечи в закрытом диапазоне [from, till]. Было бы
    // меньше боли если бы выдавала как обычно в программировании
    // полуоткрытый диапазон [from, till).
    // Ну самый простой вариант - переложить работу по удаленю
    // дублей на DataFrame.
    let col_name = String::from("ts_nanos");

    df.unique_stable(Some(&[col_name]), UniqueKeepStrategy::Any, None)
        .unwrap()
}
fn get_last_dt(df: &DataFrame) -> NaiveDateTime {
    let ts = df
        .column("ts_nanos")
        .unwrap()
        .i64()
        .unwrap()
        .last()
        .unwrap();

    let utc_dt = avin_utils::dt(ts).naive_utc();

    utc_dt + MSK_TIME_DIF
}
fn parse_json_bars(json: serde_json::Value) -> DataFrame {
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
    let candles_data: &Vec<serde_json::Value> =
        json["candles"]["data"].as_array().unwrap();
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

    df
}
fn parse_json_trades_stat(json: serde_json::Value) -> DataFrame {
    // json["data"]["columns"] = Array [
    //     String("tradedate"),
    //     String("tradetime"),
    //     String("secid"),
    //     String("pr_open"),
    //     String("pr_high"),
    //     String("pr_low"),
    //     String("pr_close"),
    //     String("pr_std"),
    //     String("vol"),
    //     String("val"),
    //     String("trades"),
    //     String("pr_vwap"),
    //     String("pr_change"),
    //     String("trades_b"),
    //     String("trades_s"),
    //     String("val_b"),
    //     String("val_s"),
    //     String("vol_b"),
    //     String("vol_s"),
    //     String("disb"),
    //     String("pr_vwap_b"),
    //     String("pr_vwap_s"),
    //     String("SYSTIME"),
    //     String("sec_pr_open"),
    //     String("sec_pr_high"),
    //     String("sec_pr_low"),
    //     String("sec_pr_close"),
    // ]

    // json["data"]["metadata"] = Array [
    // "metadata": Object {
    //         "SYSTIME": Object {
    //             "bytes": Number(19),
    //             "max_size": Number(0),
    //             "type": String("datetime"),
    //         },
    //         "disb": Object {
    //             "type": String("double"),
    //         },
    //         "pr_change": Object {
    //             "type": String("double"),
    //         },
    //         "pr_close": Object {
    //             "type": String("double"),
    //         },
    //         "pr_high": Object {
    //             "type": String("double"),
    //         },
    //         "pr_low": Object {
    //             "type": String("double"),
    //         },
    //         "pr_open": Object {
    //             "type": String("double"),
    //         },
    //         "pr_std": Object {
    //             "type": String("double"),
    //         },
    //         "pr_vwap": Object {
    //             "type": String("double"),
    //         },
    //         "pr_vwap_b": Object {
    //             "type": String("double"),
    //         },
    //         "pr_vwap_s": Object {
    //             "type": String("double"),
    //         },
    //         "sec_pr_close": Object {
    //             "type": String("int32"),
    //         },
    //         "sec_pr_high": Object {
    //             "type": String("int32"),
    //         },
    //         "sec_pr_low": Object {
    //             "type": String("int32"),
    //         },
    //         "sec_pr_open": Object {
    //             "type": String("int32"),
    //         },
    //         "secid": Object {
    //             "bytes": Number(36),
    //             "max_size": Number(0),
    //             "type": String("string"),
    //         },
    //         "tradedate": Object {
    //             "bytes": Number(10),
    //             "max_size": Number(0),
    //             "type": String("date"),
    //         },
    //         "trades": Object {
    //             "type": String("int32"),
    //         },
    //         "trades_b": Object {
    //             "type": String("int32"),
    //         },
    //         "trades_s": Object {
    //             "type": String("int32"),
    //         },
    //         "tradetime": Object {
    //             "bytes": Number(10),
    //             "max_size": Number(0),
    //             "type": String("time"),
    //         },
    //         "val": Object {
    //             "type": String("double"),
    //         },
    //         "val_b": Object {
    //             "type": String("double"),
    //         },
    //         "val_s": Object {
    //             "type": String("double"),
    //         },
    //         "vol": Object {
    //             "type": String("int32"),
    //         },
    //         "vol_b": Object {
    //             "type": String("int64"),
    //         },
    //         "vol_s": Object {
    //             "type": String("int64"),
    //         },
    //     },

    // json["data"]["data"] = Array [
    //     String("2024-01-11"),
    //     String("12:50:00"),
    //     String("SBER"),
    //     Number(275.08),
    //     Number(275.15),
    //     Number(275.05),
    //     Number(275.13),
    //     Number(0),
    //     Number(11491),
    //     Number(31615539),
    //     Number(194),
    //     Number(275.13),
    //     Number(0.0182),
    //     Number(124),
    //     Number(70),
    //     Number(27739332),
    //     Number(3876207),
    //     Number(10082),
    //     Number(1409),
    //     Number(0.75),
    //     Number(275.14),
    //     Number(275.1),
    //     String("2024-08-13 19:10:26"),
    //     Number(1),
    //     Number(234),
    //     Number(28),
    //     Number(297),
    // ],

    // tmp Vec for create DataFrame
    let mut date: Vec<&str> = Vec::new(); // 0
    let mut time: Vec<&str> = Vec::new(); // 1
    let mut open: Vec<f64> = Vec::new(); // 3
    let mut high: Vec<f64> = Vec::new(); // 4
    let mut low: Vec<f64> = Vec::new(); // 5
    let mut close: Vec<f64> = Vec::new(); // 6
    let mut std: Vec<f64> = Vec::new(); // 7
    let mut vol: Vec<u64> = Vec::new(); // 8
    let mut val: Vec<f64> = Vec::new(); // 9
    let mut trades: Vec<u64> = Vec::new(); // 10
    let mut vwap: Vec<f64> = Vec::new(); // 11
    let mut change: Vec<f64> = Vec::new(); // 12
    let mut trades_b: Vec<u64> = Vec::new(); // 13
    let mut trades_s: Vec<u64> = Vec::new(); // 14
    let mut val_b: Vec<f64> = Vec::new(); // 15
    let mut val_s: Vec<f64> = Vec::new(); // 16
    let mut vol_b: Vec<u64> = Vec::new(); // 17
    let mut vol_s: Vec<u64> = Vec::new(); // 18
    let mut disb: Vec<f64> = Vec::new(); // 19
    let mut vwap_b: Vec<f64> = Vec::new(); // 20
    let mut vwap_s: Vec<f64> = Vec::new(); // 21

    // exaple row:
    // ["2024-01-11","12:45:00","SBER",275.15,275.15,275,275.08,0.0001,10246,28182035,167,275.05,-0.0254,83,84,5298652,22883383,1926,8320,-0.62,275.11,275.04,"2024-08-13 19:10:26",1,1,8,298]
    let data = json["data"]["data"].as_array().unwrap();
    for i in data {
        date.push(i[0].as_str().unwrap()); // 0
        time.push(i[1].as_str().unwrap()); // 1
        open.push(i[3].as_f64().unwrap()); // 3
        high.push(i[4].as_f64().unwrap()); // 4
        low.push(i[5].as_f64().unwrap()); // 5
        close.push(i[6].as_f64().unwrap()); // 6
        std.push(i[7].as_f64().unwrap()); // 7
        vol.push(i[8].as_u64().unwrap()); // 8
        val.push(i[9].as_f64().unwrap()); // 9
        trades.push(i[10].as_u64().unwrap()); // 10
        vwap.push(i[11].as_f64().unwrap()); // 11
        change.push(i[12].as_f64().unwrap()); // 12
        trades_b.push(i[13].as_u64().unwrap()); // 13
        trades_s.push(i[14].as_u64().unwrap()); // 14
        val_b.push(i[15].as_f64().unwrap()); // 15
        val_s.push(i[16].as_f64().unwrap()); // 16
        vol_b.push(i[17].as_u64().unwrap()); // 17
        vol_s.push(i[18].as_u64().unwrap()); // 18
        disb.push(i[19].as_f64().unwrap()); // 19
        vwap_b.push(i[20].as_f64().unwrap_or(0.0)); // 20
        vwap_s.push(i[21].as_f64().unwrap_or(0.0)); // 21
    }

    // convert date & time to timestamp
    let mut timestamps: Vec<i64> = Vec::new();
    let zipped_vec = date.iter().zip(time.iter());
    for (d, t) in zipped_vec {
        let str_dt = format!("{d} {t}+03:00");
        let dt = DateTime::parse_from_str(&str_dt, "%Y-%m-%d %H:%M:%S%z");
        let ts = dt.unwrap().timestamp_nanos_opt().unwrap();
        timestamps.push(ts);
    }

    // create DataFrame
    let df = df!(
        "ts_nanos" => timestamps,
        "open" => open,
        "high" => high,
        "low" => low,
        "close" => close,
        "std" => std,
        "vol" => vol,
        "val" => val,
        "trades" => trades,
        "vwap" => vwap,
        "change" => change,
        "trades_b" => trades_b,
        "trades_s" => trades_s,
        "val_b" => val_b,
        "val_s" => val_s,
        "vol_b" => vol_b,
        "vol_s" => vol_s,
        "disb" => disb,
        "vwap_b" => vwap_b,
        "vwap_s" => vwap_s,
    );

    df.unwrap()
}
