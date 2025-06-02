/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

use chrono::prelude::*;

use crate::BarEvent;
use crate::Chart;
use crate::Cmd;
use crate::DATA_DIR;
use crate::TicEvent;
use crate::TimeFrame;
use crate::Usr;
use crate::conf::DEFAULT_BARS_COUNT;
use crate::data::IID;
use crate::data::Manager;

#[derive(Debug)]
pub struct Share {
    iid: IID,
    charts: HashMap<TimeFrame, Chart>,
}

impl Share {
    pub fn all() -> Vec<Share> {
        let mut shares: Vec<Share> = Vec::new();

        // shares dir path
        let mut dir_path = std::path::PathBuf::new();
        dir_path.push(&DATA_DIR);
        dir_path.push("MOEX");
        dir_path.push("SHARE");

        // shares dirs: dir name == ticker
        let dirs = Cmd::get_dirs(&dir_path).unwrap();
        if dirs.is_empty() {
            log::warn!("Shares not found! Dir empty: {}", dir_path.display());
            return shares;
        }

        // create shares from dir name (ticker)
        for dir in dirs.iter() {
            let ticker = Cmd::name(dir).unwrap();
            let s = format!("MOEX_SHARE_{}", ticker);
            let share = Share::new(&s).unwrap();
            shares.push(share);
        }

        shares
    }
    pub fn new(s: &str) -> Result<Share, &'static str> {
        let iid = Manager::find(s)?;
        let share = Share::from_iid(iid);

        Ok(share)
    }
    pub fn from_iid(iid: IID) -> Self {
        assert!(iid.category() == "SHARE");

        Self {
            iid,
            charts: HashMap::new(),
        }
    }
    pub fn from_info(info: HashMap<String, String>) -> Share {
        let iid = IID::new(info);
        let share = Share::from_iid(iid);

        share
    }

    // identification
    pub fn iid(&self) -> &IID {
        &self.iid
    }
    pub fn exchange(&self) -> &String {
        &self.iid.exchange()
    }
    pub fn category(&self) -> &String {
        self.iid.category()
    }
    pub fn ticker(&self) -> &String {
        &self.iid.ticker()
    }
    pub fn figi(&self) -> &String {
        &self.iid.figi()
    }
    pub fn info(&self) -> &HashMap<String, String> {
        &self.iid.info()
    }
    pub fn path(&self) -> PathBuf {
        self.iid.path()
    }

    // chart
    pub fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        self.charts.get(tf)
    }
    pub fn chart_mut(&mut self, tf: &TimeFrame) -> Option<&mut Chart> {
        self.charts.get_mut(tf)
    }
    pub fn load_chart(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&Chart, &'static str> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;

        return self.load_chart_period(tf, &begin, &end);
    }
    pub fn load_chart_mut(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&mut Chart, &'static str> {
        // let end = Utc::now();
        // let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;
        let begin = Usr::date("2025-03-10");
        let end = Usr::date("2025-03-13");

        self.load_chart_period(tf, &begin, &end).unwrap();

        Ok(self.charts.get_mut(tf).unwrap())
    }
    pub fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str> {
        let chart = Chart::load(&self.iid, tf, begin, end)?;
        self.charts.insert(tf.clone(), chart);

        Ok(self.charts[tf].as_ref())
    }
    pub fn load_chart_empty(&mut self, tf: &TimeFrame) -> &Chart {
        let chart = Chart::empty(&self.iid, &tf);
        self.charts.insert(tf.clone(), chart);

        self.charts[tf].as_ref()
    }

    // events
    pub fn bar_event(&mut self, e: BarEvent) {
        let chart = self.charts.get_mut(&e.tf).unwrap();
        chart.swallow_bar(e.bar);
    }
    pub fn tic_event(&mut self, _e: TicEvent) {
        todo!();
    }
}
impl std::fmt::Display for Share {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Share={} {}", self.exchange(), self.ticker())
    }
}
impl Hash for Share {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.figi().hash(state);
    }
}
impl PartialEq for Share {
    fn eq(&self, other: &Self) -> bool {
        self.figi() == other.figi()
    }
}
impl Eq for Share {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_from_str() {
        let share = Share::new("moex_share_sber").unwrap();
        assert_eq!(share.exchange(), "MOEX");
        assert_eq!(share.category(), "SHARE");
        assert_eq!(share.ticker(), "SBER");
        assert_eq!(share.figi(), "BBG004730N88");
    }
    #[test]
    fn load_chart() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::H1;
        let begin = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let chart = share.load_chart_period(&tf, &begin, &end).unwrap();

        assert_eq!(chart.tf(), &tf);
        assert_eq!(
            chart.first().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 3, 6, 0, 0).unwrap()
        );
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2025, 1, 31, 20, 0, 0).unwrap()
        );
    }
    #[test]
    fn load_chart_no_args() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::Day;

        let chart = share.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
