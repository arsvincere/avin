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
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let mut share = Share::new("moex_share_sber").unwrap();

    let tf = TimeFrame::new("D");
    let begin = Usr::date("2024-12-20");
    let end = Usr::date("2025-01-01");
    share.load_chart_period(&tf, &begin, &end).unwrap();

    let chart = share.mut_chart(&tf).unwrap();
    chart.features(Features::Extremum, true);

    // real-time trend
    let trend = chart.trend(&T1, 0).unwrap();
    println!("{}", trend);

    // last historical trend
    let trend = chart.trend(&T1, 1).unwrap();
    println!("{}", trend);

    let trend = chart.trend(&T1, 2).unwrap();
    println!("{}", trend);

    let trend = chart.trend(&T1, 3).unwrap();
    println!("{}", trend);
}
