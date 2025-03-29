use avin::Instrument;
use avin::Manager;
use avin::MarketData;
use chrono::prelude::*;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let instr = Instrument::from("moex_share_sber").unwrap();
    let market_data = MarketData::BAR_1M;
    let begin = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    let start = Instant::now();
    let bars = Manager::request(&instr, &market_data, &begin, &end).unwrap();
    let duration = start.elapsed();

    println!("Request bars: {:?}", duration);
}

// Request bars: 944.485158ms  - collect
// Request bars: 785.288914ms  - iter
