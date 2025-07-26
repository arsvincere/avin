/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Bar, TimeFrame};

/// That event sending from broker when bar updated, or new historical bar.
///
/// # ru
/// Это событие отправляется брокером когда бар обновлен, или сформировался
/// новый исторический бар.
///
/// Содержит FIGI инструмента, таймфрейм и собственно бар.
#[derive(Debug, Clone)]
pub struct BarEvent {
    pub figi: String,
    pub tf: TimeFrame,
    pub bar: Bar,
}
impl BarEvent {
    pub fn new(figi: String, tf: TimeFrame, bar: Bar) -> Self {
        Self { figi, tf, bar }
    }
}
impl std::fmt::Display for BarEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BarEvent={} {} {}", self.figi, self.tf, self.bar)
    }
}
