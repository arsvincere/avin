// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod key;
mod storage;
mod write_operation;

pub use key::MarketDataKey;
pub use storage::{MarketDataStorage, StorageStatus};
pub use write_operation::WriteOperation;
