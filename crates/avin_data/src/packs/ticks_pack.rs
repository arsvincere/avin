// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use avin_core::TimeRange;
use avin_domain::{InstrumentInfo, Tick, TimeFrame};

use crate::DataError;

pub struct TicksPack {
    instrument: InstrumentInfo,
    timeframe: TimeFrame,
    range: TimeRange,
    bars: Vec<Tick>,
}

impl TicksPack {
    pub fn new(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
        bars: Vec<Tick>,
    ) -> Result<Self, DataError> {
        // check sorted
        // check in range
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

    pub fn bars(&self) -> &[Tick] {
        &self.bars
    }
}
