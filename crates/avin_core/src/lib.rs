// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod data_provider;
mod error;
mod time;
mod time_range;

pub use data_provider::DataProvider;
pub use error::CoreError;
pub use time::Time;
pub use time_range::TimeRange;
