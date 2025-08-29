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

    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let tf = TimeFrame::Day;
    let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let df = Manager::load(&iid, tf.market_data(), begin, end).unwrap();
    let bars = Bar::from_df(&df).unwrap();

    let chart = Chart::new(&iid, tf, bars);
    assert_eq!(*chart.iid(), iid);
    assert_eq!(chart.tf(), tf);
    assert_eq!(chart.bars().len(), 256);

    // let iid = Manager::find_iid("moex_share_sber").unwrap();
    // let md = MarketData::BAR_1M;
    //
    // let source = SourceMoex::new();
    // let from = Utc.with_ymd_and_hms(2025, 8, 4, 19, 20, 0).unwrap();
    // let till = Utc.with_ymd_and_hms(2025, 8, 5, 19, 20, 0).unwrap();
    // let bars = source.get(&iid, md, from, till).await;
    // dbg!(bars);
}
