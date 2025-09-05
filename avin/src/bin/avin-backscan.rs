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
    let tf = TimeFrame::H1;
    asset.load_chart(tf).unwrap();
    let chart = asset.chart_mut(tf).unwrap();

    let filter = MyFilter::default();
    let marker = Marker::new(
        MarkerShape::Circle,
        MarkerColor::Yellow,
        MarkerSize::Small,
    );

    Scanner::scan(chart, filter, marker);
}

#[derive(Default)]
struct MyFilter {}
impl Filter for MyFilter {
    fn name(&self) -> &'static str {
        "my_filter"
    }
    fn apply(&self, chart: &Chart) -> bool {
        let trend = match chart.trend(Term::T1, 0) {
            Some(t) => t,
            None => return false,
        };
        if !trend.is_bear() {
            return false;
        }

        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if !(0.90..=0.95).contains(&cdf) {
            return false;
        }

        if trend.len() < 5 {
            return false;
        }

        true

        // let trend = match chart.trend(Term::T1, 0) {
        //     Some(t) => t,
        //     None => return false,
        // };
        // if trend.is_bear() {
        //     return false;
        // }
        //
        // let cdf = chart.trend_abs_cdf(trend).unwrap();
        // if cdf > 0.80 && cdf < 0.90 {
        //     return true;
        // }
        //
        // false
    }
}
// #[derive(Default)]
// struct MyFilter {}
// impl Filter for MyFilter {
//     fn name(&self) -> &'static str {
//         "my_filter"
//     }
//     fn apply(&self, chart: &Chart) -> bool {
//         let trend = match chart.trend(Term::T3, 0) {
//             Some(t) => t,
//             None => return false,
//         };
//         if trend.is_bear() {
//             return false;
//         }
//         let size = chart.trend_abs_size(trend).unwrap();
//         if size == Size::Biggest {
//             return true;
//         }
//
//         false
//     }
// }
