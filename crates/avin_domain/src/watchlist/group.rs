// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use crate::InstrumentId;

/// A named group of instruments inside a [`Watchlist`](crate::Watchlist).
///
/// `WatchlistGroup` keeps instruments in their stored order and allows the
/// same instrument to appear more than once.
///
/// A newly created group is empty.
///
/// # Examples
///
/// ```
/// use avin_domain::WatchlistGroup;
///
/// let group = WatchlistGroup::new("shares");
///
/// assert_eq!(group.name(), "shares");
/// assert!(group.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct WatchlistGroup {
    name: String,
    instruments: Vec<InstrumentId>,
}

impl WatchlistGroup {
    /// Creates an empty watchlist group with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instruments: Vec::new(),
        }
    }

    /// Returns the group name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the instruments in their stored order.
    pub fn instruments(&self) -> &[InstrumentId] {
        &self.instruments
    }

    /// Returns whether the group contains no instruments.
    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    /// Returns the number of instruments in the group.
    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    /// Returns the instrument at the given index, if it exists.
    pub fn get(&self, n: usize) -> Option<&InstrumentId> {
        self.instruments.get(n)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_group_is_empty() {
        let group = WatchlistGroup::new("shares");

        assert_eq!(group.name(), "shares");
        assert!(group.is_empty());
        assert_eq!(group.len(), 0);
        assert!(group.instruments().is_empty());
        assert_eq!(group.get(0), None);
    }

    #[test]
    fn instruments_preserve_order_and_duplicates() {
        let sber = InstrumentId::from_str("MOEX.SHARE.SBER").unwrap();
        let gazp = InstrumentId::from_str("MOEX.SHARE.GAZP").unwrap();

        let group = WatchlistGroup {
            name: "shares".to_string(),
            instruments: vec![sber.clone(), gazp.clone(), sber.clone()],
        };

        assert!(!group.is_empty());
        assert_eq!(group.len(), 3);
        assert_eq!(
            group.instruments(),
            &[sber.clone(), gazp.clone(), sber.clone()]
        );

        assert_eq!(group.get(0), Some(&sber));
        assert_eq!(group.get(1), Some(&gazp));
        assert_eq!(group.get(2), Some(&sber));
        assert_eq!(group.get(3), None);
    }
}
