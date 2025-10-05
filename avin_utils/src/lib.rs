/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

//! # AVIN  -  Ars Vincere.
//!
//! Utils for general crate `avin`.
//!
//! # ru
//! Утилиты для основного крейта `avin`.

mod cmd;
mod conf;
mod error;
mod logger;
mod misc;
mod timer;

pub use cmd::Cmd;
pub use conf::{CFG, Configuration};
pub use error::AvinError;
pub use logger::init_logger;
pub use misc::{
    DAY_BEGIN, DAY_END, MINUTES_IN_DAY, MSK_OFFSET, bisect_left,
    bisect_right, dt, filter_dt, max, min, next_month, replace_ts, round,
    round_price, str_date_to_utc, str_dt_to_utc, sum, ts,
};
pub use timer::Timer;
