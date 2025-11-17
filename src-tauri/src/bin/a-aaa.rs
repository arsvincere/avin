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

    // if let Some(e) = event_rx.recv().await {
    //     let figi = e.figi();
    //     let asset = self.asset_list.find_figi_mut(figi).unwrap();
    //
    //     log::debug!("Event {e}");
    //
    //     match e {
    //         Event::Bar(e) => asset.bar_event(e),
    //         Event::Tic(e) => asset.tic_event(e),
    //         Event::Order(_e) => todo!(),
    //         Event::OrderBook(_e) => todo!(),
    //     }
    //
    //     // TODO:
    //     // Тут теперь когда ассеты обновлены можно применять к
    //     // ним фильтр и выдавать звуковой сигнал
    //
    //     for filter in self.filters {
    //         let result = filter.apply(asset);
    //         if let Some(notice) = result {
    //             self.notify(notice);
    //         }
    //     }
    // }
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
