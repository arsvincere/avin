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

use avin_analyse::*;
use avin_connect::*;
use avin_core::*;
use avin_data::*;
use avin_utils::*;

const SHUTDOWN_TIME: NaiveTime = NaiveTime::from_hms_opt(21, 0, 0).unwrap();

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let msg = "SBER Tic B".to_string();
    let body = "90.550.000".to_string();

    let mut command = Command::new("/bin/notify-send");
    command.arg("-u"); // silent
    command.arg("critical");
    command.arg(msg);
    command.arg(body);

    // execute
    command.spawn().unwrap().wait().unwrap();
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
