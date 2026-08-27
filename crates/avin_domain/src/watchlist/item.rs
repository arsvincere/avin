// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use crate::{InstrumentId, WatchlistGroup};

#[derive(Debug, Clone)]
pub enum WatchlistItem {
    Instrument(InstrumentId),
    Group(WatchlistGroup),
}
