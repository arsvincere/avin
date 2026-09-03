// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use avin_core::Time;
use chrono::{DateTime, Utc};

use crate::{BarDirection, PriceRange};

/// An OHLCV market bar.
///
/// Bar values are assumed to contain valid market data; external data
/// must be validated before constructing bars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bar {
    /// Bar start time.
    pub time: Time,
    /// Open price.
    pub o: f64,
    /// High price.
    pub h: f64,
    /// Low price.
    pub l: f64,
    /// Close price.
    pub c: f64,
    /// Volume.
    pub v: u64,
}

impl Bar {
    /// Creates a bar from trusted OHLCV values without validation.
    pub fn new(time: Time, o: f64, h: f64, l: f64, c: f64, v: u64) -> Bar {
        Bar {
            time,
            o,
            h,
            l,
            c,
            v,
        }
    }

    /// Returns the bar start timestamp as a UTC datetime.
    pub fn dt(&self) -> DateTime<Utc> {
        self.time.dt()
    }

    /// Returns the bar direction.
    pub fn direction(&self) -> BarDirection {
        if self.is_bull() {
            BarDirection::Bull
        } else if self.is_bear() {
            BarDirection::Bear
        } else {
            BarDirection::Neutral
        }
    }

    /// Returns whether the bar is bearish.
    pub fn is_bear(&self) -> bool {
        self.o > self.c
    }

    /// Returns whether the bar is bullish.
    pub fn is_bull(&self) -> bool {
        self.o < self.c
    }

    /// Returns whether the bar is neutral.
    pub fn is_neutral(&self) -> bool {
        self.o == self.c
    }

    /// Returns the full price range of the bar: `[L, H]`.
    ///
    /// # Panics
    ///
    /// Panics if the price range cannot be constructed from the bar values.
    pub fn range(&self) -> PriceRange {
        PriceRange::new(self.l, self.h).unwrap()
    }

    /// Returns the body price range: `[min(O, C), max(O, C)]`.
    ///
    /// # Panics
    ///
    /// Panics if the price range cannot be constructed from the bar values.
    pub fn body(&self) -> PriceRange {
        PriceRange::new(self.o.min(self.c), self.o.max(self.c)).unwrap()
    }

    /// Returns the lower wick price range: `[L, min(O, C)]`.
    ///
    /// # Panics
    ///
    /// Panics if the price range cannot be constructed from the bar values.
    pub fn lower(&self) -> PriceRange {
        PriceRange::new(self.l, self.o.min(self.c)).unwrap()
    }

    /// Returns the upper wick price range: `[max(O, C), H]`.
    ///
    /// # Panics
    ///
    /// Panics if the price range cannot be constructed from the bar values.
    pub fn upper(&self) -> PriceRange {
        PriceRange::new(self.o.max(self.c), self.h).unwrap()
    }

    /// Returns whether the closed price range `[L, H]` contains the given
    /// price.
    pub fn contains(&self, price: f64) -> bool {
        self.l <= price && price <= self.h
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} O={} H={} L={} C={} V={}",
            self.time, self.o, self.h, self.l, self.c, self.v
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_and_dt() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let bar = Bar::new(time, 10.0, 11.1, 9.9, 10.5, 5000);

        assert_eq!(bar.time, time);
        assert_eq!(bar.o, 10.0);
        assert_eq!(bar.h, 11.1);
        assert_eq!(bar.l, 9.9);
        assert_eq!(bar.c, 10.5);
        assert_eq!(bar.v, 5000);

        let dt = time.dt();
        assert_eq!(bar.dt(), dt);
    }

    #[test]
    fn direction() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let vol = 5000;

        let bull_bar = Bar::new(time, 10.0, 11.1, 9.9, 10.5, vol);
        assert!(bull_bar.is_bull());
        assert!(!bull_bar.is_bear());
        assert!(!bull_bar.is_neutral());
        assert_eq!(bull_bar.direction(), BarDirection::Bull);

        let bear_bar = Bar::new(time, 10.0, 11.1, 9.9, 9.5, vol);
        assert!(!bear_bar.is_bull());
        assert!(bear_bar.is_bear());
        assert!(!bear_bar.is_neutral());
        assert_eq!(bear_bar.direction(), BarDirection::Bear);

        let neutral_bar = Bar::new(time, 10.0, 11.1, 9.9, 10.0, vol);
        assert!(!neutral_bar.is_bull());
        assert!(!neutral_bar.is_bear());
        assert!(neutral_bar.is_neutral());
        assert_eq!(neutral_bar.direction(), BarDirection::Neutral);
    }

    #[test]
    fn ranges() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let vol = 5000;

        let bull = Bar::new(time, 10.0, 11.1, 9.9, 10.5, vol);
        assert_eq!(bull.range(), PriceRange::new(9.9, 11.1).unwrap());
        assert_eq!(bull.body(), PriceRange::new(10.0, 10.5).unwrap());
        assert_eq!(bull.lower(), PriceRange::new(9.9, 10.0).unwrap());
        assert_eq!(bull.upper(), PriceRange::new(10.5, 11.1).unwrap());

        let bear = Bar::new(time, 10.0, 11.1, 9.4, 9.5, vol);
        assert_eq!(bear.range(), PriceRange::new(9.4, 11.1).unwrap());
        assert_eq!(bear.body(), PriceRange::new(9.5, 10.0).unwrap());
        assert_eq!(bear.lower(), PriceRange::new(9.4, 9.5).unwrap());
        assert_eq!(bear.upper(), PriceRange::new(10.0, 11.1).unwrap());

        let neutral = Bar::new(time, 10.0, 11.1, 9.9, 10.0, vol);
        assert_eq!(neutral.range(), PriceRange::new(9.9, 11.1).unwrap());
        assert_eq!(neutral.body(), PriceRange::new(10.0, 10.0).unwrap());
        assert_eq!(neutral.lower(), PriceRange::new(9.9, 10.0).unwrap());
        assert_eq!(neutral.upper(), PriceRange::new(10.0, 11.1).unwrap());
    }

    #[test]
    fn contains() {
        let bar = Bar::new(Time::new(123), 10.0, 11.1, 9.9, 10.5, 5000);

        assert!(bar.contains(10.3));
        assert!(bar.contains(9.9));
        assert!(bar.contains(11.1));

        assert!(!bar.contains(11.11));
        assert!(!bar.contains(9.89));
    }

    #[test]
    fn display() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let bar = Bar::new(time, 10.0, 11.1, 9.9, 10.5, 5000);

        assert_eq!(
            bar.to_string(),
            "2026-08-20 14:20:05 O=10 H=11.1 L=9.9 C=10.5 V=5000"
        );
    }
}
