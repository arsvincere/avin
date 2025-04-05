/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 *                                                                         *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use avin::*;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let dt = Utc::now();
    let ts = dt.timestamp();
    let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
    assert_eq!(b.dt(), dt);
    assert_eq!(b.o, 10.0);
    assert_eq!(b.h, 11.1);
    assert_eq!(b.l, 9.9);
    assert_eq!(b.c, 10.5);
    assert_eq!(b.v, 5000);
}
