/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DEFAULT_BARS_COUNT;
use crate::core::chart::Chart;
use crate::core::event::{BarEvent, Event};
use crate::core::iid::IID;
use crate::core::timeframe::TimeFrame;
use chrono::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Asset {
    iid: IID,
    charts: HashMap<TimeFrame, Chart>,
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}
impl Asset {
    pub fn from_iid(iid: IID) -> Asset {
        let charts = HashMap::new();

        // let (sender, receiver) = mpsc::channel(2);
        let (sender, receiver) = mpsc::unbounded_channel();

        let asset = Asset {
            iid,
            charts,
            sender,
            receiver,
        };

        asset
    }
    pub fn from_str(s: &str) -> Result<Asset, &'static str> {
        let iid = IID::from(s)?;

        Ok(Asset::from_iid(iid))
    }
    pub fn to_string(&self) -> String {
        self.iid.to_string()
    }

    pub fn iid(&self) -> IID {
        self.iid.clone()
    }
    pub fn exchange(&self) -> &String {
        &self.iid.exchange
    }
    pub fn category(&self) -> &String {
        &self.iid.category
    }
    pub fn ticker(&self) -> &String {
        &self.iid.ticker
    }
    pub fn sender(&self) -> mpsc::UnboundedSender<Event> {
        self.sender.clone()
    }
    pub fn chart(&self, tf: &TimeFrame) -> Option<&Chart> {
        self.charts.get(tf)
    }

    pub fn load_chart(
        &mut self,
        tf: &TimeFrame,
    ) -> Result<&Chart, &'static str> {
        let end = Utc::now();
        let begin = end - tf.timedelta() * DEFAULT_BARS_COUNT;

        return self.load_chart_period(tf, &begin, &end);
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
    pub fn path(&self) -> PathBuf {
        self.iid.path()
    }

    pub async fn listen(&mut self) {
        while let Some(e) = self.receiver.recv().await {
            println!("got = {:?}", e);
            self.receive(e);
        }
    }

    pub fn receive(&mut self, e: Event) {
        match e {
            Event::Bar(bar_event) => self.receive_bar_event(bar_event),
        }
    }

    fn receive_bar_event(&mut self, e: BarEvent) {
        let tf = e.tf.name().as_str();
        match tf {
            "1M" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "5M" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "10M" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "1H" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "D" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "W" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            "M" => self.charts.get_mut(&e.tf).unwrap().receive(e),
            _ => todo!(),
        };
    }
}
impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Asset={} {} {}",
            self.exchange(),
            self.category(),
            self.ticker()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_chart() {
        let mut asset = Asset::from_str("moex_share_sber").unwrap();
        let tf = TimeFrame::new("1H");
        let begin = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let chart = asset.load_chart_period(&tf, &begin, &end).unwrap();

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
        let mut asset = Asset::from_str("moex_share_sber").unwrap();
        let tf = TimeFrame::new("D");

        let chart = asset.load_chart(&tf).unwrap();
        assert_eq!(chart.tf(), &tf);

        assert!(chart.bars().len() > 1000);
        assert!(chart.bars().len() < 5000);
    }
}
