// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::Year;
use avin_domain::{DataProvider, InstrumentId, MarketData};

pub enum MarketDataKey {
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

impl MarketDataKey {
    pub fn provider(provider: DataProvider) -> Self {
        Self::Provider { provider }
    }

    pub fn instrument(provider: DataProvider, iid: InstrumentId) -> Self {
        Self::Instrument { provider, iid }
    }

    pub fn market_data(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
    ) -> Self {
        Self::MarketData { provider, iid, md }
    }

    pub fn year(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        year: Year,
    ) -> Self {
        Self::Year {
            provider,
            iid,
            md,
            year,
        }
    }
}
