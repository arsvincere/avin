// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use avin_utils::AvinError;

use crate::{
    Category, Chart, HasCharts, InstrumentInfo, InstrumentInfoView, TimeFrame,
};

// TODO: Test access to loaded charts when market-data loading/attachment
// is implemented in the service layer.

pub struct Future {
    info: InstrumentInfo,
    charts: HashMap<TimeFrame, Chart>,
}

impl TryFrom<InstrumentInfo> for Future {
    type Error = AvinError;

    fn try_from(value: InstrumentInfo) -> Result<Self, Self::Error> {
        let category = value.category();

        if category != Category::Future {
            return Err(AvinError::Value(format!(
                "invalid instrument info, expected 'Future' category, got {}",
                category
            )));
        }

        Ok(Self {
            info: value,
            charts: HashMap::new(),
        })
    }
}

impl InstrumentInfoView for Future {
    fn info(&self) -> &InstrumentInfo {
        &self.info
    }
}

impl HasCharts for Future {
    fn chart(&self, tf: TimeFrame) -> Option<&Chart> {
        self.charts.get(&tf)
    }
}
