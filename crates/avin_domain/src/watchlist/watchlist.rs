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

impl Watchlist {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> &[WatchlistItem] {
        &self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, n: usize) -> Option<&WatchlistItem> {
        self.items.get(n)
    }
}
