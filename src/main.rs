/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 *                                                                         *
 * * * * * * * * Open source cross-platform trading system * * * * * * * * */

use avin::*;

#[tokio::main]
async fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    let mut share = Share::new("moex_share_sber").unwrap();
    let tf = TimeFrame::Day;
    let begin = Usr::date("2024-12-20");
    let end = Usr::date("2025-01-01");
    let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

    let v = chart.bars();
    // for i in v.iter() {
    //     println!("{:?}", i);
    // }
    // Bar { ts_nanos: 1734642000000000000, ...
    // Bar { ts_nanos: 1734901200000000000, ...  // from
    // Bar { ts_nanos: 1734987600000000000, ...
    // Bar { ts_nanos: 1735074000000000000, ...  // till
    // Bar { ts_nanos: 1735160400000000000, ...
    // Bar { ts_nanos: 1735246800000000000, ...
    // Bar { ts_nanos: 1735333200000000000, ...
    // Bar { ts_nanos: 1735506000000000000, ...

    let from = Chart::bisect_left(v, &1734901200000000000);
    let till = Chart::bisect_left(v, &1735074000000000000);
    let s = &v[from..=till];
    assert_eq!(s.len(), 3);

    // // test other values
    // assert_eq!(Chart::bisect_left(v, &1734642000000000000), 0); // x == 0
    assert_eq!(Chart::bisect_left(v, &1734901200000000000), 1); // x == 1
    assert_eq!(Chart::bisect_left(v, &1734987600000000000), 2); // x == 2
    assert_eq!(Chart::bisect_left(v, &1735074000000000000), 3); // x == 3
    assert_eq!(Chart::bisect_left(v, &1735160400000000000), 4); // x == 4
    assert_eq!(Chart::bisect_left(v, &1735246800000000000), 5); // x == 5
    assert_eq!(Chart::bisect_left(v, &1735333200000000000), 6); // x == 6
    assert_eq!(Chart::bisect_left(v, &1735506000000000000), 7); // x == 7
    //
    // // test out of vector values
    // assert_eq!(Chart::bisect_left(v, &1000000000000000000), 0); // x < 0
    // assert_eq!(Chart::bisect_left(v, &1734642000000000001), 1); // 0<x<1
    // assert_eq!(Chart::bisect_left(v, &1999999999999999999), 8); // 7 < x
}
