/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::*;

fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    TrendAnalytic::analyse_all().unwrap();
    VolumeAnalytic::analyse_all().unwrap();
    // ClusterAnalytic::analyse_all().unwrap();
}
