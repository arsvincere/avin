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
