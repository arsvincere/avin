/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DEFAULT_BARS_COUNT;
use crate::core::chart::Chart;
use crate::core::event::{BarEvent, Event};
use crate::core::timeframe::TimeFrame;
use crate::data::Category;
use crate::data::IID;
use crate::data::Manager;
use chrono::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub trait Asset {
    fn iid(&self) -> IID;
    fn exchange(&self) -> &String;
    fn category(&self) -> Category;
    fn ticker(&self) -> &String;
    fn figi(&self) -> &String;
    fn info(&self) -> &HashMap<String, String>;
    fn path(&self) -> PathBuf;

    fn chart(&self, tf: &TimeFrame) -> Option<&Chart>;
    fn load_chart(&mut self, tf: &TimeFrame) -> Result<&Chart, &'static str>;
    fn load_chart_period(
        &mut self,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<&Chart, &'static str>;
}

#[derive(Debug)]
pub struct Share {
    iid: IID,
    charts: HashMap<TimeFrame, Chart>,
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl Share {
    pub fn new(iid: IID) -> Share {
        let (sender, receiver) = mpsc::unbounded_channel();

        let share = Share {
            iid,
            charts: HashMap::new(),
            sender,
            receiver,
        };

        share
    }
    pub fn from_str(s: &str) -> Result<Share, &'static str> {
        let iid = Manager::find(s)?;
        let share = Share::new(iid);

        Ok(share)

        // let iid = IID::from(s)?;
        //
        // Ok(Share::from_iid(iid))
    }
    pub fn from_info(info: HashMap<String, String>) -> Share {
        let iid = IID::new(info);
        let share = Share::new(iid);

        share
    }
    pub fn to_string(&self) -> String {
        self.iid.to_string()
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }

    /// Run event loop - receiving new <Event>
    pub async fn start(&mut self) {
        while let Some(e) = self.receiver.recv().await {
            self.receive(e);
        }
    }

    fn receive(&mut self, e: Event) {
        match e {
            Event::Bar(bar_event) => self.receive_bar_event(bar_event),
            Event::Tic(_tic_event) => todo!(),
        }
    }
    fn receive_bar_event(&mut self, e: BarEvent) {
        self.charts.get_mut(&e.tf).unwrap().receive(e);
    }
}
impl Asset for Share {
    fn iid(&self) -> IID {
        self.iid.clone()
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
}
impl std::fmt::Display for Share {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Share={} {}", self.exchange(), self.ticker())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_from_str() {
        let share = Share::from_str("moex_share_sber").unwrap();
        assert_eq!(share.exchange(), "MOEX");
        assert_eq!(share.category(), Category::SHARE);
        assert_eq!(share.ticker(), "SBER");
        assert_eq!(share.figi(), "BBG004730N88");
    }
    #[test]
    fn load_chart() {
        let mut share = Share::from_str("moex_share_sber").unwrap();
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
        let mut share = Share::from_str("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = share.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
