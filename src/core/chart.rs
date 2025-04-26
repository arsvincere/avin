/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::bar::Bar;
use crate::core::timeframe::TimeFrame;
use crate::data::IID;
use crate::data::Manager;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Chart {
    iid: IID,
    tf: TimeFrame,
    bars: Vec<Bar>,
    now: Option<Bar>,
}
impl Chart {
    pub fn new(iid: &IID, tf: &TimeFrame, bars: Vec<Bar>) -> Self {
        Self {
            iid: iid.clone(),
            tf: tf.clone(),
            bars,
            now: None,
        }
    }
    pub fn empty(iid: &IID, tf: &TimeFrame) -> Self {
        Self::new(iid, tf, Vec::new())
    }
    pub fn load(
        iid: &IID,
        tf: &TimeFrame,
        begin: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        match Manager::request(&iid, &tf.to_market_data(), begin, end) {
            Ok(df) => {
                let bars = Bar::from_df(df).unwrap();
                let chart = Self::new(iid, tf, bars);

                Ok(chart)
            }
            Err(e) => {
                log::warn!("{}, using empty chart", e);
                Ok(Self::empty(iid, tf))
            }
        }
    }

    /// Return chart instrument id
    pub fn iid(&self) -> &IID {
        &self.iid
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

    /// Swallow new bar.
    /// Depending on datetime of 'new_bar' this function do:
    ///  - only update real-time bar
    ///  - add new historical bar and update real-time
    pub fn swallow_bar(&mut self, new_bar: Bar) {
        match self.now.take() {
            None => {
                // receive first real time bar
                self.now = Some(new_bar);
            }
            Some(old_bar) => {
                // only update now bar
                if old_bar.ts_nanos == new_bar.ts_nanos {
                    self.now = Some(new_bar);
                // new historical bar and update now bar
                } else if old_bar.ts_nanos < new_bar.ts_nanos {
                    self.bars.push(old_bar);
                    self.now = Some(new_bar);
                }
                // Тинькофф бывает прокидывает в дата стриме
                // исторические бары законченные уже после новых
                // реал-тайм баров. По факту же - последний
                // реал-тайм бар который был в потоке как незаконченный
                // он равен этому законченному историческому бару
                // так что в моем алгоритме приема баров он не нужен, игнор.
                else if old_bar.ts_nanos > new_bar.ts_nanos {
                    return;
                }
            }
        }
    }
}
impl AsRef<Chart> for Chart {
    fn as_ref(&self) -> &Chart {
        &self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Usr;
    use std::collections::HashMap;

    #[test]
    fn new() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);

        let tf = TimeFrame::new("D");
        let begin = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let df = Manager::request(&iid, &tf.to_market_data(), &begin, &end)
            .unwrap();
        let bars = Bar::from_df(df).unwrap();

        let chart = Chart::new(&iid, &tf, bars);
        assert_eq!(chart.iid, iid);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 256);
        assert!(chart.now.is_none());
    }
    #[test]
    fn empty() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);
        let tf = TimeFrame::new("D");

        let chart = Chart::empty(&iid, &tf);
        assert_eq!(chart.tf, tf);
        assert_eq!(chart.bars.len(), 0);
        assert!(chart.now.is_none());
    }
    #[test]
    fn load() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        let iid = IID::new(info);
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
