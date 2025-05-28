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

    let share = Share::new("MOEX_SHARE_SBER").unwrap();
    let mut test = Test::new("Every", share.iid());
    test.set_begin(&Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap());
    test.set_end(&Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap());
    let mut tester = Tester::new();

    let t = utils::Timer::new();
    tester.run(&mut test).await;
    t.stop("");
}
