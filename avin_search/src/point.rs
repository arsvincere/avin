/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub ts: i64,
    pub price: f64,
}
impl Point {
    pub fn new(ts: i64, price: f64) -> Self {
        Self { ts, price }
    }
}
