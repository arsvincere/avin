/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(dead_code)]
#![allow(unused)]

use avin_adviser::Adviser;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let mut adviser = Adviser::new();
    adviser.start().await;
}
