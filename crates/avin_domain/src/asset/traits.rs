// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

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

    fn figi(&self) -> &str {
        self.info().figi()
    }

    fn name(&self) -> &str {
        self.info().name()
    }

    fn lot(&self) -> u32 {
        self.info().lot()
    }

    fn step(&self) -> f64 {
        self.info().step()
    }
}

pub trait HasCharts {
    fn chart(&self, tf: TimeFrame) -> Option<&Chart>;
}
