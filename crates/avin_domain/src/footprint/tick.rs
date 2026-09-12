// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: impl, docs, tests

use avin_core::{Direction, Price, Quantity, Time};

pub struct Tick {
    pub time: Time,
    pub direction: Direction,
    pub price: Price,
    pub quantity: Quantity,
}
