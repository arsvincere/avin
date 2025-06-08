/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};

use crate::{BarEvent, Chart, IID, Manager, TicEvent, TimeFrame};

use super::Share;

#[derive(Debug)]
pub enum Asset {
    SHARE(Share),
}
impl Asset {
    // build
    pub fn new(s: &str) -> Result<Asset, &'static str> {
        let iid = Manager::find(s)?;
        let share = Share::from_iid(iid);
        let asset = Asset::SHARE(share);

        Ok(asset)
    }
    pub fn from_iid(iid: IID) -> Self {
        assert!(iid.category() == "SHARE");
        let share = Share::from_iid(iid);

        Asset::SHARE(share)
    }
    pub fn from_csv(line: &str) -> Result<Self, String> {
        // line example: 'MOEX;SHARE;SBER;'
        let parts: Vec<&str> = line.split(';').collect();
        let exchange = parts.get(0).expect("invalid line");
        let category = parts.get(1).expect("invalid line");
        let ticker = parts.get(2).expect("invalid line");

        let query = format!("{}_{}_{}", exchange, category, ticker);
        let result = Manager::find(&query);

        match result {
            Ok(iid) => {
                let asset = Asset::from_iid(iid);
                return Ok(asset);
            }
            Err(why) => {
                let msg = format!("fail create from csv {}, {}", line, why);
                return Err(msg);
            }
        }
    }

    // identification
    pub fn iid(&self) -> &IID {
        match self {
            Self::SHARE(share) => share.iid(),
        }
    }
    pub fn exchange(&self) -> &String {
        match self {
            Self::SHARE(share) => share.exchange(),
        }
    }
    pub fn category(&self) -> &String {
        match self {
            Self::SHARE(share) => share.category(),
        }
    }
    pub fn ticker(&self) -> &String {
        match self {
            Self::SHARE(share) => share.ticker(),
        }
    }
    pub fn figi(&self) -> &String {
        match self {
            Self::SHARE(share) => share.figi(),
        }
    }
    pub fn info(&self) -> &HashMap<String, String> {
        match self {
            Self::SHARE(share) => share.info(),
        }
    }
    pub fn path(&self) -> PathBuf {
        match self {
            Self::SHARE(share) => share.path(),
        }
    }

    // chart
    pub fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        match self {
            Self::SHARE(share) => share.chart(tf),
        }
    }
    pub fn chart_mut(&mut self, tf: &TimeFrame) -> Option<&mut Chart> {
        match self {
            Self::SHARE(share) => share.chart_mut(tf),
        }
    }
    pub fn load_chart(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&Chart, &'static str> {
        match self {
            Self::SHARE(share) => share.load_chart(tf),
        }
    }
    pub fn load_chart_mut(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&mut Chart, &'static str> {
        match self {
            Self::SHARE(share) => share.load_chart_mut(tf),
        }
    }
    pub fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str> {
        match self {
            Self::SHARE(share) => share.load_chart_period(tf, begin, end),
        }
    }
    pub fn load_chart_empty(&mut self, tf: &TimeFrame) -> &Chart {
        match self {
            Self::SHARE(share) => share.load_chart_empty(tf),
        }
    }

    // event
    pub fn bar_event(&mut self, e: BarEvent) {
        match self {
            Self::SHARE(share) => share.bar_event(e),
        }
    }
    pub fn tic_event(&mut self, _e: TicEvent) {
        todo!();
    }
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SHARE(s) => write!(f, "Asset={}", s.iid()),
        }
    }
}
impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.figi() == other.figi()
    }
}
