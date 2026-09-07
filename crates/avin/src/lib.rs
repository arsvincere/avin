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

// self
pub mod cli;
mod error;

// re-exports
pub mod prelude;

pub mod model {
    pub use avin_core::*;
    pub use avin_domain::*;
}
pub use avin_system as system;

pub mod err {
    pub use crate::error::AvinError;
    pub use avin_core::CoreError;
    pub use avin_domain::DomainError;
    pub use avin_system::SystemError;
}
