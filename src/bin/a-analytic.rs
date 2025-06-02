/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::{Analytic, TrendAnalytic, utils};

fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    TrendAnalytic::analyse_all().unwrap();

    // let share = Share::new("MOEX_SHARE_GAZP").unwrap();
    // let tf = TimeFrame::Day;
    // TrendAnalytic::analyse(share.iid(), &tf).unwrap();
}
