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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    /// Бычий
    Bull,
    /// Доджи - открытие == закрытие
    Doji,
    /// Медвежий
    Bear,
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
