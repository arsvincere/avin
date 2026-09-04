// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::{DataProvider, MarketData, Year};
use avin_domain::InstrumentId;

pub enum StorageKey {
    Provider {
        provider: DataProvider,
    },

    Instrument {
        provider: DataProvider,
        iid: InstrumentId,
    },

    MarketData {
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
    },

    Year {
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        year: Year,
    },
}
