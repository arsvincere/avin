/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use std::{fs::File, path::Path, process::Command};

use chrono::{
    DateTime, Datelike, Days, NaiveDate, NaiveTime, TimeDelta, TimeZone,
    Timelike, Utc,
};
use polars::io::SerReader;
use polars::prelude::*;
use strum::{IntoEnumIterator, VariantNames};

use avin_connect::*;
use avin_core::*;
use avin_data::*;
use avin_utils::*;

const SHUTDOWN_TIME: NaiveTime = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let iid = Manager::find_iid("moex_share_gmkn").unwrap();
    let source = Source::TINKOFF;
    let md = MarketData::BAR_1M;
    let year = 2021;

    Data::download(&iid, source, md, year).await.unwrap();
}

//------------------------------------------------------------------------------

struct Client {}
impl Client {}

struct Broker {}
impl Broker {
    fn new() -> Self {
        Self {}
    }
}
