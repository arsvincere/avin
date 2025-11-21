#![allow(unused)]

use chrono::prelude::*;

use avin_analyse::*;
use avin_connect::*;
use avin_core::*;
use avin_search::*;
use avin_strategy::*;
use avin_tester::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let mut asset = Asset::new("moex_share_vtbr").unwrap();
    let source = Source::MOEXALGO;
    let tf = TimeFrame::M10;
    let begin = avin_utils::str_date_to_utc("2024-01-01");
    let end = avin_utils::str_date_to_utc("2025-01-01");
    asset.load_chart_period(source, tf, begin, end).unwrap();
    let chart = asset.chart_mut(tf).unwrap();

    let condition = MyCondition::default();
    let marker = Marker::new(
        MarkerShape::Circle,
        MarkerColor::Yellow,
        MarkerSize::Small,
    );

    Filter::run(chart, condition, marker);
}

#[derive(Default)]
struct MyCondition {}
impl Condition for MyCondition {
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

        let cdf = chart.trend_speed_cdf(trend).unwrap();
        if !(0.80..=0.90).contains(&cdf) {
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
