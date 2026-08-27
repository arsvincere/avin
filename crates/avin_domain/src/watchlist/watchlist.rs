// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use crate::WatchlistItem;

#[derive(Debug)]
pub struct Watchlist {
    name: String,
    items: Vec<WatchlistItem>,
}

/// An ordered collection of instruments and instrument groups.
///
/// A watchlist contains top-level [`WatchlistItem`] entries in their stored
/// order. Items may be individual instruments or named instrument groups.
///
/// Duplicate instruments and groups are allowed.
///
/// A newly created watchlist is empty.
///
/// # Examples
///
/// ```
/// use avin_domain::Watchlist;
///
/// let watchlist = Watchlist::new("My favorite");
///
/// assert_eq!(watchlist.name(), "My favorite");
/// assert!(watchlist.is_empty());
/// ```
impl Watchlist {
    /// Creates an empty watchlist with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    /// Returns the watchlist name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the top-level items in their stored order.
    pub fn items(&self) -> &[WatchlistItem] {
        &self.items
    }

    /// Returns whether the watchlist contains no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of top-level items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns the item at the given index, if it exists.
    pub fn get(&self, n: usize) -> Option<&WatchlistItem> {
        self.items.get(n)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{InstrumentId, WatchlistGroup};

    use super::*;

    #[test]
    fn new_watchlist_is_empty() {
        let watchlist = Watchlist::new("test_name");

        assert_eq!(watchlist.name(), "test_name");
        assert!(watchlist.is_empty());
        assert_eq!(watchlist.len(), 0);
        assert!(watchlist.items().is_empty());
        assert!(watchlist.get(0).is_none());
    }

    #[test]
    fn items_preserve_order_and_duplicates() {
        let sber = InstrumentId::from_str("MOEX.SHARE.SBER").unwrap();
        let gazp = InstrumentId::from_str("MOEX.SHARE.GAZP").unwrap();

        let watchlist = Watchlist {
            name: "test_name".to_string(),
            items: vec![
                WatchlistItem::Instrument(sber.clone()),
                WatchlistItem::Group(WatchlistGroup::new("futures")),
                WatchlistItem::Instrument(gazp.clone()),
                WatchlistItem::Instrument(sber.clone()),
            ],
        };

        assert!(!watchlist.is_empty());
        assert_eq!(watchlist.len(), 4);

        match watchlist.get(0) {
            Some(WatchlistItem::Instrument(iid)) => {
                assert_eq!(iid, &sber);
            }
            _ => panic!("expected instrument"),
        }

        match watchlist.get(1) {
            Some(WatchlistItem::Group(group)) => {
                assert_eq!(group.name(), "futures");
            }
            _ => panic!("expected group"),
        }

        match watchlist.get(2) {
            Some(WatchlistItem::Instrument(iid)) => {
                assert_eq!(iid, &gazp);
            }
            _ => panic!("expected instrument"),
        }

        match watchlist.get(3) {
            Some(WatchlistItem::Instrument(iid)) => {
                assert_eq!(iid, &sber);
            }
            _ => panic!("expected instrument"),
        }

        assert!(watchlist.get(4).is_none());
    }
}
