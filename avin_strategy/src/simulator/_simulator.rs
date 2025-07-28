/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::VecDeque;

use avin_core::{Asset, Bar, Iid, Manager, MarketData};
use chrono::{DateTime, Utc};

pub struct Simulator {
    asset: Asset,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    bars_1m: VecDeque<Bar>,
}
impl Simulator {
    pub fn new(iid: &Iid, begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let df = Manager::load(iid, MarketData::BAR_1M, begin, end).unwrap();
        let vec_bars = Bar::from_df(&df).unwrap();

        Self {
            asset: Asset::from_iid(iid.clone()),
            begin,
            end,
            bars_1m: VecDeque::from(vec_bars),
        }
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
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

    pub fn start(&mut self) {
        //
    }
}
