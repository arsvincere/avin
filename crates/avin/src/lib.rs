// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────
// CREATED: 2023.07.23 15:06

//! # AVIN  -  Ars Vincere (лат. искусство побеждать)
//! ```text
//!                             Open source cross-platform trading system
//!                                      __   _    _  ___  __   _
//!                                     /__\   \  /    |   | \  |
//!                                    |    |   \/    _|_  |  \_|
//!
//! ```

pub use avin_model::{
    Bar, BarDirection, Exchange, InstrumentKind, PriceRange, Symbol,
    TimeFrame,
};

pub use avin_utils::AvinError;
