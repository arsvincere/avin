#![allow(dead_code)]
#![allow(unused)]

use avin_analyse::*;
use avin_core::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;

fn main() {
    let tf = TimeFrame::Day;
    let begin = str_date_to_utc("2025-06-23");
    let end = str_date_to_utc("2025-07-29");
    let mut asset = Asset::new("moex_share_afks").unwrap();
    asset.load_chart_period(tf, begin, end).unwrap();
    let chart = asset.chart_mut(tf).unwrap();
    ExtremumIndicator::init(chart);
    TrendAnalytic::init(chart);
    let trends = chart.all_trend(Term::T1);
    for i in trends.iter() {
        println!("{i}");
    }
    println!("-----------------------------------");

    let iid = Manager::find_iid("moex_share_afks").unwrap();
    let mut simulator = Simulator::new(&iid, begin, end);
    simulator.activate(tf);
    let n = 24 * 60;
    let chart = simulator.asset_mut().chart_mut(tf).unwrap();
    ExtremumIndicator::init(chart);
    TrendAnalytic::init(chart);

    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);
    simulator.step(n);

    let chart = simulator.asset().chart(tf).unwrap();
    println!("{}", &chart.now().unwrap());
    let trends = chart.all_trend(Term::T1);
    for i in trends.iter() {
        println!("{i}");
    }
}

// Trend: +4.19 / 4 = 1.05 [vol: 202026600  2025-06-23 00:00:00 -> 2025-06-26 00:00:00]
// Trend: -2.54 / 2 = 1.27 [vol: 112197200  2025-06-26 00:00:00 -> 2025-06-27 00:00:00]
// Trend: +9.34 / 5 = 1.87 [vol: 357876900  2025-06-27 00:00:00 -> 2025-07-01 00:00:00]
// Trend: -2.12 / 2 = 1.06 [vol: 200090000  2025-07-01 00:00:00 -> 2025-07-02 00:00:00]
// Trend: +3.05 / 2 = 1.53 [vol: 147454200  2025-07-02 00:00:00 -> 2025-07-03 00:00:00]
// Trend: -3.31 / 2 = 1.65 [vol: 138290300  2025-07-03 00:00:00 -> 2025-07-04 00:00:00]
// Trend: +1 / 2 = 0.5 [vol: 53604200  2025-07-04 00:00:00 -> 2025-07-05 00:00:00]
// Trend: -1.03 / 3 = 0.34 [vol: 93016500  2025-07-05 00:00:00 -> 2025-07-07 00:00:00]
// Trend: +4.53 / 2 = 2.27 [vol: 301899000  2025-07-07 00:00:00 -> 2025-07-08 00:00:00]
// Trend: -5.09 / 2 = 2.55 [vol: 373274800  2025-07-08 00:00:00 -> 2025-07-09 00:00:00]
// Trend: +4.7 / 2 = 2.35 [vol: 243600700  2025-07-09 00:00:00 -> 2025-07-10 00:00:00]
// Trend: -6.9 / 2 = 3.45 [vol: 162818100  2025-07-10 00:00:00 -> 2025-07-11 00:00:00]
// Trend: +1.29 / 2 = 0.65 [vol: 85354400  2025-07-11 00:00:00 -> 2025-07-12 00:00:00]
// Trend: -2.4 / 3 = 0.8 [vol: 154968300  2025-07-12 00:00:00 -> 2025-07-14 00:00:00]
// Trend: +15.26 / 6 = 2.54 [vol: 725700200  2025-07-14 00:00:00 -> 2025-07-19 00:00:00]
// Trend: -4.04 / 4 = 1.01 [vol: 180621800  2025-07-19 00:00:00 -> 2025-07-22 00:00:00]
// Trend: +3.53 / 2 = 1.76 [vol: 154782600  2025-07-22 00:00:00 -> 2025-07-23 00:00:00]
// Trend: -5.76 / 3 = 1.92 [vol: 319623100  2025-07-23 00:00:00 -> 2025-07-25 00:00:00]
// Trend: +1.67 / 2 = 0.84 [vol: 190950800  2025-07-25 00:00:00 -> 2025-07-26 00:00:00]
//
