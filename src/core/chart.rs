use crate::core::bar::Bar;
use crate::core::timeframe::TimeFrame;

#[derive(Debug)]
pub struct Chart {
    pub tf: TimeFrame,
    pub bars: Vec<Bar>,
    pub now: Bar,
}
