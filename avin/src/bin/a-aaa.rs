#![allow(dead_code)]
#![allow(unused)]

use avin_analyse::*;
use avin_core::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;

use polars::prelude::*;

#[tokio::main]
async fn main() {
    let source = Source::MOEX;
    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let md = MarketData::TRADE_STATS;
    let year = 2024;

    Manager::download(source, &iid, md, year).await.unwrap();
}
