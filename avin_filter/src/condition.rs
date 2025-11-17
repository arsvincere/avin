/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::Chart;

pub trait Condition {
    fn name(&self) -> &'static str;
    fn apply(&self, chart: &Chart) -> bool;
}
