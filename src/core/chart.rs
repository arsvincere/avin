/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::bar::Bar;
use crate::core::event::BarEvent;
use crate::core::iid::IID;
use crate::core::timeframe::TimeFrame;
use crate::data::Manager;
use bitcode::{Decode, Encode};
use chrono::prelude::*;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Chart {
    tf: TimeFrame,
    bars: Vec<Bar>,
    now: Option<Bar>,
}

impl Chart {
    pub fn new(_asset: &IID, tf: &TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            tf: tf.clone(),
            bars,
            now: None,
        }
    }
    pub fn empty(_asset: &IID, tf: TimeFrame) -> Self {
        Self {
            tf,
            bars: Vec::new(),
            now: None,
        }
    }
    pub fn load(
        iid: &IID,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        let df = Manager::request(&iid, &tf.to_market_data(), begin, end)?;
        let bars = Bar::from_df(df).unwrap();

        let chart = Self {
            tf: tf.clone(),
            bars,
            now: None,
        };
        Ok(chart)
    }

    /// Return chart timeframe
    pub fn tf(&self) -> &TimeFrame {
        &self.tf
    }
    /// Return bars of chart
    pub fn bars(&self) -> &Vec<Bar> {
        &self.bars
    }
    /// Return fist historical bar of chart
    pub fn first(&self) -> Option<&Bar> {
        self.bars.first()
    }
    /// Return last historical bar of chart
    pub fn last(&self) -> Option<&Bar> {
        self.bars.last()
    }
    /// Return real-time bar of chart
    pub fn now(&self) -> Option<&Bar> {
        self.now.as_ref()
    }
    /// Return real-time bar close
    pub fn last_price(&self) -> Option<f64> {
        match &self.now {
            Some(bar) => Some(bar.c),
            None => None,
        }
    }

    pub fn receive(&mut self, e: BarEvent) {
        println!("chart got = {:?}", e);
    }
}
impl AsRef<Chart> for Chart {
    fn as_ref(&self) -> &Chart {
        &self
    }
}

#[cfg(test)]
mod tests {
    use crate::Usr;

    use super::*;

    #[test]
    fn new() {
        let iid = IID::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let df = Manager::request(&iid, &tf.to_market_data(), &begin, &end)
            .unwrap();
        let bars = Bar::from_df(df).unwrap();

        let chart = Chart::new(&iid, &tf, bars);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 256);
        assert!(chart.now.is_none());
    }
    #[test]
    fn empty() {
        let iid = IID::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = Chart::empty(&iid, tf.clone());
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
        assert!(chart.now.is_none());
    }
    #[test]
    fn load() {
        let iid = IID::from("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");
        let begin = Usr::date("2023-08-01");
        let end = Usr::date("2023-09-01");

        let chart = Chart::load(&iid, &tf, &begin, &end).unwrap();
        assert_eq!(chart.tf(), &tf);
        assert_eq!(chart.bars().len(), 23);
        assert!(chart.now().is_none());

        assert_eq!(chart.first().unwrap().dt(), begin);
        assert_eq!(
            chart.last().unwrap().dt(),
            Utc.with_ymd_and_hms(2023, 8, 30, 21, 0, 0).unwrap(),
        )
    }
}
