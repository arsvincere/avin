// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(clippy::module_inception)]

mod data;
mod error;
mod time;
mod value;

pub use data::{DataProvider, MarketData};
pub use error::CoreError;
pub use time::{Time, TimeRange, Year};
pub use value::{Price, PriceRange, Quantity};
