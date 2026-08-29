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

#[cfg(test)]
mod tests {
    use crate::Exchange;

    use super::*;

    fn get_info(category: Category) -> InstrumentInfo {
        let mut info = HashMap::from([
            ("exchange".to_string(), "MOEX".to_string()),
            ("category".to_string(), "SHARE".to_string()),
            ("ticker".to_string(), "SBER".to_string()),
            ("figi".to_string(), "BBG004730N88".to_string()),
            ("name".to_string(), "Сбер Банк".to_string()),
            ("lot".to_string(), "1".to_string()),
            ("step".to_string(), "0.01".to_string()),
        ]);

        info.insert("category".to_string(), category.to_string());

        InstrumentInfo::new(info).unwrap()
    }

    #[test]
    fn valid_info() {
        let share = Share::try_from(get_info(Category::Share)).unwrap();

        assert_eq!(share.exchange(), Exchange::Moex);
        assert_eq!(share.category(), Category::Share);
        assert_eq!(share.ticker().to_string(), "SBER");
        assert_eq!(share.name(), "Сбер Банк");

        assert!(share.chart(TimeFrame::Day).is_none());
    }

    #[test]
    fn invalid_category() {
        let result = Share::try_from(get_info(Category::Future));

        assert!(matches!(result, Err(AvinError::Value(_))));
    }
}
