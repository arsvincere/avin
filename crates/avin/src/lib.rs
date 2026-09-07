// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

//! # AVIN  -  Ars Vincere (лат. искусство побеждать)
//! ```text
//!                             Open source cross-platform trading system
//!                                      __   _    _  ___  __   _
//!                                     /__\   \  /    |   | \  |
//!                                    |    |   \/    _|_  |  \_|
//!
//! ```

pub mod prelude;

pub mod model {
    pub use avin_core::*;
    pub use avin_domain::*;
}

pub mod cli;

mod error;
pub use error::AvinError;
