// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

/// # en
/// Bar direction.
///
/// Indicates whether the bar is bullish, neutral, or bearish.
///
/// # ru
/// Направление бара.
///
/// Указывает, является ли бар бычьим, нейтральным или медвежьим.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarDirection {
    Bull = 1,
    Neutral = 0,
    Bear = -1,
}

impl std::fmt::Display for BarDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BarDirection::Bull => write!(f, "Bull"),
            BarDirection::Neutral => write!(f, "Neutral"),
            BarDirection::Bear => write!(f, "Bear"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values() {
        assert_eq!(BarDirection::Bull as i8, 1);
        assert_eq!(BarDirection::Neutral as i8, 0);
        assert_eq!(BarDirection::Bear as i8, -1);
    }

    #[test]
    fn display() {
        assert_eq!(BarDirection::Bull.to_string(), "Bull");
        assert_eq!(BarDirection::Neutral.to_string(), "Neutral");
        assert_eq!(BarDirection::Bear.to_string(), "Bear");
    }
}
