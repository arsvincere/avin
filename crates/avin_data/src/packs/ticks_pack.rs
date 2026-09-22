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
    range: TimeRange,
    ticks: Vec<Tick>,
}

impl TicksPack {
    pub fn new(
        instrument: InstrumentInfo,
        range: TimeRange,
        ticks: Vec<Tick>,
    ) -> Result<Self, DataError> {
        // check sorted
        // check in range
        todo!();
    }

    pub fn instrument(&self) -> &InstrumentInfo {
        &self.instrument
    }

    pub fn range(&self) -> TimeRange {
        self.range
    }

    pub fn ticks(&self) -> &[Tick] {
        &self.ticks
    }
}
