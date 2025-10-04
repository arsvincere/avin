/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the market data type.
///
/// # ru
/// Перечисление для выбора типа данных.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub enum MarketData {
    BAR_1M,
    BAR_10M,
    BAR_1H,
    BAR_DAY,
    BAR_WEEK,
    BAR_MONTH,
    TIC,
    TRADE_STATS,
    ORDER_STATS,
    OB_STATS,
}
impl MarketData {
    /// Return market data type name.
    ///
    /// # ru
    /// Возвращает название типа биржевых данных.
    pub fn name(&self) -> &'static str {
        match self {
            Self::BAR_1M => "BAR_1M",
            Self::BAR_10M => "BAR_10M",
            Self::BAR_1H => "BAR_1H",
            Self::BAR_DAY => "BAR_DAY",
            Self::BAR_WEEK => "BAR_WEEK",
            Self::BAR_MONTH => "BAR_MONTH",
            Self::TIC => "TIC",
            Self::TRADE_STATS => "TRADE_STATS",
            Self::ORDER_STATS => "ORDER_STATS",
            Self::OB_STATS => "OB_STATS",
        }
    }
}
impl From<&str> for MarketData {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "BAR_1M" => MarketData::BAR_1M,
            "BAR_10M" => MarketData::BAR_10M,
            "BAR_1H" => MarketData::BAR_1H,
            "BAR_DAY" => MarketData::BAR_DAY,
            "BAR_WEEK" => MarketData::BAR_WEEK,
            "BAR_MONTH" => MarketData::BAR_MONTH,
            "TIC" => MarketData::TIC,
            "TRADE_STATS" => MarketData::TRADE_STATS,
            "ORDER_STATS" => MarketData::ORDER_STATS,
            "OB_STATS" => MarketData::OB_STATS,
            _ => panic!("Invalid value for MarketData: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(MarketData::BAR_1M.name(), "BAR_1M");
        assert_eq!(MarketData::BAR_10M.name(), "BAR_10M");
        assert_eq!(MarketData::BAR_1H.name(), "BAR_1H");
        assert_eq!(MarketData::BAR_DAY.name(), "BAR_DAY");
        assert_eq!(MarketData::BAR_WEEK.name(), "BAR_WEEK");
        assert_eq!(MarketData::BAR_MONTH.name(), "BAR_MONTH");
        assert_eq!(MarketData::TIC.name(), "TIC");
        assert_eq!(MarketData::TRADE_STATS.name(), "TRADE_STATS");
        assert_eq!(MarketData::ORDER_STATS.name(), "ORDER_STATS");
        assert_eq!(MarketData::OB_STATS.name(), "OB_STATS");
    }
    #[test]
    fn to_str() {
        assert_eq!(MarketData::BAR_1M.to_string(), "BAR_1M");
        assert_eq!(MarketData::BAR_10M.to_string(), "BAR_10M");
        assert_eq!(MarketData::BAR_1H.to_string(), "BAR_1H");
        assert_eq!(MarketData::BAR_DAY.to_string(), "BAR_DAY");
        assert_eq!(MarketData::BAR_WEEK.to_string(), "BAR_WEEK");
        assert_eq!(MarketData::BAR_MONTH.to_string(), "BAR_MONTH");
        assert_eq!(MarketData::TIC.to_string(), "TIC");
        assert_eq!(MarketData::TRADE_STATS.to_string(), "TRADE_STATS");
        assert_eq!(MarketData::ORDER_STATS.to_string(), "ORDER_STATS");
        assert_eq!(MarketData::OB_STATS.to_string(), "OB_STATS");
    }
    #[test]
    fn from_str() {
        assert_eq!(MarketData::BAR_1M, "BAR_1M".into());
        assert_eq!(MarketData::BAR_10M, "BAR_10M".into());
        assert_eq!(MarketData::BAR_1H, "BAR_1H".into());
        assert_eq!(MarketData::BAR_DAY, "BAR_DAY".into());
        assert_eq!(MarketData::BAR_WEEK, "BAR_WEEK".into());
        assert_eq!(MarketData::BAR_MONTH, "BAR_MONTH".into());
        assert_eq!(MarketData::TIC, "TIC".into());
        assert_eq!(MarketData::TRADE_STATS, "TRADE_STATS".into());
        assert_eq!(MarketData::ORDER_STATS, "ORDER_STATS".into());
        assert_eq!(MarketData::OB_STATS, "OB_STATS".into());
    }
}
