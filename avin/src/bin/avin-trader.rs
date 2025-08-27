/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::trader::*;
use avin::utils;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let mut trader = Trader::new();
    trader.start().await;
}
