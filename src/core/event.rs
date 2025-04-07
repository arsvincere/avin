/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::bar::Bar;
use crate::core::timeframe::TimeFrame;

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Bar(BarEvent),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BarEvent {
    pub bar: Bar,
    pub tf: TimeFrame,
}
impl BarEvent {
    pub fn new(bar: Bar, tf: TimeFrame) -> Self {
        BarEvent { bar, tf }
    }
}
