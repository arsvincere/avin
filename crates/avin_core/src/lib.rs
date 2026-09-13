// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(clippy::module_inception)]

mod common;
mod data;
mod error;
mod price;
mod time;

pub use common::{Direction, Quantity};
pub use data::{DataProvider, MarketData};
pub use error::CoreError;
pub use price::{Price, PriceRange};
pub use time::{Time, TimeRange, Year};
