// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::DataProvider;
use avin_domain::Category;

pub enum InfoKey {
    Provider {
        provider: DataProvider,
    },

    Category {
        provider: DataProvider,
        category: Category,
    },
}
