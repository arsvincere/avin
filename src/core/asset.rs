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

use crate::conf::DEFAULT_BARS_COUNT;
use crate::data::Category;
use crate::data::IID;
use crate::data::Manager;

use super::BarEvent;
use super::Chart;
use super::TicEvent;
use super::TimeFrame;

pub trait Asset {
    fn iid(&self) -> &IID;
    fn exchange(&self) -> &String;
    fn category(&self) -> Category;
    fn ticker(&self) -> &String;
    fn figi(&self) -> &String;
    fn info(&self) -> &HashMap<String, String>;
    fn path(&self) -> PathBuf;

    fn chart(&self, tf: &TimeFrame) -> Option<&Chart>;
    fn mut_chart(&mut self, tf: &TimeFrame) -> Option<&mut Chart>;
    fn load_chart(&mut self, tf: &TimeFrame) -> Result<&Chart, &'static str>;
    fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str>;
    fn load_chart_empty(&mut self, tf: &TimeFrame) -> &Chart;
}

#[derive(Debug)]
pub struct Share {
    iid: IID,
    charts: HashMap<TimeFrame, Chart>,
}

impl Share {
    pub fn new(s: &str) -> Result<Share, &'static str> {
        let iid = Manager::find(s)?;
        let share = Share::from_iid(iid);

        Ok(share)
    }
    pub fn from_iid(iid: IID) -> Self {
        assert!(iid.category() == Category::SHARE);

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
    // pub fn to_string(&self) -> String {
    //     self.iid.to_string()
    // }

    pub fn bar_event(&mut self, e: BarEvent) {
        let chart = self.charts.get_mut(&e.tf).unwrap();
        chart.swallow_bar(e.bar);
    }
    pub fn tic_event(&mut self, _e: TicEvent) {
        todo!();
    }
}
impl Asset for Share {
    fn iid(&self) -> &IID {
        &self.iid
    }
    fn exchange(&self) -> &String {
        &self.iid.exchange()
    }
    fn category(&self) -> Category {
        self.iid.category()
    }
    fn ticker(&self) -> &String {
        &self.iid.ticker()
    }
    fn figi(&self) -> &String {
        &self.iid.figi()
    }
    fn info(&self) -> &HashMap<String, String> {
        &self.iid.info()
    }
    fn path(&self) -> PathBuf {
        self.iid.path()
    }

    fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        self.charts.get(tf)
    }
    fn mut_chart(&mut self, tf: &TimeFrame) -> Option<&mut Chart> {
        self.charts.get_mut(tf)
    }
    fn load_chart(&mut self, tf: &TimeFrame) -> Result<&Chart, &'static str> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;

        return self.load_chart_period(tf, &begin, &end);
    }
    fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str> {
        let chart = Chart::load(&self.iid, tf, begin, end)?;
        self.charts.insert(tf.clone(), chart);

        Ok(self.charts[tf].as_ref())
    }
    fn load_chart_empty(&mut self, tf: &TimeFrame) -> &Chart {
        let chart = Chart::empty(&self.iid, &tf);
        self.charts.insert(tf.clone(), chart);

        self.charts[tf].as_ref()
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
        assert_eq!(share.category(), Category::SHARE);
        assert_eq!(share.ticker(), "SBER");
        assert_eq!(share.figi(), "BBG004730N88");
    }
    #[test]
    fn load_chart() {
        let mut share = Share::new("moex_share_sber").unwrap();
        let tf = TimeFrame::new("1H");
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
        let tf = TimeFrame::new("D");

        let chart = share.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
