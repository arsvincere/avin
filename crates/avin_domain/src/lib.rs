// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(clippy::module_inception)]

mod asset;
mod chart;
mod error;
mod instrument;
mod watchlist;

pub use asset::{Asset, Future, HasCharts, InstrumentInfoView, Share};
pub use chart::{Bar, BarDirection, Chart, TimeFrame};
pub use error::DomainError;
pub use instrument::{
    Category, Exchange, InstrumentId, InstrumentInfo, InstrumentList,
    MarketData, Ticker,
};
pub use watchlist::{Watchlist, WatchlistGroup, WatchlistItem};
