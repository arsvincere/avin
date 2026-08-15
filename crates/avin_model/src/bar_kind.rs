// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

/// Bar kind.
///
/// # ru
/// Тип бара: бычий, доджи, медвежий.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    /// Бычий
    Bull = 1,
    /// Доджи (открытие == закрытие)
    Doji = 0,
    /// Медвежий
    Bear = -1,
}

impl std::fmt::Display for BarKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BarKind::Bull => write!(f, "Bull"),
            BarKind::Doji => write!(f, "Doji"),
            BarKind::Bear => write!(f, "Bear"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(BarKind::Bull.to_string(), "Bull");
        assert_eq!(BarKind::Doji.to_string(), "Doji");
        assert_eq!(BarKind::Bear.to_string(), "Bear");
    }
}
