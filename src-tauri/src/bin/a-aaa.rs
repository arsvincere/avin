/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use std::{fs::File, path::Path, process::Command};

use chrono::{DateTime, Datelike, TimeDelta, TimeZone, Utc};
use polars::io::SerReader;
use polars::prelude::*;
use strum::IntoEnumIterator;

use avin_connect::*;
use avin_core::*;
use avin_data::*;
use avin_utils::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let source = Source::TINKOFF;
    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let md = MarketData::BAR_1M;

    // let year = 2018;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2019;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2020;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2021;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2022;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2023;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2024;
    // Data::download(&iid, source, md, year).await.unwrap();
    // let year = 2025;
    // Data::download(&iid, source, md, year).await.unwrap();

    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_5M;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_10M;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_15M;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_1H;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_4H;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_DAY;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_WEEK;
    // Data::convert(&iid, source, input, output).unwrap();
    // let input = MarketData::BAR_1M;
    // let output = MarketData::BAR_MONTH;
    // Data::convert(&iid, source, input, output).unwrap();

    // let md = MarketData::BAR_1M;
    // Data::update(&iid, source, md).await.unwrap();

    // let r = Data::update_all().await;
    // match r {
    //     Ok(_) => (),
    //     Err(e) => log::error!("{e}"),
    // }
}
