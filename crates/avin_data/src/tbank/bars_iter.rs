// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use avin_core::TimeRange;
use avin_domain::{InstrumentInfo, TimeFrame};

use crate::DataError;

pub(super) struct TBankBarsIterator {}

impl TBankBarsIterator {
    pub(super) fn new(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
    ) -> Result<Self, DataError> {
        todo!();
    }
}
