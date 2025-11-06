/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Chart;

use super::Condition;

const NAME: &str = "bull_3";

pub struct MyCondition {}
impl Condition for MyCondition {
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
