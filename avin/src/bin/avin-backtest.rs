#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use chrono::prelude::*;

use avin::connect::*;
use avin::core::*;
use avin::strategy::*;
use avin::tester::*;
use avin::utils;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let tickers = ["SBER"];
    // let tickers = [
    //     "AFKS", "AFLT", "ASTR", "CHMF", "GAZP", "GMKN", "LKOH", "MOEX",
    //     "NLMK", "NVTK", "OZON", "PIKK", "PLZL", "ROSN", "SBER", "SNGS", "T",
    //     "TATN", "VTBR", "YDEX",
    // ];

    for ticker in tickers {
        let strategy = BigTrendShort::default();
        let asset = Asset::new(&format!("MOEX_SHARE_{ticker}")).unwrap();
        let begin = utils::str_date_to_utc("2024-01-01");
        let end = utils::str_date_to_utc("2025-01-01");
        // let begin = utils::str_date_to_utc("2025-01-01");
        // let end = utils::str_date_to_utc("2026-01-01");

        let mut test = Test::new(&strategy, asset.iid());
        test.set_begin(&begin);
        test.set_end(&end);

        let mut tester = Tester::new();
        tester.run(strategy, &mut test).await;

        let summary = Summary::new(&test.trade_list);
        println!("{summary}");
    }
}
