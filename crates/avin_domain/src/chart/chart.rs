// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::Time;
use avin_utils::AvinError;

use crate::{Bar, InstrumentId, Ticker, TimeFrame};

/// Mutable candlestick chart for one instrument and one timeframe.
///
/// Chart stores bars ordered by increasing time.
/// The last bar is the most recent bar and may be unfinished in realtime.
///
/// # Invariants
///
/// - bars are ordered by increasing `time`;
/// - bars have unique `time`;
/// - every `Bar::time` is the beginning of its timeframe frame;
///
/// The constructor accepts trusted bars. Historical validation belongs to
/// storage/service, not to `Chart`.
pub struct Chart {
    iid: InstrumentId,
    tf: TimeFrame,
    bars: Vec<Bar>,
}
impl Chart {
    /// Creates a chart from trusted bars.
    ///
    /// Bars are expected to satisfy all `Chart` invariants.
    pub fn new(iid: InstrumentId, tf: TimeFrame, bars: Vec<Bar>) -> Self {
        Self { iid, tf, bars }
    }

    /// Creates an empty chart.
    ///
    /// Used by backtester for initial state.
    pub fn empty(iid: InstrumentId, tf: TimeFrame) -> Self {
        Self::new(iid, tf, Vec::new())
    }

    /// Returns the instrument ID.
    pub fn iid(&self) -> &InstrumentId {
        &self.iid
    }

    /// Returns the instrument ticker.
    pub fn ticker(&self) -> &Ticker {
        self.iid.ticker()
    }

    /// Returns the chart timeframe.
    pub fn tf(&self) -> TimeFrame {
        self.tf
    }

    /// Returns whether the chart contains no bars.
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    /// Returns all chart bars ordered from oldest to newest.
    ///
    /// The last bar is current and may be unfinished in realtime.
    pub fn bars(&self) -> &[Bar] {
        &self.bars
    }

    /// Returns a bar by Python-style index.
    ///
    /// Non-negative indexes count from the beginning:
    /// `0` is the first bar.
    ///
    /// Negative indexes count from the end:
    /// `-1` is the last bar, `-2` is the previous bar.
    ///
    /// Returns `None` when the index is out of bounds.
    ///
    /// ```text
    /// len = 5
    ///
    /// index:   0   1   2   3   4
    ///         bar bar bar bar bar
    /// index:  -5  -4  -3  -2  -1
    /// ```
    pub fn bar(&self, index: isize) -> Option<&Bar> {
        if index >= 0 {
            self.bars.get(index as usize)
        } else {
            let n = index.unsigned_abs();

            if n > self.bars.len() {
                None
            } else {
                self.bars.get(self.bars.len() - n)
            }
        }
    }

    /// Returns the first bar.
    pub fn first(&self) -> Option<&Bar> {
        self.bars.first()
    }

    /// Returns the last bar.
    ///
    /// The last bar is the most recent and may be unfinished in realtime.
    pub fn last(&self) -> Option<&Bar> {
        self.bars.last()
    }

    /// Returns the close price of the last bar.
    pub fn last_price(&self) -> Option<f64> {
        Some(self.last()?.c)
    }

    /// Selects bars in the closed interval `[from, till]`.
    ///
    /// Returns a borrowed slice without copying bars.
    ///
    /// # Errors
    ///
    /// Returns an error if `from > till`.
    pub fn select(
        &self,
        from: Time,
        till: Time,
    ) -> Result<&[Bar], AvinError> {
        if from > till {
            return Err(AvinError::Value(
                "Chart select from > till".to_string(),
            ));
        }

        let left = self.bars.partition_point(|bar| bar.time < from);
        let right = self.bars.partition_point(|bar| bar.time <= till);

        Ok(&self.bars[left..right])
    }

