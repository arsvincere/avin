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

use avin_adviser::*;
use avin_analyse::*;
use avin_connect::*;
use avin_core::*;
use avin_data::*;
use avin_utils::*;

const SHUTDOWN_TIME: NaiveTime = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let iid = Manager::find_iid("moex_share_abio").unwrap();
    let md = MarketData::BAR_1M;
    let year = 2025;

    // download ABIO 2025 bars 1M
    Data::download(&iid, Source::TINKOFF, md, year)
        .await
        .unwrap();

    // delete ABIO data
    let path = iid.path();
    Cmd::delete_dir(&path).unwrap();
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
