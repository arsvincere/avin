// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_utils::AvinError;

use crate::{
    Category, Chart, Future, HasCharts, InstrumentInfo, InstrumentInfoView,
    Share, TimeFrame,
};

/// Tradable runtime asset.
///
/// Represents a tradable financial instrument and optionally associated
/// market data. Market data is loaded explicitly by services and is not
/// available immediately after asset creation.
///
/// Cash currencies and non-tradable market entities such as indices are not
/// assets.
pub enum Asset {
    Share(Share),
    Future(Future),
}

impl TryFrom<InstrumentInfo> for Asset {
    type Error = AvinError;

    fn try_from(value: InstrumentInfo) -> Result<Self, Self::Error> {
        match value.category() {
            Category::Share => Ok(Self::Share(Share::try_from(value)?)),
            Category::Future => Ok(Self::Future(Future::try_from(value)?)),
            category => Err(AvinError::Value(format!(
                "unsupported asset category '{category}'"
            ))),
        }
    }
}

impl InstrumentInfoView for Asset {
    fn info(&self) -> &InstrumentInfo {
        match self {
            Self::Share(share) => share.info(),
            Self::Future(future) => future.info(),
        }
    }
}

impl HasCharts for Asset {
    fn chart(&self, tf: TimeFrame) -> Option<&Chart> {
        match self {
            Self::Share(share) => share.chart(tf),
            Self::Future(future) => future.chart(tf),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn get_info(category: Category) -> InstrumentInfo {
        let raw_info = HashMap::from([
            ("exchange".to_string(), "MOEX".to_string()),
            ("category".to_string(), category.to_string()),
            ("ticker".to_string(), "TEST".to_string()),
            ("figi".to_string(), "TEST_FIGI".to_string()),
            ("name".to_string(), "Test Instrument".to_string()),
            ("lot".to_string(), "5".to_string()),
            ("step".to_string(), "0.05".to_string()),
        ]);

        InstrumentInfo::new(raw_info).unwrap()
    }

    #[test]
    fn share() {
        let asset = Asset::try_from(get_info(Category::Share)).unwrap();

        assert!(matches!(asset, Asset::Share(_)));
        assert_eq!(asset.category(), Category::Share);
        assert_eq!(asset.name(), "Test Instrument");
        assert!(asset.chart(TimeFrame::Day).is_none());
    }

    #[test]
    fn future() {
        let asset = Asset::try_from(get_info(Category::Future)).unwrap();

        assert!(matches!(asset, Asset::Future(_)));
        assert_eq!(asset.category(), Category::Future);
        assert_eq!(asset.name(), "Test Instrument");
        assert!(asset.chart(TimeFrame::Day).is_none());
    }

    #[test]
    fn unsupported_category() {
        let result = Asset::try_from(get_info(Category::Index));

        assert!(matches!(result, Err(AvinError::Value(_))));
    }
}
