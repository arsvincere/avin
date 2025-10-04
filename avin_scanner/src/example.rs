/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Chart;

use super::Filter;

const NAME: &str = "bull_3";

pub struct MyFilter {}
impl Filter for MyFilter {
    fn name(&self) -> &'static str {
        NAME
    }
    fn apply(&self, chart: &Chart) -> bool {
        let b2 = match chart.bar(2) {
            Some(bar) => bar,
            None => return false,
        };
        let b1 = chart.bar(1).unwrap();
        let b0 = chart.bar(0).unwrap();

        b2.is_bull() && b1.is_bull() && b0.is_bull()
    }
}
