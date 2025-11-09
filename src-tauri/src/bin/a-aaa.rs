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
use strum::{IntoEnumIterator, VariantNames};

use avin_connect::*;
use avin_core::*;
use avin_data::*;
use avin_utils::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    // let source = Source::TINKOFF;
    // let iid = Manager::find_iid("moex_share_sber").unwrap();
    // let md = MarketData::BAR_1M;

    // Data::update(&iid, source, md).await.unwrap();

    // let r = Data::update_all().await;
    // match r {
    //     Ok(_) => (),
    //     Err(e) => log::error!("{e}"),
    // }
}
