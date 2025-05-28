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
use utils::Timer;

#[tokio::main]
async fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    let share = Share::new("MOEX_SHARE_GAZP").unwrap();
    let tf = TimeFrame::M1;
    let term = Term::T1;
    let begin = Usr::date("2025-01-01");
    let end = Usr::date("2025-02-01");
    let mut chart = Chart::load(share.iid(), &tf, &begin, &end).unwrap();
    chart.features(ChartFeatures::Extremum, true);

    let trend = chart.trend(&term, 1).unwrap();
    println!("{}", trend);

    let t = Timer::new();
    let p = TrendAnalytic::posterior(&trend).unwrap();
    t.stop("Time:");

    dbg!(p);
}
