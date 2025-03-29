use crate::data::MarketData;
use chrono::TimeDelta;

#[derive(Debug, PartialEq)]
pub struct TimeFrame {
    pub name: String,
}

impl TimeFrame {
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
    pub fn as_market_data(&self) -> MarketData {
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
}

// class TimeFrame:  # {{{
//     def __hash__(self):  # {{{
//         return hash(str(self))
//
//     # }}}
//     def __eq__(self, other):  # operator ==  # {{{
//         if isinstance(other, TimeFrame):
//             return self.__period == other.__period
//         elif isinstance(other, timedelta):
//             return self.__period == other
//         elif isinstance(other, str):
//             other = TimeFrame(other)
//             return self.__period == other.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сравнение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __lt__(self, other):  # operator <  # {{{
//         if isinstance(other, TimeFrame):
//             return self.__period < other.__period
//         elif isinstance(other, timedelta):
//             return self.__period < other
//         elif isinstance(other, str):
//             other = TimeFrame(other)
//             return self.__period < other.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сравнение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __le__(self, other):  # operator <=  # {{{
//         if isinstance(other, TimeFrame):
//             return self.__period <= other.__period
//         elif isinstance(other, timedelta):
//             return self.__period <= other
//         elif isinstance(other, str):
//             other = TimeFrame(other)
//             return self.__period <= other.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сравнение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __gt__(self, other):  # operator >  # {{{
//         if isinstance(other, TimeFrame):
//             return self.__period > other.__period
//         elif isinstance(other, timedelta):
//             return self.__period > other
//         elif isinstance(other, str):
//             other = TimeFrame(other)
//             return self.__period > other.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сравнение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __ge__(self, other):  # operator >=  # {{{
//         if isinstance(other, TimeFrame):
//             return self.__period >= other.__period
//         elif isinstance(other, timedelta):
//             return self.__period >= other
//         elif isinstance(other, str):
//             other = TimeFrame(other)
//             return self.__period >= other.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сравнение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __add__(self, other):  # operator +  # {{{
//         if isinstance(other, timedelta):
//             return other + self.__period
//         if isinstance(other, datetime):
//             return other + self.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сложение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __radd__(self, other):  # operator + #  {{{
//         if isinstance(other, timedelta):
//             return other + self.__period
//         if isinstance(other, datetime):
//             return other + self.__period
//         else:
//             raise CoreError(
//                 f"Недопустимое сложение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __mul__(self, other):  # operator *  # {{{
//         if isinstance(other, int):
//             return self.__period * other
//         else:
//             raise CoreError(
//                 f"Недопустимое умножение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def __rmul__(self, other):  # operator *  # {{{
//         if isinstance(other, int):
//             return self.__period * other
//         else:
//             raise CoreError(
//                 f"Недопустимое умножение <TimeFrame> и {type(other)}"
//             )
//
//     # }}}
//     def minutes(self):  # {{{
//         return int(self.__period.total_seconds() / 60)
//
//     # }}}
//
//
// TimeFrame.ALL = [
//     TimeFrame("1M"),
//     TimeFrame("5M"),
//     TimeFrame("1H"),
//     TimeFrame("D"),
//     TimeFrame("W"),
//     TimeFrame("M"),
// ]
//
//
// # }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_timeframe() {
        TimeFrame::new("7M");
    }
    #[test]
    fn as_market_data() {
        assert_eq!(TimeFrame::new("1M").as_market_data(), MarketData::BAR_1M);
        assert_eq!(TimeFrame::new("5M").as_market_data(), MarketData::BAR_5M);
        assert_eq!(
            TimeFrame::new("10M").as_market_data(),
            MarketData::BAR_10M
        );
        assert_eq!(TimeFrame::new("1H").as_market_data(), MarketData::BAR_1H);
        assert_eq!(TimeFrame::new("D").as_market_data(), MarketData::BAR_D);
        assert_eq!(TimeFrame::new("W").as_market_data(), MarketData::BAR_W);
        assert_eq!(TimeFrame::new("M").as_market_data(), MarketData::BAR_M);
    }
}
