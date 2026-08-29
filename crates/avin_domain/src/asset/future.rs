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

#[cfg(test)]
mod tests {
    use crate::Exchange;

    use super::*;

    fn get_info(category: Category) -> InstrumentInfo {
        let mut info = HashMap::from([
            ("exchange".to_string(), "MOEX".to_string()),
            ("category".to_string(), "FUTURE".to_string()),
            ("ticker".to_string(), "IMOEXF".to_string()),
            ("figi".to_string(), "TEST_FIGI".to_string()),
            ("name".to_string(), "IMOEX Future".to_string()),
            ("lot".to_string(), "1".to_string()),
            ("step".to_string(), "0.5".to_string()),
        ]);

        info.insert("category".to_string(), category.to_string());

        InstrumentInfo::new(info).unwrap()
    }

    #[test]
    fn valid_info() {
        let future = Future::try_from(get_info(Category::Future)).unwrap();

        assert_eq!(future.exchange(), Exchange::Moex);
        assert_eq!(future.category(), Category::Future);
        assert_eq!(future.ticker().to_string(), "IMOEXF");
        assert_eq!(future.name(), "IMOEX Future");

        assert!(future.chart(TimeFrame::Day).is_none());
    }

    #[test]
    fn invalid_category() {
        let result = Future::try_from(get_info(Category::Share));

        assert!(matches!(result, Err(AvinError::Value(_))));
    }
}
