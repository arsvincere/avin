// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod bars_pack;
mod instrument_pack;
mod ticks_pack;

pub use bars_pack::BarsPack;
pub use instrument_pack::InstrumentPack;
pub use ticks_pack::TicksPack;

pub type PackIterator<T> = Box<dyn Iterator<Item = Result<T, DataError>>>;
