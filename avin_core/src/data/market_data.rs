/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use strum::VariantNames;

use avin_utils::AvinError;

use crate::TimeFrame;

/// List for selecting the market data type.
///
/// # ru
/// Перечисление для выбора типа данных.
#[allow(non_camel_case_types)]
#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
)]
pub enum MarketData {
    BAR_1M,
    BAR_5M,
    BAR_10M,
    BAR_15M,
    BAR_1H,
    BAR_4H,
    BAR_DAY,
    BAR_WEEK,
    BAR_MONTH,
    TIC,
    ORDER_BOOK,
    SC_TRADE,
    SC_ORDER,
    SC_OB,
}
impl MarketData {
    pub fn all() -> Vec<MarketData> {
        vec![
            MarketData::BAR_1M,
            MarketData::BAR_5M,
            MarketData::BAR_10M,
            MarketData::BAR_15M,
            MarketData::BAR_1H,
            MarketData::BAR_4H,
            MarketData::BAR_DAY,
            MarketData::BAR_WEEK,
            MarketData::BAR_MONTH,
            MarketData::TIC,
            MarketData::ORDER_BOOK,
            MarketData::SC_TRADE,
            MarketData::SC_ORDER,
            MarketData::SC_OB,
        ]
    }
    /// Return market data type name.
    ///
    /// # ru
    /// Возвращает название типа биржевых данных.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BAR_1M => "BAR_1M",
            Self::BAR_5M => "BAR_5M",
            Self::BAR_10M => "BAR_10M",
            Self::BAR_15M => "BAR_15M",
            Self::BAR_1H => "BAR_1H",
            Self::BAR_4H => "BAR_4H",
            Self::BAR_DAY => "BAR_DAY",
            Self::BAR_WEEK => "BAR_WEEK",
            Self::BAR_MONTH => "BAR_MONTH",
            Self::TIC => "TIC",
            Self::ORDER_BOOK => "ORDER_BOOK",
            Self::SC_TRADE => "SC_TRADE",
            Self::SC_ORDER => "SC_ORDER",
            Self::SC_OB => "SC_OB",
        }
    }
    /// Return TimeFrame enum for this market data.
    ///
    /// # ru
    /// Возвращает соответствующее значение TimeFrame для данного
    /// типа рыночных данных, если это возможно.
    ///
    /// Если это тики - AvinError::InvalidValue.
    pub fn timeframe(&self) -> Result<TimeFrame, AvinError> {
        match self {
            MarketData::BAR_1M => Ok(TimeFrame::M1),
            MarketData::BAR_5M => Ok(TimeFrame::M5),
            MarketData::BAR_10M => Ok(TimeFrame::M10),
            MarketData::BAR_15M => Ok(TimeFrame::M15),
            MarketData::BAR_1H => Ok(TimeFrame::H1),
            MarketData::BAR_4H => Ok(TimeFrame::H4),
            MarketData::BAR_DAY => Ok(TimeFrame::Day),
            MarketData::BAR_WEEK => Ok(TimeFrame::Week),
            MarketData::BAR_MONTH => Ok(TimeFrame::Month),
            _ => {
                let msg = format!("Can't convert {self} -> TimeFrame");
                let e = AvinError::InvalidValue(msg);
                Err(e)
            }
        }
    }
}
// impl From<&str> for MarketData {
//     fn from(value: &str) -> Self {
//         match value.to_uppercase().as_str() {
//             "BAR_1M" => MarketData::BAR_1M,
//             "BAR_5M" => MarketData::BAR_5M,
//             "BAR_10M" => MarketData::BAR_10M,
//             "BAR_15M" => MarketData::BAR_15M,
//             "BAR_1H" => MarketData::BAR_1H,
//             "BAR_4H" => MarketData::BAR_4H,
//             "BAR_DAY" => MarketData::BAR_DAY,
//             "BAR_WEEK" => MarketData::BAR_WEEK,
//             "BAR_MONTH" => MarketData::BAR_MONTH,
//             "TIC" => MarketData::TIC,
//             "TRADE_STATS" => MarketData::TRADE_STATS,
//             "ORDER_STATS" => MarketData::ORDER_STATS,
//             "OB_STATS" => MarketData::OB_STATS,
//             _ => panic!("Invalid value for MarketData: {value}"),
//         }
//     }
// }
impl std::str::FromStr for MarketData {
    type Err = AvinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BAR_1M" => Ok(MarketData::BAR_1M),
            "BAR_5M" => Ok(MarketData::BAR_5M),
            "BAR_10M" => Ok(MarketData::BAR_10M),
            "BAR_15M" => Ok(MarketData::BAR_15M),
            "BAR_1H" => Ok(MarketData::BAR_1H),
            "BAR_4H" => Ok(MarketData::BAR_4H),
            "BAR_DAY" => Ok(MarketData::BAR_DAY),
            "BAR_WEEK" => Ok(MarketData::BAR_WEEK),
            "BAR_MONTH" => Ok(MarketData::BAR_MONTH),
            "TIC" => Ok(MarketData::TIC),
            "ORDER_BOOK" => Ok(MarketData::ORDER_BOOK),
            "SC_TRADE" => Ok(MarketData::SC_TRADE),
            "SC_ORDER" => Ok(MarketData::SC_ORDER),
            "SC_OB" => Ok(MarketData::SC_OB),
            _ => {
                let msg = format!("{s}, available={:?}", MarketData::VARIANTS);
                let e = AvinError::InvalidValue(msg);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn all() {
        let all_market_data_kind = MarketData::all();
        assert_eq!(all_market_data_kind.len(), 14);
    }

    #[test]
    fn name() {
        assert_eq!(MarketData::BAR_1M.name(), "BAR_1M");
        assert_eq!(MarketData::BAR_5M.name(), "BAR_5M");
        assert_eq!(MarketData::BAR_10M.name(), "BAR_10M");
        assert_eq!(MarketData::BAR_15M.name(), "BAR_15M");
        assert_eq!(MarketData::BAR_1H.name(), "BAR_1H");
        assert_eq!(MarketData::BAR_4H.name(), "BAR_4H");
        assert_eq!(MarketData::BAR_DAY.name(), "BAR_DAY");
        assert_eq!(MarketData::BAR_WEEK.name(), "BAR_WEEK");
        assert_eq!(MarketData::BAR_MONTH.name(), "BAR_MONTH");
        assert_eq!(MarketData::TIC.name(), "TIC");
        assert_eq!(MarketData::ORDER_BOOK.name(), "ORDER_BOOK");
        assert_eq!(MarketData::SC_TRADE.name(), "SC_TRADE");
        assert_eq!(MarketData::SC_ORDER.name(), "SC_ORDER");
        assert_eq!(MarketData::SC_OB.name(), "SC_OB");
    }
    #[test]
    fn timeframe() {
        assert_eq!(MarketData::BAR_1M.timeframe().unwrap(), TimeFrame::M1);
        assert_eq!(MarketData::BAR_5M.timeframe().unwrap(), TimeFrame::M5);
        assert_eq!(MarketData::BAR_10M.timeframe().unwrap(), TimeFrame::M10);
        assert_eq!(MarketData::BAR_15M.timeframe().unwrap(), TimeFrame::M15);
        assert_eq!(MarketData::BAR_1H.timeframe().unwrap(), TimeFrame::H1);
        assert_eq!(MarketData::BAR_4H.timeframe().unwrap(), TimeFrame::H4);
        assert_eq!(MarketData::BAR_DAY.timeframe().unwrap(), TimeFrame::Day);
        assert_eq!(MarketData::BAR_WEEK.timeframe().unwrap(), TimeFrame::Week);
        assert_eq!(
            MarketData::BAR_MONTH.timeframe().unwrap(),
            TimeFrame::Month
        );
    }
    #[test]
    fn to_str() {
        assert_eq!(MarketData::BAR_1M.to_string(), "BAR_1M");
        assert_eq!(MarketData::BAR_5M.to_string(), "BAR_5M");
        assert_eq!(MarketData::BAR_10M.to_string(), "BAR_10M");
        assert_eq!(MarketData::BAR_15M.to_string(), "BAR_15M");
        assert_eq!(MarketData::BAR_1H.to_string(), "BAR_1H");
        assert_eq!(MarketData::BAR_4H.to_string(), "BAR_4H");
        assert_eq!(MarketData::BAR_DAY.to_string(), "BAR_DAY");
        assert_eq!(MarketData::BAR_WEEK.to_string(), "BAR_WEEK");
        assert_eq!(MarketData::BAR_MONTH.to_string(), "BAR_MONTH");
        assert_eq!(MarketData::TIC.to_string(), "TIC");
        assert_eq!(MarketData::ORDER_BOOK.to_string(), "ORDER_BOOK");
        assert_eq!(MarketData::SC_TRADE.to_string(), "SC_TRADE");
        assert_eq!(MarketData::SC_ORDER.to_string(), "SC_ORDER");
        assert_eq!(MarketData::SC_OB.to_string(), "SC_OB");
    }
    #[test]
    fn from_str() {
        assert_eq!(MarketData::BAR_1M, "BAR_1M".parse().unwrap());
        assert_eq!(MarketData::BAR_5M, MarketData::from_str("BAR_5M").unwrap());
        assert_eq!(
            MarketData::BAR_10M,
            MarketData::from_str("BAR_10M").unwrap()
        );
        assert_eq!(
            MarketData::BAR_15M,
            MarketData::from_str("BAR_15M").unwrap()
        );
        assert_eq!(MarketData::BAR_1H, MarketData::from_str("BAR_1H").unwrap());
        assert_eq!(MarketData::BAR_4H, MarketData::from_str("BAR_4H").unwrap());
        assert_eq!(
            MarketData::BAR_DAY,
            MarketData::from_str("BAR_DAY").unwrap()
        );
        assert_eq!(
            MarketData::BAR_WEEK,
            MarketData::from_str("BAR_WEEK").unwrap()
        );
        assert_eq!(
            MarketData::BAR_MONTH,
            MarketData::from_str("BAR_MONTH").unwrap()
        );
        assert_eq!(MarketData::TIC, MarketData::from_str("TIC").unwrap());
        assert_eq!(
            MarketData::ORDER_BOOK,
            MarketData::from_str("ORDER_BOOK").unwrap()
        );
        assert_eq!(
            MarketData::SC_TRADE,
            MarketData::from_str("SC_TRADE").unwrap()
        );
        assert_eq!(
            MarketData::SC_ORDER,
            MarketData::from_str("SC_ORDER").unwrap()
        );
        assert_eq!(MarketData::SC_OB, MarketData::from_str("SC_OB").unwrap());
    }
}
