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
