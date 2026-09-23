// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

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
        // 1. Bars must be sorted by time.
        if !bars.is_sorted_by(|a, b| a.time <= b.time) {
            let msg = "bars must be sorted by time";
            return Err(DataError::pack(msg, None));
        }

        // 2. Bars must not contain duplicate times.
        if bars.windows(2).any(|pair| pair[0].time == pair[1].time) {
            let msg = "bars contain duplicate times";
            return Err(DataError::pack(msg, None));
        }

        // 3. All bars must be inside the pack range.
        if let (Some(first), Some(last)) = (bars.first(), bars.last()) {
            let first = first.time;
            let last = last.time;
            if !range.contains(first) || !range.contains(last) {
                let msg = format!("bars are outside pack range {range}");
                return Err(DataError::pack(msg, None));
            }
        }

        // 4. Every bar time must be aligned to the timeframe.
        if bars.iter().any(|bar| tf.begin_frame(bar.time) != bar.time) {
            let msg = format!("bars are not aligned to timeframe {tf}");
            return Err(DataError::pack(msg, None));
        }

        Ok(Self {
            instrument,
            timeframe: tf,
            range,
            bars,
        })
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
