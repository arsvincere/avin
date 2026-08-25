// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use crate::InstrumentId;

#[derive(Debug)]
pub struct WatchlistGroup {
    name: String,
    instruments: Vec<InstrumentId>,
}

impl WatchlistGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instruments: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instruments(&self) -> &[InstrumentId] {
        &self.instruments
    }

    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn get(&self, n: usize) -> Option<&InstrumentId> {
        self.instruments.get(n)
    }
}
