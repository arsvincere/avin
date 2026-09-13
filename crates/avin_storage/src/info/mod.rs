// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod key;
#[allow(clippy::module_inception)]
mod storage;

pub use key::InfoKey;
pub use storage::InstrumentInfoStorage;
