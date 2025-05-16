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
    log::info!("Welcome to AVIN Trade System!");

    // let share = Share::new("moex_share_sber").unwrap();
    // let tf = TimeFrame::new("1M");
    //
    // TrendAnalytic::analyse(&share, &tf).unwrap();
    //
    let shares = Share::all();
    for i in shares.iter() {
        println!("Share: {}", i);
    }
}
