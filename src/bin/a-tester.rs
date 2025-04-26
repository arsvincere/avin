/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin::*;
use chrono::{TimeZone, Utc};

#[tokio::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let share = Share::from_str("moex_share_sber").unwrap();
    let mut test = Test::new("Every", share.iid());
    test.set_begin(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 0, 0).unwrap());
    test.set_end(&Utc.with_ymd_and_hms(2023, 8, 1, 7, 10, 0).unwrap());
    // test.set_begin(&Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());
    // test.set_end(&Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap());
    let mut tester = Tester::new();

    let t = utils::Timer::start();
    tester.run(&mut test).await;
    t.stop("");
}
