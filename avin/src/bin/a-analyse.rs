use avin::analyse::*;
use avin::core::*;
use avin::utils;

fn main() {
    utils::init_logger();

    // Trend::analyse_all().unwrap();
    // Bar::analyse_all().unwrap();
    // ClusterAnalytic::analyse_all().unwrap();

    let tickers = ["SBER", "SNGS", "T", "TATN", "VTBR", "YDEX"];
    for ticker in tickers {
        let s = format!("MOEX_SHARE_{ticker}");
        let iid = Manager::find_iid(&s).unwrap();
        for tf in TimeFrame::all() {
            Bar::analyse(&iid, tf).unwrap();
        }
    }
}
