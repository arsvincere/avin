/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(dead_code)]
#![allow(unused)]

use avin::adviser::Adviser;
use avin::core::*;
use avin::utils;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let mut bars = vec![1, 2, 3, 4, 5];
    dbg!(&bars);

    let last_bar = bars.last_mut().unwrap();
    *last_bar = 555;

    dbg!(&bars);
}
