#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    // let source = Source::MOEX;
    // let tickers = [
    //     "AFKS", "AFLT", "ASTR", "CHMF", "GAZP", "GMKN", "LKOH", "MOEX",
    //     "NLMK", "NVTK", "OZON", "PIKK", "PLZL", "ROSN", "SBER", "SNGS", "T",
    //     "TATN", "VTBR", "YDEX",
    // ];
    // let market_data = [
    //     MarketData::ORDER_STATS,
    //     MarketData::TRADE_STATS,
    //     MarketData::OB_STATS,
    // ];
    // let years = [2020, 2021, 2022, 2023, 2024, 2025];
    //
    // for ticker in tickers {
    //     let s = format!("moex_share_{ticker}");
    //     let iid = Manager::find_iid(&s).unwrap();
    //     for md in market_data {
    //         for year in years {
    //             Manager::download(source, &iid, md, Some(year))
    //                 .await
    //                 .unwrap();
    //         }
    //     }
    // }
}
