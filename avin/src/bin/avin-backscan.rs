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
    let tf = TimeFrame::M10;
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
        if trend.is_bear() {
            return false;
        }
        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if cdf < 0.80 {
            return false;
        }

        let trend = match chart.trend(Term::T2, 0) {
            Some(t) => t,
            None => return false,
        };
        if trend.is_bear() {
            return false;
        }
        let cdf = chart.trend_abs_cdf(trend).unwrap();
        if cdf < 0.60 {
            return false;
        }

        true
    }
}
