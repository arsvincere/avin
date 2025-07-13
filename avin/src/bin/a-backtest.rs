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
    let tickers = ["AFKS", "LKOH", "ROSN"];

    for ticker in tickers {
        let s = format!("MOEX_SHARE_{ticker}");
        let asset = Asset::new(&s).unwrap();

        let strategy = PinBarLong::default();
        let mut test = Test::new(&strategy, asset.iid());
        let begin = utils::str_date_to_utc("2024-01-01");
        let end = utils::str_date_to_utc("2025-01-01");
        test.set_begin(&begin);
        test.set_end(&end);

        let mut tester = Tester::new();
        tester.run(strategy, &mut test).await;

        let summary = Summary::new(&test.trade_list);
        dbg!(summary);
    }
}
