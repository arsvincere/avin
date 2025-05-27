/*****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

//! # AVIN  -  Ars Vincere
//! ```text
//!                               Open source cross-platform trading system
//!                             ,.               ,.   , .
//!                            / |   ,-. ,-.     `|  /  . ,-. ,-. ,-. ,-. ,-.
//!                           /~~|-. |   `-.      | /   | | | |   |-' |   |-'
//!                         ,'   `-' '   `-'      `'    ' ' ' `-' `-' '   `-'
//!                                       (лат. искусство побеждать)
//! ```
//!
//! Coming soon...

mod analytic;
mod conf;
mod core;
mod data;
mod extra;
mod strategy;
mod tester;
mod tinkoff;
mod trader;

pub mod gui;
pub mod utils;

pub use analytic::*;
pub use conf::*;
pub use core::*;
pub use data::*;
pub use extra::*;
pub use strategy::*;
pub use tester::*;
pub use tinkoff::*;
pub use trader::*;
pub use utils::Cmd;
pub use utils::LOGGER;
