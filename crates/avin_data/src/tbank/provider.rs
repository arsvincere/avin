// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use avin_core::{DataProvider, MarketData, TimeRange};
use avin_domain::{
    Bar, Category, InstrumentId, InstrumentInfo, InstrumentList, Tick,
    TimeFrame,
};

use crate::DataError;

pub struct TBankProvider {}

impl TBankProvider {
    pub fn available_market_data() -> &'static [MarketData] {
        &[MarketData::Bar1M, MarketData::Tick]
    }

    pub fn fetch_instruments(
        category: Category,
    ) -> Result<InstrumentList, DataError> {
        todo!()
    }

    pub fn fetch_bars(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
    ) -> Result<Vec<Bar>, DataError> {
        if tf != TimeFrame::M1 {
            let msg = format!(
                "{} doesn't provide {tf} bars, available=[{}]",
                DataProvider::TBank,
                TimeFrame::M1,
            );
            return Err(DataError::Unavailable {
                message: msg,
                source: None,
            });
        }

        todo!()
    }

    pub fn fetch_ticks(
        instrument: InstrumentInfo,
        range: TimeRange,
    ) -> Result<Vec<Tick>, DataError> {
        todo!()
    }
}
