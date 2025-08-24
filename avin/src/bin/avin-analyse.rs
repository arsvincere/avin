/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::analyse::*;
use avin::core::*;
use avin::utils;

fn main() {
    utils::init_logger();

    // Trend::analyse_all().unwrap();
    // Bar::analyse_all().unwrap();
    // Cluster::analyse_all().unwrap();
    Quantum::analyse_all().unwrap();
}
