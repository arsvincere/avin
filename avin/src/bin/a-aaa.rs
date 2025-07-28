use avin_core::*;
use avin_strategy::*;
use avin_utils::*;

fn main() {
    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let begin = str_date_to_utc("2023-08-01");
    let end = str_date_to_utc("2023-09-01");
    let tf = TimeFrame::M1;

    let mut simulator = Simulator::new(&iid, begin, end);
    simulator.activate(tf);

    let chart = simulator.asset().chart(tf).unwrap();
    assert!(chart.now().is_none());

    simulator.next_bar();
    let chart = simulator.asset().chart(tf).unwrap();
    assert!(chart.now().is_some());

    for _i in 0..10 {
        simulator.next_bar();
    }

    let chart = simulator.asset().chart(tf).unwrap();
    let now_bar = chart.now().unwrap();
    let expect_dt = str_dt_to_utc("2023-08-01 10:09:00");
    let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
    let expect_bar =
        Bar::new(expect_ts, 267.52, 267.61, 266.84, 267.07, 1304400);
    assert_eq!(*now_bar, expect_bar);
}
