// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

/// Bar direction.
///
/// # ru
/// Тип бара: бычий, доджи, медвежий.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarDirection {
    /// Бычий
    Bull = 1,
    /// Доджи (открытие == закрытие)
    Doji = 0,
    /// Медвежий
    Bear = -1,
}

impl std::fmt::Display for BarDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BarDirection::Bull => write!(f, "Bull"),
            BarDirection::Doji => write!(f, "Doji"),
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
        assert_eq!(BarDirection::Doji as i8, 0);
        assert_eq!(BarDirection::Bear as i8, -1);
    }

    #[test]
    fn display() {
        assert_eq!(BarDirection::Bull.to_string(), "Bull");
        assert_eq!(BarDirection::Doji.to_string(), "Doji");
        assert_eq!(BarDirection::Bear.to_string(), "Bear");
    }
}
