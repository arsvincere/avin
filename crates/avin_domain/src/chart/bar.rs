// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use avin_core::{Price, PriceRange, Time};
use chrono::{DateTime, Utc};

use crate::{BarDirection, DomainError};

/// An OHLCV market bar.
///
/// Bar values are assumed to contain valid market data; bar-level invariants
/// must be validated before constructing bars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bar {
    /// Bar start time.
    pub time: Time,
    /// Open price.
    pub o: Price,
    /// High price.
    pub h: Price,
    /// Low price.
    pub l: Price,
    /// Close price.
    pub c: Price,
    /// Volume.
    pub v: u64,
}

impl Bar {
    /// Creates a bar from trusted OHLCV values without validation.
    pub fn new(
        time: Time,
        o: Price,
        h: Price,
        l: Price,
        c: Price,
        v: u64,
    ) -> Result<Self, DomainError> {
        if o < l || o > h {
            return Err(DomainError::Bar(format!(
                "open price {o} is outside bar range [{l}, {h}]"
            )));
        }

        if c < l || c > h {
            return Err(DomainError::Bar(format!(
                "close price {c} is outside bar range [{l}, {h}]"
            )));
        }

        Ok(Self::new_unchecked(time, o, h, l, c, v))
    }

    // TODO: вынести в "приватный трейт"
    pub fn new_unchecked(
        time: Time,
        o: Price,
        h: Price,
        l: Price,
        c: Price,
        v: u64,
    ) -> Bar {
        Bar {
            time,
            o,
            h,
            l,
            c,
            v,
        }
    }

    /// Returns the bar start time as a UTC datetime.
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
    pub fn contains(&self, price: Price) -> bool {
        self.range().contains(price)
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
        let open = Price::new(10.0).unwrap();
        let high = Price::new(11.0).unwrap();
        let low = Price::new(9.0).unwrap();
        let close = Price::new(10.5).unwrap();
        let vol = 5000;
        let bar = Bar::new(time, open, high, low, close, vol).unwrap();

        assert_eq!(bar.time, time);
        assert_eq!(bar.o.value(), 10.0);
        assert_eq!(bar.h.value(), 11.0);
        assert_eq!(bar.l.value(), 9.0);
        assert_eq!(bar.c.value(), 10.5);
        assert_eq!(bar.v, 5000);

        let dt = time.dt();
        assert_eq!(bar.dt(), dt);
    }

    #[test]
    fn direction() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let open = Price::new(10.0).unwrap();
        let high = Price::new(11.0).unwrap();
        let low = Price::new(9.0).unwrap();
        let close = Price::new(10.5).unwrap();
        let vol = 5000;

        let bull = Bar::new(time, open, high, low, close, vol).unwrap();
        assert!(bull.is_bull());
        assert!(!bull.is_bear());
        assert!(!bull.is_neutral());
        assert_eq!(bull.direction(), BarDirection::Bull);

        let close = Price::new(9.5).unwrap();
        let bear = Bar::new(time, open, high, low, close, vol).unwrap();
        assert!(!bear.is_bull());
        assert!(bear.is_bear());
        assert!(!bear.is_neutral());
        assert_eq!(bear.direction(), BarDirection::Bear);

        let close = Price::new(10.0).unwrap();
        let neutral = Bar::new(time, open, high, low, close, vol).unwrap();
        assert!(!neutral.is_bull());
        assert!(!neutral.is_bear());
        assert!(neutral.is_neutral());
        assert_eq!(neutral.direction(), BarDirection::Neutral);
    }

    #[test]
    fn ranges() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let open = Price::new(10.0).unwrap();
        let high = Price::new(11.0).unwrap();
        let low = Price::new(9.0).unwrap();
        let close = Price::new(10.5).unwrap();
        let vol = 5000;

        let bull = Bar::new(time, open, high, low, close, vol).unwrap();
        assert_eq!(bull.range(), PriceRange::new(low, high).unwrap());
        assert_eq!(bull.body(), PriceRange::new(open, close).unwrap());
        assert_eq!(bull.lower(), PriceRange::new(low, open).unwrap());
        assert_eq!(bull.upper(), PriceRange::new(close, high).unwrap());

        let close = Price::new(9.5).unwrap();
        let bear = Bar::new(time, open, high, low, close, vol).unwrap();
        assert_eq!(bear.range(), PriceRange::new(low, high).unwrap());
        assert_eq!(bear.body(), PriceRange::new(close, open).unwrap());
        assert_eq!(bear.lower(), PriceRange::new(low, close).unwrap());
        assert_eq!(bear.upper(), PriceRange::new(open, high).unwrap());

        let close = Price::new(10.0).unwrap();
        let neutral = Bar::new(time, open, high, low, close, vol).unwrap();
        assert_eq!(neutral.range(), PriceRange::new(low, high).unwrap());
        assert_eq!(neutral.body(), PriceRange::new(open, close).unwrap());
        assert_eq!(neutral.body(), PriceRange::new(close, open).unwrap());
        assert_eq!(neutral.lower(), PriceRange::new(low, open).unwrap());
        assert_eq!(neutral.lower(), PriceRange::new(low, close).unwrap());
        assert_eq!(neutral.upper(), PriceRange::new(open, high).unwrap());
        assert_eq!(neutral.upper(), PriceRange::new(close, high).unwrap());
    }

    #[test]
    fn contains() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let open = Price::new(10.0).unwrap();
        let high = Price::new(11.0).unwrap();
        let low = Price::new(9.0).unwrap();
        let close = Price::new(10.5).unwrap();
        let vol = 5000;

        let bar = Bar::new(time, open, high, low, close, vol).unwrap();

        assert!(bar.contains(Price::new(10.3).unwrap()));
        assert!(bar.contains(Price::new(9.0).unwrap()));
        assert!(bar.contains(Price::new(11.0).unwrap()));

        assert!(!bar.contains(Price::new(11.01).unwrap()));
        assert!(!bar.contains(Price::new(8.99).unwrap()));
    }

    #[test]
    fn display() {
        let time = Time::from_str("2026-08-20 14:20:05").unwrap();
        let open = Price::new(10.0).unwrap();
        let high = Price::new(11.0).unwrap();
        let low = Price::new(9.0).unwrap();
        let close = Price::new(10.5).unwrap();
        let vol = 5000;

        let bar = Bar::new(time, open, high, low, close, vol).unwrap();

        assert_eq!(
            bar.to_string(),
            "2026-08-20 14:20:05 O=10 H=11 L=9 C=10.5 V=5000"
        );
    }
}
