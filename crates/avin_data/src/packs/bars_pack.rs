// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::TimeRange;
use avin_domain::{Bar, InstrumentInfo, TimeFrame};

use crate::DataError;

/// A validated pack of historical bars for one instrument and timeframe.
///
/// The pack covers the specified [`TimeRange`] and may contain no bars.
pub struct BarsPack {
    instrument: InstrumentInfo,
    timeframe: TimeFrame,
    range: TimeRange,
    bars: Vec<Bar>,
}

impl BarsPack {
    /// Creates a pack of historical bars.
    ///
    /// Validates that bars:
    /// - are ordered by increasing time;
    /// - have unique times;
    /// - belong to `range`;
    /// - are aligned to the timeframe.
    ///
    /// # Errors
    ///
    /// Returns [`DataError::Pack`] if validation fails.
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

    /// Returns the instrument associated with the bars.
    pub fn instrument(&self) -> &InstrumentInfo {
        &self.instrument
    }

    /// Returns the bars timeframe.
    pub fn timeframe(&self) -> TimeFrame {
        self.timeframe
    }

    /// Returns the time range represented by the pack.
    pub fn range(&self) -> TimeRange {
        self.range
    }

    /// Returns all bars of pack.
    pub fn bars(&self) -> &[Bar] {
        &self.bars
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use avin_core::{Price, Quantity, Time};

    use super::*;

    fn instrument() -> InstrumentInfo {
        InstrumentInfo::new_unchecked(HashMap::new())
    }

    fn bar(time: &str) -> Bar {
        Bar::new_unchecked(
            Time::from_str(time).unwrap(),
            Price::new(10.0).unwrap(),
            Price::new(11.0).unwrap(),
            Price::new(9.0).unwrap(),
            Price::new(10.5).unwrap(),
            Quantity::new(100.0).unwrap(),
        )
    }

    fn range() -> TimeRange {
        TimeRange::new(
            Time::from_str("2026-01-01 10:00").unwrap(),
            Time::from_str("2026-01-01 11:00").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn valid() {
        let bars = vec![
            bar("2026-01-01 10:00"),
            bar("2026-01-01 10:01"),
            bar("2026-01-01 10:02"),
        ];
        let instrument = instrument();
        let tf = TimeFrame::M1;
        let range = range();

        let pack = BarsPack::new(instrument, tf, range, bars);

        assert!(pack.is_ok());
    }

    #[test]
    fn reject_unsorted() {
        let bars = vec![
            bar("2026-01-01 10:00"),
            bar("2026-01-01 10:02"),
            bar("2026-01-01 10:01"),
        ];
        let instrument = instrument();
        let tf = TimeFrame::M1;
        let range = range();

        let result = BarsPack::new(instrument, tf, range, bars);

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn reject_duplicates() {
        let bars = vec![
            bar("2026-01-01 10:00"),
            bar("2026-01-01 10:01"),
            bar("2026-01-01 10:01"),
        ];
        let instrument = instrument();
        let tf = TimeFrame::M1;
        let range = range();

        let result = BarsPack::new(instrument, tf, range, bars);

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn reject_outside_range() {
        let bars = vec![
            bar("2026-01-01 09:59"),
            bar("2026-01-01 10:00"),
            bar("2026-01-01 10:01"),
        ];
        let instrument = instrument();
        let tf = TimeFrame::M1;
        let range = range();

        let result = BarsPack::new(instrument, tf, range, bars);

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }

    #[test]
    fn reject_misaligned_time() {
        let bars = vec![
            bar("2026-01-01 10:00"),
            bar("2026-01-01 10:01:30"),
            bar("2026-01-01 10:02"),
        ];
        let instrument = instrument();
        let tf = TimeFrame::M1;
        let range = range();

        let result = BarsPack::new(instrument, tf, range, bars);

        assert!(matches!(result, Err(DataError::Pack { .. })));
    }
}
