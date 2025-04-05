/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::data::MarketData;
use bitcode::{Decode, Encode};
use chrono::TimeDelta;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Clone, Encode, Decode)]
pub struct TimeFrame {
    name: String,
}

impl TimeFrame {
    pub fn new(name: &str) -> Self {
        let valid_name = match name {
            "1M" => name,
            "5M" => name,
            "10M" => name,
            "1H" => name,
            "D" => name,
            "W" => name,
            "M" => name,
            _ => panic!("Invalid TimeFrame: {name}"),
        };

        TimeFrame {
            name: valid_name.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn timedelta(&self) -> TimeDelta {
        match self.name.as_str() {
            "1M" => TimeDelta::new(60, 0).unwrap(),
            "5M" => TimeDelta::new(5 * 60, 0).unwrap(),
            "10M" => TimeDelta::new(10 * 60, 0).unwrap(),
            "1H" => TimeDelta::new(60 * 60, 0).unwrap(),
            "D" => TimeDelta::new(24 * 60 * 60, 0).unwrap(),
            "W" => TimeDelta::new(7 * 24 * 60 * 60, 0).unwrap(),
            "M" => TimeDelta::new(31 * 24 * 60 * 60, 0).unwrap(),
            _ => panic!("Invalid TimeFrame: {}", self.name),
        }
    }
    pub fn to_market_data(&self) -> MarketData {
        match self.name.as_str() {
            "1M" => MarketData::BAR_1M,
            "5M" => MarketData::BAR_5M,
            "10M" => MarketData::BAR_10M,
            "1H" => MarketData::BAR_1H,
            "D" => MarketData::BAR_D,
            "W" => MarketData::BAR_W,
            "M" => MarketData::BAR_M,
            _ => panic!("Invalid TimeFrame: {}", self.name),
        }
    }
}

impl Hash for TimeFrame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_timeframe() {
        TimeFrame::new("7M");
    }
    #[test]
    fn to_market_data() {
        assert_eq!(TimeFrame::new("1M").to_market_data(), MarketData::BAR_1M);
        assert_eq!(TimeFrame::new("5M").to_market_data(), MarketData::BAR_5M);
        assert_eq!(
            TimeFrame::new("10M").to_market_data(),
            MarketData::BAR_10M
        );
        assert_eq!(TimeFrame::new("1H").to_market_data(), MarketData::BAR_1H);
        assert_eq!(TimeFrame::new("D").to_market_data(), MarketData::BAR_D);
        assert_eq!(TimeFrame::new("W").to_market_data(), MarketData::BAR_W);
        assert_eq!(TimeFrame::new("M").to_market_data(), MarketData::BAR_M);
    }
}
