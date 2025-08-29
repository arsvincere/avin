/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::VecDeque;

use avin_analyse::TrendAnalytic;
use avin_core::{Asset, Bar, BarEvent, ExtremumIndicator, TimeFrame};
use avin_data::{Iid, Manager, MarketData};
use chrono::{DateTime, Utc};

pub struct Simulator {
    asset: Asset,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    bars_1m: VecDeque<Bar>,
}
impl Simulator {
    pub fn new(iid: &Iid, begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        // create asset and empty charts
        let mut asset = Asset::from_iid(iid.clone());
        for tf in TimeFrame::all() {
            asset.load_chart_empty(tf);
            let chart = asset.chart_mut(tf).unwrap();
            ExtremumIndicator::init(chart);
            TrendAnalytic::init(chart);
        }

        Self {
            asset,
            begin,
            end,
            bars_1m: load_bars(iid, begin, end),
        }
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }
    pub fn asset_mut(&mut self) -> &mut Asset {
        &mut self.asset
    }
    pub fn begin(&self) -> DateTime<Utc> {
        self.begin // DateTime has Copy trait
    }
    pub fn end(&self) -> DateTime<Utc> {
        self.end // DateTime has Copy trait
    }
    pub fn set_asset(&mut self, iid: &Iid) {
        self.asset = Asset::from_iid(iid.clone());
    }
    pub fn set_begin(&mut self, dt: DateTime<Utc>) {
        self.begin = dt
    }
    pub fn set_end(&mut self, dt: DateTime<Utc>) {
        self.end = dt
    }

    pub fn step(&mut self, n: usize) -> &Asset {
        let mut i = 0;
        while i != n {
            self.next_bar();
            i += 1;
        }

        &self.asset
    }
    pub fn next_bar(&mut self) -> Option<&Asset> {
        let bar = self.bars_1m.pop_front()?;
        let figi = self.asset.figi().clone();
        let e = BarEvent::new(figi, TimeFrame::M1, bar);
        self.asset.bar_event(e);

        Some(&self.asset)
    }
    pub fn restart(&mut self) {
        self.asset.clear();
        self.bars_1m = load_bars(self.asset.iid(), self.begin, self.end);
    }
}

fn load_bars(iid: &Iid, b: DateTime<Utc>, e: DateTime<Utc>) -> VecDeque<Bar> {
    let df = Manager::load(iid, MarketData::BAR_1M, b, e).unwrap();
    let vec_bars = Bar::from_df(&df).unwrap();

    VecDeque::from(vec_bars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use avin_utils::*;

    #[test]
    fn simulate_1m_next() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let begin = str_date_to_utc("2023-08-01");
        let end = str_date_to_utc("2023-09-01");
        let tf = TimeFrame::M1;

        let mut simulator = Simulator::new(&iid, begin, end);

        let chart = simulator.asset().chart(tf).unwrap();
        assert!(chart.now().is_none());

        simulator.next_bar();
        let chart = simulator.asset().chart(tf).unwrap();
        assert!(chart.now().is_some());

        for _i in 0..10 {
            simulator.next_bar();
        }

        let chart = simulator.asset().chart(tf).unwrap();
        let now_bar = chart.now().unwrap();
        let expect_dt = str_dt_to_utc("2023-08-01 10:09:00");
        let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
        let expect_bar =
            Bar::new(expect_ts, 267.52, 267.61, 266.84, 267.07, 1304400);
        assert_eq!(*now_bar, expect_bar);
    }
    #[test]
    fn simulate_1m_step() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let begin = str_date_to_utc("2023-08-01");
        let end = str_date_to_utc("2023-09-01");
        let tf = TimeFrame::M1;

        let mut simulator = Simulator::new(&iid, begin, end);

        let chart = simulator.asset().chart(tf).unwrap();
        assert!(chart.now().is_none());

        simulator.next_bar();
        let chart = simulator.asset().chart(tf).unwrap();
        assert!(chart.now().is_some());

        simulator.step(10);

        let chart = simulator.asset().chart(tf).unwrap();
        let now_bar = chart.now().unwrap();
        let expect_dt = str_dt_to_utc("2023-08-01 10:09:00");
        let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
        let expect_bar =
            Bar::new(expect_ts, 267.52, 267.61, 266.84, 267.07, 1304400);
        assert_eq!(*now_bar, expect_bar);
    }
    #[test]
    fn simulate_10m() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let begin = str_date_to_utc("2023-08-01");
        let end = str_date_to_utc("2023-09-01");
        let tf = TimeFrame::M10;

        let mut simulator = Simulator::new(&iid, begin, end);

        simulator.next_bar();
        simulator.next_bar();
        simulator.step(60);

        let chart = simulator.asset().chart(tf).unwrap();
        let now_bar = chart.now().unwrap();
        let expect_dt = str_dt_to_utc("2023-08-01 11:00:00");
        let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
        let expect_bar =
            Bar::new(expect_ts, 267.29, 267.35, 266.33, 266.5, 521050);
        assert_eq!(*now_bar, expect_bar);
    }
    #[test]
    fn simulate_1h() {
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let begin = str_date_to_utc("2023-08-01");
        let end = str_date_to_utc("2023-09-01");
        let tf = TimeFrame::H1;

        let mut simulator = Simulator::new(&iid, begin, end);

        simulator.next_bar();
        simulator.next_bar();
        simulator.step(120);

        let chart = simulator.asset().chart(tf).unwrap();
        let now_bar = chart.now().unwrap();
        let expect_dt = str_dt_to_utc("2023-08-01 12:00:00");
        let expect_ts = expect_dt.timestamp_nanos_opt().unwrap();
        let expect_bar =
            Bar::new(expect_ts, 267.89, 268.39, 267.83, 268.31, 245660);
        assert_eq!(*now_bar, expect_bar);
    }
}
