/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(unused)]

use avin_analyse::*;
use avin_core::*;

fn main() {
    avin_utils::init_logger();

    // Trend::analyse_all().unwrap();
    // Bar::analyse_all().unwrap();
    Tic::analyse().unwrap();
    // Cluster::analyse_all().unwrap();
    // Quantum::analyse_all().unwrap();
}
