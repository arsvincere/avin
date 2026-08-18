// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use chrono::{DateTime, Utc};

/// Convert datetime UTC -> timestamp nanos.
#[inline]
pub fn ts(dt: DateTime<Utc>) -> i64 {
    dt.timestamp_nanos_opt().unwrap()
}

/// Convert timestamp nanos -> datetime UTC.
#[inline]
pub fn dt(ts: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(ts)
}
