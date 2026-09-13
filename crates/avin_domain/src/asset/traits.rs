// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_core::{Price, Quantity};

use crate::{
    Category, Chart, Exchange, InstrumentId, InstrumentInfo, Ticker,
    TimeFrame,
};

pub trait InstrumentInfoView {
    fn info(&self) -> &InstrumentInfo;

    fn iid(&self) -> InstrumentId {
        self.info().iid()
    }

    fn exchange(&self) -> Exchange {
        self.info().exchange()
    }

    fn category(&self) -> Category {
        self.info().category()
    }

    fn ticker(&self) -> Ticker {
        self.info().ticker()
    }

    fn name(&self) -> &str {
        self.info().name()
    }

    fn price_step(&self) -> Price {
        self.info().price_step()
    }

    fn lot_size(&self) -> Quantity {
        self.info().lot_size()
    }
}

pub trait HasCharts {
    fn chart(&self, tf: TimeFrame) -> Option<&Chart>;
}
