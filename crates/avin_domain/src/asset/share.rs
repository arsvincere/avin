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

pub struct Share {
    info: InstrumentInfo,
    charts: HashMap<TimeFrame, Chart>,
}

impl TryFrom<InstrumentInfo> for Share {
    type Error = AvinError;

    fn try_from(value: InstrumentInfo) -> Result<Self, Self::Error> {
        let category = value.category();

        if category != Category::Share {
            return Err(AvinError::Value(format!(
                "invalid instrument info, expected 'Share' category, got {}",
                category
            )));
        }

        Ok(Self {
            info: value,
            charts: HashMap::new(),
        })
    }
}

impl InstrumentInfoView for Share {
    fn info(&self) -> &InstrumentInfo {
        &self.info
    }
}

impl HasCharts for Share {
    fn chart(&self, tf: TimeFrame) -> Option<&Chart> {
        self.charts.get(&tf)
    }
}
