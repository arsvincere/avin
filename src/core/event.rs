/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::bar::Bar;
use crate::core::tic::Tic;
use crate::core::timeframe::TimeFrame;

#[derive(Debug, Clone)]
pub enum Event {
    Bar(BarEvent),
    Tic(TicEvent),
}

impl Event {
    pub fn figi(&self) -> &String {
        match self {
            Self::Bar(e) => &e.figi,
            Self::Tic(e) => &e.figi,
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Event::Bar(e) => write!(f, "Event={}", e),
            Event::Tic(e) => write!(f, "Event={}", e),
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct TicEvent {
    pub figi: String,
    pub tic: Tic,
}
impl TicEvent {
    pub fn new(figi: String, tic: Tic) -> Self {
        Self { figi, tic }
    }
}
impl std::fmt::Display for TicEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TicEvent={} {}", self.figi, self.tic)
    }
}
