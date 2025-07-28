use avin_core::*;
use avin_strategy::*;
use avin_utils::*;

fn main() {
    let iid = Manager::find_iid("moex_share_sber").unwrap();
    let begin = str_date_to_utc("2023-08-01");
    let end = str_date_to_utc("2023-09-01");
    let tf = TimeFrame::H1;

    let mut simulator = Simulator::new(&iid, begin, end);
    simulator.activate(tf);

    simulator.next_bar();
    simulator.next_bar();
    let chart = simulator.asset().chart(tf).unwrap();
    println!("{}", chart.now().unwrap());

    simulator.step(120);
    let chart = simulator.asset().chart(tf).unwrap();
    println!("{}", chart.now().unwrap());

    let chart = simulator.asset().chart(tf).unwrap();
    let now_bar = chart.now().unwrap();
    let expect_dt = str_dt_to_utc("2023-08-01 12:00:00");
    let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
    let expect_bar =
        Bar::new(expect_ts, 267.89, 268.39, 267.83, 268.31, 245660);
    println!("{now_bar}");
    println!("{expect_bar}");
    assert_eq!(*now_bar, expect_bar);
}
