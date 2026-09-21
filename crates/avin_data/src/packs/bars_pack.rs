// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use avin_core::TimeRange;
use avin_domain::{Bar, InstrumentInfo, TimeFrame};

use crate::DataError;

pub struct BarsPack {
    instrument: InstrumentInfo,
    timeframe: TimeFrame,
    range: TimeRange,
    bars: Vec<Bar>,
}

impl BarsPack {
    pub fn new(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
        bars: Vec<Bar>,
    ) -> Result<Self, DataError> {
        // check sorted
        // check duplicates
        // check in range
        // check ts ??? (что соответствуют таймфрейму)
        todo!();
    }

    pub fn instrument(&self) -> &InstrumentInfo {
        &self.instrument
    }

    pub fn timeframe(&self) -> TimeFrame {
        self.timeframe
    }

    pub fn range(&self) -> TimeRange {
        self.range
    }

    pub fn bars(&self) -> &[Bar] {
        &self.bars
    }
}
