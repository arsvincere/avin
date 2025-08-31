#![allow(dead_code)]
#![allow(unused)]

use chrono::prelude::*;

use avin_analyse::*;
use avin_core::*;
use avin_data::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let mut asset = Asset::new("moex_share_sber").unwrap();
    let tf = TimeFrame::M10;
    asset.load_chart(tf).unwrap();
    let chart = asset.chart_mut(tf).unwrap();

    let b2 = chart.bar(2).unwrap();
    let b1 = chart.bar(1).unwrap();
    let b0 = chart.bar(0).unwrap();

    println!("{b2}");
    println!("{b1}");
    println!("{b0}");

    // let iid = Manager::find_iid("moex_share_sber").unwrap();
    // let md = MarketData::BAR_1M;
    //
    // let source = SourceMoex::new();
    // let from = Utc.with_ymd_and_hms(2025, 8, 4, 19, 20, 0).unwrap();
    // let till = Utc.with_ymd_and_hms(2025, 8, 5, 19, 20, 0).unwrap();
    // let bars = source.get(&iid, md, from, till).await;
    // dbg!(bars);
}
