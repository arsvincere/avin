// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use avin_core::{MarketData, TimeRange};
use avin_domain::InstrumentId;

pub struct TBankProvider {}

impl TBankProvider {
    pub fn available_market_data() -> &'static [MarketData] {
        &[MarketData::Bar1M, MarketData::Tick]
    }

    pub fn cache() {
        todo!()
    }

    pub fn fetch(iid: InstrumentId, md: MarketData, range: TimeRange) {
        todo!()
    }
}
