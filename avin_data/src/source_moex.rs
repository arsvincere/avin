/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::Days;
use chrono::TimeDelta;
use chrono::prelude::*;
use polars::prelude::*;

use avin_utils::AvinError;
use avin_utils::CFG;
use avin_utils::Cmd;

use avin_core::{DataSchema, Iid, MarketData};

const SERVICE: &str = "https://apim.moex.com/iss";
const MSK_TIME_DIF: TimeDelta = TimeDelta::new(10800, 0).unwrap();

pub struct SourceMoex {
    token: String,
    client: reqwest::Client,
}
impl Default for SourceMoex {
    fn default() -> Self {
        SourceMoex::new()
    }
}
impl SourceMoex {
    // build
    pub fn new() -> Self {
        let token_path = CFG.connect.moex_token();
        let token = Cmd::read(&token_path).unwrap().trim().to_string();

        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    // public
    #[allow(dead_code)]
    pub async fn cache_instruments_info() -> Result<(), AvinError> {
        todo!()
    }
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
            MarketData::BAR_DAY => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_WEEK => self.get_bars(iid, md, from, till).await,
            MarketData::BAR_MONTH => self.get_bars(iid, md, from, till).await,
            MarketData::TIC => todo!(),
            MarketData::TRADE_STATS => self.get_trades(iid, from, till).await,
            MarketData::ORDER_STATS => self.get_orders(iid, from, till).await,
            MarketData::OB_STATS => self.get_ob(iid, from, till).await,
        }
    }

    // get bars
    pub async fn get_bars(
        &self,
        iid: &Iid,
        _md: MarketData,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        // TODO: make this function private

        let mut bars = DataFrame::empty_with_schema(&DataSchema::bar());

        let mut dt = from;
        while dt < till {
            let response = self
                .try_request_bars(iid, MarketData::BAR_1M, &dt, &till)
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

    // get tics
    // TODO:

    // get trades
    async fn get_trades(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        let mut trades = DataFrame::empty_with_schema(&DataSchema::trades());

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

    // get orders
    async fn get_orders(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        let mut ob = DataFrame::empty_with_schema(&DataSchema::orders());

        let mut f = from;
        while f < till {
            // NOTE: данные отдает не более 1000шт за раз, поэтому
            // качаем по 3 дня. Время игнорится, from-till учитывается
            // только дата дня, время отбрасывается.
            let t = f.checked_add_days(Days::new(3)).unwrap();
            let response = self.try_request_orders(iid, f, t).await.unwrap();

            let json: serde_json::Value = match response.json().await {
                Err(e) => {
                    log::error!("Error parsing response to json: {e}");
                    panic!();
                }
                Ok(json) => json,
            };

            let part = parse_json_orders_stat(json);
            ob.extend(&part).unwrap();

            if f < till {
                f = t;
            } else {
                break;
            }
        }

        ob = drop_duplicate_timestamp(ob);

        Ok(ob)
    }
    async fn try_request_orders(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<reqwest::Response, AvinError> {
        let url = self.get_url_orders_stat(iid, from, till);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        Ok(response)
    }
    fn get_url_orders_stat(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> String {
        // # пример
        // https://apim.moex.com/iss/datashop/algopack/eq/orderstats/ABIO.json?
        // from=2024-04-17&
        // till=2024-04-19

        let from = from.date();
        let till = till.date();

        let mut url = String::from(SERVICE);
        url += "/datashop/algopack/eq/orderstats";
        url += format!("/{}.json?", iid.ticker()).as_str();
        url += format!("from={from}&").as_str();
        url += format!("till={till}").as_str();

        url
    }

    // get ob
    async fn get_ob(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<DataFrame, AvinError> {
        let mut ob = DataFrame::empty_with_schema(&DataSchema::ob());

        let mut f = from;
        while f < till {
            // NOTE: данные отдает не более 1000шт за раз, поэтому
            // качаем по 3 дня. Время игнорится, from-till учитывается
            // только дата дня, время отбрасывается.
            let t = f.checked_add_days(Days::new(3)).unwrap();
            let response = self.try_request_ob(iid, f, t).await.unwrap();

            let json: serde_json::Value = match response.json().await {
                Err(e) => {
                    log::error!("Error parsing response to json: {e}");
                    panic!();
                }
                Ok(json) => json,
            };

            let part = parse_json_ob_stat(json);
            ob.extend(&part).unwrap();

            if f < till {
                f = t;
            } else {
                break;
            }
        }

        ob = drop_duplicate_timestamp(ob);

        Ok(ob)
    }
    async fn try_request_ob(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<reqwest::Response, AvinError> {
        let url = self.get_url_ob_stat(iid, from, till);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        Ok(response)
    }
    fn get_url_ob_stat(
        &self,
        iid: &Iid,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> String {
        // # пример
        // https://apim.moex.com/iss/datashop/algopack/eq/obstats/ABIO.json?
        // from=2024-04-17&
        // till=2024-04-19

        let from = from.date();
        let till = till.date();

        let mut url = String::from(SERVICE);
        url += "/datashop/algopack/eq/obstats";
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
        MarketData::BAR_DAY => "interval=24",
        MarketData::BAR_WEEK => "interval=7",
        MarketData::BAR_MONTH => "interval=31",
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

    df.unique_stable(Some(&[col_name]), UniqueKeepStrategy::Last, None)
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
    let mut vol: Vec<i64> = Vec::new();
    // let val: Vec<f64> = Vec::new();

    for candle in candles_data {
        let array = candle.as_array().unwrap();

        let o = array[0].as_f64().unwrap();
        let c = array[1].as_f64().unwrap();
        let h = array[2].as_f64().unwrap();
        let l = array[3].as_f64().unwrap();
        // let val = array[4].as_f64().unwrap();
        let v = array[5].as_i64().unwrap();
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

    // Array [
    //     String("2025-01-03"),
    //     String("23:45:00"),
    //     String("SBER"),
    //     Number(0.8),
    //     Number(9.8),
    //     Number(1.8),
    //     Number(741),
    //     Number(1050),
    //     Number(285433),
    //     Number(306540),
    //     Number(760228680),
    //     Number(866640230),
    //     Number(-0.39),
    //     Number(-0.39),
    //     Number(-0.04),
    //     Number(-0.07),
    //     Number(266.34),
    //     Number(282.72),
    //     Number(272.37),
    //     Number(272.42),
    //     String("2025-06-10 14:54:58"),
    // ],

    // tmp Vec for create DataFrame
    let mut date: Vec<&str> = Vec::new(); // 0
    let mut time: Vec<&str> = Vec::new(); // 1
    let mut open: Vec<Option<f64>> = Vec::new(); // 3
    let mut high: Vec<Option<f64>> = Vec::new(); // 4
    let mut low: Vec<Option<f64>> = Vec::new(); // 5
    let mut close: Vec<Option<f64>> = Vec::new(); // 6
    let mut std: Vec<Option<f64>> = Vec::new(); // 7
    let mut vol: Vec<Option<i64>> = Vec::new(); // 8
    let mut val: Vec<Option<f64>> = Vec::new(); // 9
    let mut trades: Vec<Option<i64>> = Vec::new(); // 10
    let mut vwap: Vec<Option<f64>> = Vec::new(); // 11
    let mut change: Vec<Option<f64>> = Vec::new(); // 12
    let mut trades_b: Vec<Option<i64>> = Vec::new(); // 13
    let mut trades_s: Vec<Option<i64>> = Vec::new(); // 14
    let mut val_b: Vec<Option<f64>> = Vec::new(); // 15
    let mut val_s: Vec<Option<f64>> = Vec::new(); // 16
    let mut vol_b: Vec<Option<i64>> = Vec::new(); // 17
    let mut vol_s: Vec<Option<i64>> = Vec::new(); // 18
    let mut disb: Vec<Option<f64>> = Vec::new(); // 19
    let mut vwap_b: Vec<Option<f64>> = Vec::new(); // 20
    let mut vwap_s: Vec<Option<f64>> = Vec::new(); // 21

    // collect values
    let data = json["data"]["data"].as_array().unwrap();
    for i in data {
        date.push(i[0].as_str().unwrap()); // 0
        time.push(i[1].as_str().unwrap()); // 1
        open.push(i[3].as_f64()); // 3
        high.push(i[4].as_f64()); // 4
        low.push(i[5].as_f64()); // 5
        close.push(i[6].as_f64()); // 6
        std.push(i[7].as_f64()); // 7
        vol.push(i[8].as_i64()); // 8
        val.push(i[9].as_f64()); // 9
        trades.push(i[10].as_i64()); // 10
        vwap.push(i[11].as_f64()); // 11
        change.push(i[12].as_f64()); // 12
        trades_b.push(i[13].as_i64()); // 13
        trades_s.push(i[14].as_i64()); // 14
        val_b.push(i[15].as_f64()); // 15
        val_s.push(i[16].as_f64()); // 16
        vol_b.push(i[17].as_i64()); // 17
        vol_s.push(i[18].as_i64()); // 18
        disb.push(i[19].as_f64()); // 19
        vwap_b.push(i[20].as_f64()); // 20
        vwap_s.push(i[21].as_f64()); // 21
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
fn parse_json_orders_stat(json: serde_json::Value) -> DataFrame {
    // json["data"]["columns"] = Array [
    //     String("tradedate"),
    //     String("tradetime"),
    //     String("secid"),
    //     String("put_orders_b"),
    //     String("put_orders_s"),
    //     String("put_val_b"),
    //     String("put_val_s"),
    //     String("put_vol_b"),
    //     String("put_vol_s"),
    //     String("put_vwap_b"),
    //     String("put_vwap_s"),
    //     String("put_vol"),
    //     String("put_val"),
    //     String("put_orders"),
    //     String("cancel_orders_b"),
    //     String("cancel_orders_s"),
    //     String("cancel_val_b"),
    //     String("cancel_val_s"),
    //     String("cancel_vol_b"),
    //     String("cancel_vol_s"),
    //     String("cancel_vwap_b"),
    //     String("cancel_vwap_s"),
    //     String("cancel_vol"),
    //     String("cancel_val"),
    //     String("cancel_orders"),
    //     String("SYSTIME"),
    // ]

    // json["data"]["metadata"] = Object {
    //     "SYSTIME": Object {
    //         "bytes": Number(19),
    //         "max_size": Number(0),
    //         "type": String("datetime"),
    //     },
    //     "cancel_orders": Object {
    //         "type": String("int64"),
    //     },
    //     "cancel_orders_b": Object {
    //         "type": String("int32"),
    //     },
    //     "cancel_orders_s": Object {
    //         "type": String("int32"),
    //     },
    //     "cancel_val": Object {
    //         "type": String("double"),
    //     },
    //     "cancel_val_b": Object {
    //         "type": String("double"),
    //     },
    //     "cancel_val_s": Object {
    //         "type": String("double"),
    //     },
    //     "cancel_vol": Object {
    //         "type": String("int64"),
    //     },
    //     "cancel_vol_b": Object {
    //         "type": String("int32"),
    //     },
    //     "cancel_vol_s": Object {
    //         "type": String("int64"),
    //     },
    //     "cancel_vwap_b": Object {
    //         "type": String("double"),
    //     },
    //     "cancel_vwap_s": Object {
    //         "type": String("double"),
    //     },
    //     "put_orders": Object {
    //         "type": String("int32"),
    //     },
    //     "put_orders_b": Object {
    //         "type": String("int32"),
    //     },
    //     "put_orders_s": Object {
    //         "type": String("int32"),
    //     },
    //     "put_val": Object {
    //         "type": String("double"),
    //     },
    //     "put_val_b": Object {
    //         "type": String("double"),
    //     },
    //     "put_val_s": Object {
    //         "type": String("double"),
    //     },
    //     "put_vol": Object {
    //         "type": String("int32"),
    //     },
    //     "put_vol_b": Object {
    //         "type": String("int32"),
    //     },
    //     "put_vol_s": Object {
    //         "type": String("int32"),
    //     },
    //     "put_vwap_b": Object {
    //         "type": String("double"),
    //     },
    //     "put_vwap_s": Object {
    //         "type": String("double"),
    //     },
    //     "secid": Object {
    //         "bytes": Number(36),
    //         "max_size": Number(0),
    //         "type": String("string"),
    //     },
    //     "tradedate": Object {
    //         "bytes": Number(10),
    //         "max_size": Number(0),
    //         "type": String("date"),
    //     },
    //     "tradetime": Object {
    //         "bytes": Number(10),
    //         "max_size": Number(0),
    //         "type": String("time"),
    //     },
    // }

    // Array [
    //     String("2025-01-03"),
    //     String("23:50:00"),
    //     String("SBER"),
    //     Number(3228),
    //     Number(277),
    //     Number(529675423),
    //     Number(104444143),
    //     Number(198557),
    //     Number(46465),
    //     Number(266.76),
    //     Number(224.78),
    //     Number(245022),
    //     Number(634119566),
    //     Number(3505),
    //     Number(3112),
    //     Number(269),
    //     Number(502970767),
    //     Number(197531368),
    //     Number(185138),
    //     Number(70461),
    //     Number(271.67),
    //     Number(280.34),
    //     Number(255599),
    //     Number(700502136),
    //     Number(3381),
    //     String("2025-01-03 23:50:13"),
    // ],

    // tmp Vec for create DataFrame
    let mut date: Vec<&str> = Vec::new(); // 0
    let mut time: Vec<&str> = Vec::new(); // 1
    let mut put_orders_b: Vec<Option<i64>> = Vec::new(); // 3
    let mut put_orders_s: Vec<Option<i64>> = Vec::new(); // 4
    let mut put_val_b: Vec<Option<f64>> = Vec::new(); // 5
    let mut put_val_s: Vec<Option<f64>> = Vec::new(); // 6
    let mut put_vol_b: Vec<Option<i64>> = Vec::new(); // 7
    let mut put_vol_s: Vec<Option<i64>> = Vec::new(); // 8
    let mut put_vwap_b: Vec<Option<f64>> = Vec::new(); // 9
    let mut put_vwap_s: Vec<Option<f64>> = Vec::new(); // 10
    let mut put_vol: Vec<Option<i64>> = Vec::new(); // 11
    let mut put_val: Vec<Option<f64>> = Vec::new(); // 12
    let mut put_orders: Vec<Option<i64>> = Vec::new(); // 13
    let mut cancel_orders_b: Vec<Option<i64>> = Vec::new(); // 14
    let mut cancel_orders_s: Vec<Option<i64>> = Vec::new(); // 15
    let mut cancel_val_b: Vec<Option<f64>> = Vec::new(); // 16
    let mut cancel_val_s: Vec<Option<f64>> = Vec::new(); // 17
    let mut cancel_vol_b: Vec<Option<i64>> = Vec::new(); // 18
    let mut cancel_vol_s: Vec<Option<i64>> = Vec::new(); // 19
    let mut cancel_vwap_b: Vec<Option<f64>> = Vec::new(); // 20
    let mut cancel_vwap_s: Vec<Option<f64>> = Vec::new(); // 21
    let mut cancel_vol: Vec<Option<i64>> = Vec::new(); // 22
    let mut cancel_val: Vec<Option<f64>> = Vec::new(); // 23
    let mut cancel_orders: Vec<Option<i64>> = Vec::new(); // 24

    // collect values
    let data = json["data"]["data"].as_array().unwrap();
    for i in data {
        date.push(i[0].as_str().unwrap());
        time.push(i[1].as_str().unwrap());
        put_orders_b.push(i[3].as_i64());
        put_orders_s.push(i[4].as_i64());
        put_val_b.push(i[5].as_f64());
        put_val_s.push(i[6].as_f64());
        put_vol_b.push(i[7].as_i64());
        put_vol_s.push(i[8].as_i64());
        put_vwap_b.push(i[9].as_f64());
        put_vwap_s.push(i[10].as_f64());
        put_vol.push(i[11].as_i64());
        put_val.push(i[12].as_f64());
        put_orders.push(i[13].as_i64());
        cancel_orders_b.push(i[14].as_i64());
        cancel_orders_s.push(i[15].as_i64());
        cancel_val_b.push(i[16].as_f64());
        cancel_val_s.push(i[17].as_f64());
        cancel_vol_b.push(i[18].as_i64());
        cancel_vol_s.push(i[19].as_i64());
        cancel_vwap_b.push(i[20].as_f64());
        cancel_vwap_s.push(i[21].as_f64());
        cancel_vol.push(i[22].as_i64());
        cancel_val.push(i[23].as_f64());
        cancel_orders.push(i[24].as_i64());
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
        "put_orders_b" => put_orders_b,
        "put_orders_s" => put_orders_s,
        "put_val_b" => put_val_b,
        "put_val_s" => put_val_s,
        "put_vol_b" => put_vol_b,
        "put_vol_s" => put_vol_s,
        "put_vwap_b" => put_vwap_b,
        "put_vwap_s" => put_vwap_s,
        "put_vol" => put_vol,
        "put_val" => put_val,
        "put_orders" => put_orders,
        "cancel_orders_b" => cancel_orders_b,
        "cancel_orders_s" => cancel_orders_s,
        "cancel_val_b" => cancel_val_b,
        "cancel_val_s" => cancel_val_s,
        "cancel_vol_b" => cancel_vol_b,
        "cancel_vol_s" => cancel_vol_s,
        "cancel_vwap_b" => cancel_vwap_b,
        "cancel_vwap_s" => cancel_vwap_s,
        "cancel_vol" => cancel_vol,
        "cancel_val" => cancel_val,
        "cancel_orders" => cancel_orders,
    );

    df.unwrap()
}
fn parse_json_ob_stat(json: serde_json::Value) -> DataFrame {
    // json["data"]["columns"] = Array [
    //     String("tradedate"),
    //     String("tradetime"),
    //     String("secid"),
    //     String("spread_bbo"),
    //     String("spread_lv10"),
    //     String("spread_1mio"),
    //     String("levels_b"),
    //     String("levels_s"),
    //     String("vol_b"),
    //     String("vol_s"),
    //     String("val_b"),
    //     String("val_s"),
    //     String("imbalance_vol_bbo"),
    //     String("imbalance_val_bbo"),
    //     String("imbalance_vol"),
    //     String("imbalance_val"),
    //     String("vwap_b"),
    //     String("vwap_s"),
    //     String("vwap_b_1mio"),
    //     String("vwap_s_1mio"),
    //     String("SYSTIME"),
    // ]

    // json["data"]["metadata"] = Object {
    //     "SYSTIME": Object {
    //         "bytes": Number(19),
    //         "max_size": Number(0),
    //         "type": String("datetime"),
    //     },
    //     "imbalance_val": Object {
    //         "type": String("double"),
    //     },
    //     "imbalance_val_bbo": Object {
    //         "type": String("double"),
    //     },
    //     "imbalance_vol": Object {
    //         "type": String("double"),
    //     },
    //     "imbalance_vol_bbo": Object {
    //         "type": String("double"),
    //     },
    //     "levels_b": Object {
    //         "type": String("int32"),
    //     },
    //     "levels_s": Object {
    //         "type": String("int32"),
    //     },
    //     "secid": Object {
    //         "bytes": Number(36),
    //         "max_size": Number(0),
    //         "type": String("string"),
    //     },
    //     "spread_1mio": Object {
    //         "type": String("double"),
    //     },
    //     "spread_bbo": Object {
    //         "type": String("double"),
    //     },
    //     "spread_lv10": Object {
    //         "type": String("double"),
    //     },
    //     "tradedate": Object {
    //         "bytes": Number(10),
    //         "max_size": Number(0),
    //         "type": String("date"),
    //     },
    //     "tradetime": Object {
    //         "bytes": Number(10),
    //         "max_size": Number(0),
    //         "type": String("time"),
    //     },
    //     "val_b": Object {
    //         "type": String("int64"),
    //     },
    //     "val_s": Object {
    //         "type": String("int64"),
    //     },
    //     "vol_b": Object {
    //         "type": String("int64"),
    //     },
    //     "vol_s": Object {
    //         "type": String("int64"),
    //     },
    //     "vwap_b": Object {
    //         "type": String("double"),
    //     },
    //     "vwap_b_1mio": Object {
    //         "type": String("double"),
    //     },
    //     "vwap_s": Object {
    //         "type": String("double"),
    //     },
    //     "vwap_s_1mio": Object {
    //         "type": String("double"),
    //     },
    // }

    // json["data"]["data"] = Object {
    // Array [
    //     String("2025-01-03"),
    //     String("23:50:00"),
    //     String("SBER"),
    //     Number(1),
    //     Number(11.4),
    //     Number(2.3),
    //     Number(709),
    //     Number(1044),
    //     Number(267125),
    //     Number(283553),
    //     Number(711028247),
    //     Number(800643986),
    //     Number(-0.02),
    //     Number(-0.02),
    //     Number(-0.03),
    //     Number(-0.06),
    //     Number(266.18),
    //     Number(282.34),
    //     Number(272.17),
    //     Number(272.23),
    //     String("2025-06-10 14:54:58"),
    // ],

    // tmp Vec for create DataFrame
    let mut date: Vec<&str> = Vec::new(); // 0
    let mut time: Vec<&str> = Vec::new(); // 1
    let mut spread_bbo: Vec<Option<f64>> = Vec::new(); // 3
    let mut spread_lv10: Vec<Option<f64>> = Vec::new(); // 4
    let mut spread_1mio: Vec<Option<f64>> = Vec::new(); // 5
    let mut levels_b: Vec<Option<i64>> = Vec::new(); // 6
    let mut levels_s: Vec<Option<i64>> = Vec::new(); // 7
    let mut vol_b: Vec<Option<i64>> = Vec::new(); // 8
    let mut vol_s: Vec<Option<i64>> = Vec::new(); // 9
    let mut val_b: Vec<Option<i64>> = Vec::new(); // 10
    let mut val_s: Vec<Option<i64>> = Vec::new(); // 11
    let mut imbalance_vol_bbo: Vec<Option<f64>> = Vec::new(); // 12
    let mut imbalance_val_bbo: Vec<Option<f64>> = Vec::new(); // 13
    let mut imbalance_vol: Vec<Option<f64>> = Vec::new(); // 14
    let mut imbalance_val: Vec<Option<f64>> = Vec::new(); // 15
    let mut vwap_b: Vec<Option<f64>> = Vec::new(); // 16
    let mut vwap_s: Vec<Option<f64>> = Vec::new(); // 17
    let mut vwap_b_1mio: Vec<Option<f64>> = Vec::new(); // 18
    let mut vwap_s_1mio: Vec<Option<f64>> = Vec::new(); // 19

    // collect values
    let data = json["data"]["data"].as_array().unwrap();
    for i in data {
        date.push(i[0].as_str().unwrap());
        time.push(i[1].as_str().unwrap());
        spread_bbo.push(i[3].as_f64());
        spread_lv10.push(i[4].as_f64());
        spread_1mio.push(i[5].as_f64());
        levels_b.push(i[6].as_i64());
        levels_s.push(i[7].as_i64());
        vol_b.push(i[8].as_i64());
        vol_s.push(i[9].as_i64());
        val_b.push(i[10].as_i64());
        val_s.push(i[11].as_i64());
        imbalance_vol_bbo.push(i[12].as_f64());
        imbalance_val_bbo.push(i[13].as_f64());
        imbalance_vol.push(i[14].as_f64());
        imbalance_val.push(i[15].as_f64());
        vwap_b.push(i[16].as_f64());
        vwap_s.push(i[17].as_f64());
        vwap_b_1mio.push(i[18].as_f64());
        vwap_s_1mio.push(i[19].as_f64());
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
        "spread_bbo" => spread_bbo,
        "spread_lv10" => spread_lv10,
        "spread_1mio" => spread_1mio,
        "levels_b" => levels_b,
        "levels_s" => levels_s,
        "vol_b" => vol_b,
        "vol_s" => vol_s,
        "val_b" => val_b,
        "val_s" => val_s,
        "imbalance_vol_bbo" => imbalance_vol_bbo,
        "imbalance_val_bbo" => imbalance_val_bbo,
        "imbalance_vol" => imbalance_vol,
        "imbalance_val" => imbalance_val,
        "vwap_b" => vwap_b,
        "vwap_s" => vwap_s,
        "vwap_b_1mio" => vwap_b_1mio,
        "vwap_s_1mio" => vwap_s_1mio,
    );

    df.unwrap()
}
