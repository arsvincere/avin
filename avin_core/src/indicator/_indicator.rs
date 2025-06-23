/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Bar;

use super::extremum::ExtremumData;

#[derive(Debug)]
pub enum Indicator {
    Extremum(ExtremumData),
    // MA(MAData),
    // EMA(EMAData),
    // RSI(RSIData),
}
impl Indicator {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Extremum(i) => i.id(),
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Extremum(i) => i.name(),
        }
    }
    pub fn init(&mut self, bars: &[Bar], now: Option<&Bar>) {
        match self {
            Self::Extremum(i) => i.init(bars, now),
        }
    }
    pub fn update(&mut self, bars: &[Bar], now: Option<&Bar>) {
        match self {
            Self::Extremum(i) => i.update(bars, now),
        }
    }
}
