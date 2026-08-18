// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod constant;
mod dt;
mod error;

pub use constant::{
    DAY_BEGIN, DAY_END, ONE_DAY, ONE_HOUR, ONE_MINUTE, ONE_MONTH, ONE_SECOND,
    ONE_WEEK, ONE_YEAR,
};
pub use dt::{dt, next_month_start, prev_month_start, ts, utc_now};
pub use error::AvinError;
