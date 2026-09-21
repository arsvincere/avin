// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod data;
mod error;
mod instrument;

pub use data::DataService;
pub use error::ServiceError;
pub use instrument::InstrumentService;
