#![allow(dead_code)]
#![allow(unused)]

use chrono::prelude::*;

use avin::analyse::*;
use avin::connect::*;
use avin::core::*;
use avin::scanner::*;
use avin::strategy::*;
use avin::tester::*;
use avin::utils;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let mut asset = Asset::new("moex_share_sber").unwrap();
    let tf = TimeFrame::M1;
    asset.load_chart(tf).unwrap();
    let chart = asset.chart_mut(tf).unwrap();

    let filter = MyFilter::default();
    let marker = Marker::new(
        MarkerShape::Asterisk,
        MarkerColor::Yellow,
        MarkerSize::S,
    );

    Scanner::scan(chart, filter, marker);
}

#[derive(Default)]
struct MyFilter {}
impl Filter for MyFilter {
    fn name(&self) -> &'static str {
        "bull_cdf_abs"
    }
    fn apply(&self, chart: &Chart) -> bool {
        let b2 = match chart.bar(2) {
            Some(bar) => bar,
            None => return false,
        };
        let b1 = chart.bar(1).unwrap();
        let b0 = chart.bar(0).unwrap();

        b2.is_bull() && b1.is_bull() && b0.is_bull()

        // // получаем текущий тренд, если нет возвращаем false
        // let trend = match chart.trend(Term::T1, 0) {
        //     Some(t) => t,
        //     None => return false,
        // };
        //
        // if trend.is_bear() {
        //     return false;
        // }
        //
        // // получаем cdf дельты тренда
        // let cdf = chart.trend_abs_cdf(trend).unwrap();
        //
        // cdf >= 0.60
    }
}
