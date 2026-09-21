// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod error;
mod packs;
mod tbank;

pub use error::DataError;
pub use packs::{BarsPack, InstrumentPack};
pub use tbank::TBankProvider;
