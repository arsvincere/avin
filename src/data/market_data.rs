/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#![allow(non_camel_case_types)]

use crate::core::TimeFrame;

#[derive(Debug, PartialEq, Clone)]
pub enum MarketData {
    BAR_1M,
    BAR_5M,
    BAR_10M,
    BAR_1H,
    BAR_D,
    BAR_W,
    BAR_M,
    TIC,
}

impl MarketData {
    pub fn name(&self) -> String {
        match self {
            MarketData::BAR_1M => String::from("BAR_1M"),
            MarketData::BAR_5M => String::from("BAR_5M"),
            MarketData::BAR_10M => String::from("BAR_10M"),
            MarketData::BAR_1H => String::from("BAR_1H"),
            MarketData::BAR_D => String::from("BAR_D"),
            MarketData::BAR_W => String::from("BAR_W"),
            MarketData::BAR_M => String::from("BAR_M"),
            MarketData::TIC => String::from("TIC"),
        }
    }
    pub fn from(s: &str) -> Result<MarketData, &'static str> {
        let market_data = s.to_uppercase();
        match market_data.as_str() {
            "1M" => Ok(MarketData::BAR_1M),
            "5M" => Ok(MarketData::BAR_5M),
            "10M" => Ok(MarketData::BAR_10M),
            "1H" => Ok(MarketData::BAR_1H),
            "D" => Ok(MarketData::BAR_D),
            "W" => Ok(MarketData::BAR_W),
            "M" => Ok(MarketData::BAR_M),
            "BAR_1M" => Ok(MarketData::BAR_1M),
            "BAR_5M" => Ok(MarketData::BAR_5M),
            "BAR_10M" => Ok(MarketData::BAR_10M),
            "BAR_1H" => Ok(MarketData::BAR_1H),
            "BAR_D" => Ok(MarketData::BAR_D),
            "BAR_W" => Ok(MarketData::BAR_W),
            "BAR_M" => Ok(MarketData::BAR_M),
            "TIC" => Ok(MarketData::TIC),
            _ => Err("Invalid data type"),
        }
    }
}
impl std::fmt::Display for MarketData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MarketData={}", self.name())
    }
}
impl From<TimeFrame> for MarketData {
    fn from(tf: TimeFrame) -> MarketData {
        match tf {
            TimeFrame::M1 => MarketData::BAR_1M,
            TimeFrame::M10 => MarketData::BAR_10M,
            TimeFrame::H1 => MarketData::BAR_1H,
            TimeFrame::Day => MarketData::BAR_D,
            TimeFrame::Week => MarketData::BAR_W,
            TimeFrame::Month => MarketData::BAR_M,
        }
    }
}
