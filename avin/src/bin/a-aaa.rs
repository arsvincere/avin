#![allow(dead_code)]
#![allow(unused)]

use avin_analyse::*;
use avin_core::*;
use avin_data::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();
    let m = SourceMoex::new();
    let begin = Utc.with_ymd_and_hms(2025, 8, 4, 19, 20, 0).unwrap();
    let till = Utc.with_ymd_and_hms(2025, 8, 5, 19, 20, 0).unwrap();
    let bars = m.get_bars(&begin, &till).await;
    dbg!(bars);
}
