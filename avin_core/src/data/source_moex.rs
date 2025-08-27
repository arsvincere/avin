/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, NaiveDateTime, Utc};
use polars::prelude::*;

use avin_utils::{AvinError, CFG, Cmd, MSK_OFFSET};

use crate::{Iid, MarketData};

pub struct SourceMoex {
    token: String,
    service: String,
    client: reqwest::Client,
}
impl SourceMoex {
    pub fn new() -> Self {
        let token_path = CFG.connect.moex_token();
        let token = Cmd::read(&token_path).unwrap().trim().to_string();

        Self {
            token,
            service: "https://apim.moex.com/iss".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(
        &self,
        iid: &Iid,
        market_data: MarketData,
        begin: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DataFrame, AvinError> {
        let from = Self::utc_to_msk(begin);
        let till = Self::utc_to_msk(end);

        let response = self
            .try_request(iid, market_data, from, till)
            .await
            .unwrap();

        let json: serde_json::Value = match response.json().await {
            Err(e) => {
                eprintln!("Error parsing response: {e}");
                panic!();
            }
            Ok(json) => json,
        };

        let df = self.parse_json_trades_stat(json);

        Ok(df)

        //     "data": [
        //         ["2024-04-17", "10:05:00", "ABIO", 115.28, 115.5, 114.76, 115.14, 0.0008, 1625, 1869839, 74, 115.07, -0.1214, 30, 44, 710666, 1159173, 617, 1008, -0.24, 115.18, 115, "2024-04-17 10:05:12"],
    }

    fn parse_json_trades_stat(&self, _json: serde_json::Value) -> DataFrame {
        // json["data"]["data"]["columns"] = Array [
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

        let mut _date_time: Vec<&str> = Vec::new();

        todo!();
    }

    // let candles_data = json["candles"]["data"].as_array().unwrap();

    async fn try_request(
        &self,
        iid: &Iid,
        market_data: MarketData,
        from: NaiveDateTime,
        till: NaiveDateTime,
    ) -> Result<reqwest::Response, AvinError> {
        let url = self.get_url(iid, market_data, from, till);
        let request = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .build()
            .unwrap();
        let response = self.client.execute(request).await.unwrap();

        dbg!(&response);

        Ok(response)
    }
    fn get_url(
        &self,
        iid: &Iid,
        market_data: MarketData,
        _begin: NaiveDateTime,
        _end: NaiveDateTime,
    ) -> String {
        let mut url = self.service.clone();

        match market_data {
            MarketData::TRADE_STATS => {
                url += "/datashop/algopack/eq/tradestats";
                url += format!("/{}.json?", iid.ticker()).as_str();
                url += "from=2024-01-01&";
                url += "till=2025-01-01";
            }
            _ => todo!(),
        }

        dbg!(&url);

        url
    }
    fn utc_to_msk(dt: DateTime<Utc>) -> NaiveDateTime {
        dt.naive_utc() + MSK_OFFSET
    }
}
