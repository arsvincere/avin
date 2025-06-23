/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::TimeFrame;

/// List for selecting the market data type.
///
/// # ru
/// Перечисление для выбора типа данных.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, strum::Display)]
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
    /// Return market data type name
    ///
    /// # ru
    /// Возвращает название типа биржевых данных
    pub fn name(&self) -> String {
        match self {
            Self::BAR_1M => "BAR_1M".to_string(),
            Self::BAR_5M => "BAR_5M".to_string(),
            Self::BAR_10M => "BAR_10M".to_string(),
            Self::BAR_1H => "BAR_1H".to_string(),
            Self::BAR_D => "BAR_D".to_string(),
            Self::BAR_W => "BAR_W".to_string(),
            Self::BAR_M => "BAR_M".to_string(),
            Self::TIC => "TIC".to_string(),
        }
    }
}
impl From<&str> for MarketData {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "1M" => MarketData::BAR_1M,
            "5M" => MarketData::BAR_5M,
            "10M" => MarketData::BAR_10M,
            "1H" => MarketData::BAR_1H,
            "D" => MarketData::BAR_D,
            "W" => MarketData::BAR_W,
            "M" => MarketData::BAR_M,
            "BAR_1M" => MarketData::BAR_1M,
            "BAR_5M" => MarketData::BAR_5M,
            "BAR_10M" => MarketData::BAR_10M,
            "BAR_1H" => MarketData::BAR_1H,
            "BAR_D" => MarketData::BAR_D,
            "BAR_W" => MarketData::BAR_W,
            "BAR_M" => MarketData::BAR_M,
            "TIC" => MarketData::TIC,
            _ => panic!("Invalid value for MarketData: {}", value),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(MarketData::TIC.to_string(), "TIC");

        assert_eq!(MarketData::BAR_1M.to_string(), "BAR_1M");
        assert_eq!(MarketData::BAR_5M.to_string(), "BAR_5M");
        assert_eq!(MarketData::BAR_10M.to_string(), "BAR_10M");
        assert_eq!(MarketData::BAR_1H.to_string(), "BAR_1H");
        assert_eq!(MarketData::BAR_D.to_string(), "BAR_D");
        assert_eq!(MarketData::BAR_W.to_string(), "BAR_W");
        assert_eq!(MarketData::BAR_M.to_string(), "BAR_M");
    }
    #[test]
    fn name() {
        assert_eq!(MarketData::TIC.name(), "TIC");

        assert_eq!(MarketData::BAR_1M.name(), "BAR_1M");
        assert_eq!(MarketData::BAR_5M.name(), "BAR_5M");
        assert_eq!(MarketData::BAR_10M.name(), "BAR_10M");
        assert_eq!(MarketData::BAR_1H.name(), "BAR_1H");
        assert_eq!(MarketData::BAR_D.name(), "BAR_D");
        assert_eq!(MarketData::BAR_W.name(), "BAR_W");
        assert_eq!(MarketData::BAR_M.name(), "BAR_M");
    }
    #[test]
    fn from_str() {
        assert_eq!(MarketData::TIC, "TIC".into());

        assert_eq!(MarketData::BAR_1M, "BAR_1M".into());
        assert_eq!(MarketData::BAR_10M, "BAR_10M".into());
        assert_eq!(MarketData::BAR_1H, "BAR_1H".into());
        assert_eq!(MarketData::BAR_D, "BAR_D".into());
        assert_eq!(MarketData::BAR_W, "BAR_W".into());
        assert_eq!(MarketData::BAR_M, "BAR_M".into());

        assert_eq!(MarketData::BAR_1M, "1M".into());
        assert_eq!(MarketData::BAR_10M, "10M".into());
        assert_eq!(MarketData::BAR_1H, "1H".into());
        assert_eq!(MarketData::BAR_D, "D".into());
        assert_eq!(MarketData::BAR_W, "W".into());
        assert_eq!(MarketData::BAR_M, "M".into());
    }
    #[test]
    fn from_timeframe() {
        assert_eq!(MarketData::BAR_1M, TimeFrame::M1.into());
        assert_eq!(MarketData::BAR_10M, TimeFrame::M10.into());
        assert_eq!(MarketData::BAR_1H, TimeFrame::H1.into());
        assert_eq!(MarketData::BAR_D, TimeFrame::Day.into());
        assert_eq!(MarketData::BAR_W, TimeFrame::Week.into());
        assert_eq!(MarketData::BAR_M, TimeFrame::Month.into());
    }
}
