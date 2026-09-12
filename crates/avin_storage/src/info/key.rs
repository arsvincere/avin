// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::{Category, DataProvider};

pub enum InfoKey {
    Provider {
        provider: DataProvider,
    },

    Category {
        provider: DataProvider,
        category: Category,
    },
}