    /// Inserts a bar or replaces an existing bar with the same time.
    ///
    /// Maintains bars ordered by increasing time.
    pub fn upsert(&mut self, bar: Bar) {
        // 1. The chart is empty — add the first bar.
        if self.bars.is_empty() {
            self.bars.push(bar);
            return;
        }

        // 2. The bar is newer than the last one — append it.
        if bar.time > self.bars.last().unwrap().time {
            self.bars.push(bar);
            return;
        }

        // 3. Find the bar position by time.
        let index = self.bars.partition_point(|b| b.time < bar.time);

        // 4. A bar with the same time exists — replace it.
        if index < self.bars.len() && self.bars[index].time == bar.time {
            self.bars[index] = bar;
            return;
        }

        // 5. No bar with the same time — insert it at the
        // correct position.
        self.bars.insert(index, bar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Category, Exchange};

    const NANOS_PER_SECOND: i64 = 1_000_000_000;

    fn iid() -> InstrumentId {
        InstrumentId::new(
            Exchange::Moex,
            Category::Share,
            Ticker::new("SBER").unwrap(),
        )
    }

    fn bar(n: i64) -> Bar {
        let time = Time::new(n * NANOS_PER_SECOND);
        let price = (n + 1) as f64;

        Bar::new(time, price, price, price, price, (n + 1) as u64 * 100)
    }

    fn chart() -> Chart {
        Chart::new(iid(), TimeFrame::S1, (0_i64..5).map(bar).collect())
    }

    #[test]
    fn new() {
        let chart = chart();

        assert_eq!(chart.iid(), &iid());
        assert_eq!(chart.ticker(), &Ticker::new("SBER").unwrap());
        assert_eq!(chart.tf(), TimeFrame::S1);
        assert!(!chart.is_empty());
        assert_eq!(chart.bars().len(), 5);
        assert_eq!(chart.first(), Some(&bar(0)));
        assert_eq!(chart.last(), Some(&bar(4)));
        assert_eq!(chart.last_price(), Some(5.0));
    }

    #[test]
    fn empty() {
        let chart = Chart::empty(iid(), TimeFrame::S1);

        assert!(chart.is_empty());
        assert!(chart.bars().is_empty());
        assert_eq!(chart.first(), None);
        assert_eq!(chart.last(), None);
        assert_eq!(chart.last_price(), None);
    }

    #[test]
    fn bar_index() {
        let chart = chart();

        assert_eq!(chart.bar(0), Some(&bar(0)));
        assert_eq!(chart.bar(1), Some(&bar(1)));
        assert_eq!(chart.bar(4), Some(&bar(4)));

        assert_eq!(chart.bar(-1), Some(&bar(4)));
        assert_eq!(chart.bar(-2), Some(&bar(3)));
        assert_eq!(chart.bar(-5), Some(&bar(0)));

        assert_eq!(chart.bar(5), None);
        assert_eq!(chart.bar(-6), None);
    }

    #[test]
    fn select() {
        let chart = chart();

        let selected = chart
            .select(
                Time::new(NANOS_PER_SECOND),
                Time::new(3 * NANOS_PER_SECOND),
            )
            .unwrap();

        assert_eq!(selected, &[bar(1), bar(2), bar(3)]);

        let selected = chart
            .select(
                Time::new(NANOS_PER_SECOND + 1),
                Time::new(4 * NANOS_PER_SECOND - 1),
            )
            .unwrap();

        assert_eq!(selected, &[bar(2), bar(3)]);

        let selected = chart
            .select(
                Time::new(5 * NANOS_PER_SECOND),
                Time::new(6 * NANOS_PER_SECOND),
            )
            .unwrap();

        assert!(selected.is_empty());
    }

    #[test]
    fn select_invalid_range() {
        let chart = chart();

        assert!(
            chart
                .select(Time::new(NANOS_PER_SECOND), Time::new(0))
                .is_err()
        );
    }

    #[test]
    fn upsert() {
        let mut chart = Chart::empty(iid(), TimeFrame::S1);

        // Empty chart.
        chart.upsert(bar(2));
        assert_eq!(chart.bars(), &[bar(2)]);

        // Append.
        chart.upsert(bar(4));
        assert_eq!(chart.bars(), &[bar(2), bar(4)]);

        // Insert in the middle.
        chart.upsert(bar(3));
        assert_eq!(chart.bars(), &[bar(2), bar(3), bar(4)]);

        // Insert at the beginning.
        chart.upsert(bar(1));
        assert_eq!(chart.bars(), &[bar(1), bar(2), bar(3), bar(4)]);

        // Replace.
        let time = Time::new(3 * NANOS_PER_SECOND);
        let replacement = Bar::new(time, 30.0, 31.0, 29.0, 30.5, 999);

        chart.upsert(replacement);

        assert_eq!(chart.bars().len(), 4);
        assert_eq!(chart.bar(2), Some(&replacement));
    }
}
