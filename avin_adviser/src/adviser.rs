/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_analyse::TrendAnalytic;
use avin_connect::Tinkoff;
use avin_core::{
    Action, AssetList, Event, ExtremumIndicator, GetBarsAction, MarketData,
    Source, StreamAction, TimeFrame,
};
use avin_utils::{Informer, Notice, NoticePriority};

use crate::Condition;
use chrono::Utc;

pub struct Adviser {
    asset_list: AssetList,
    conditions: Vec<Box<dyn Condition>>,
}
impl Adviser {
    pub fn new(asset_list: AssetList) -> Self {
        Self {
            asset_list,
            conditions: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: impl Condition) {
        self.conditions.push(Box::new(condition));
    }

    pub async fn start(&mut self) {
        log::info!("Load charts");
        for asset in self.asset_list.assets_mut().iter_mut() {
            for tf in TimeFrame::all() {
                asset.load_chart(Source::TINKOFF, tf).unwrap();

                let chart = asset.chart_mut(tf).unwrap();
                ExtremumIndicator::init(chart);
                TrendAnalytic::init(chart);
            }
        }

        let (broker_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();
        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

        log::info!("Load broker");
        let mut broker = Tinkoff::new(action_rx, event_tx);
        broker.connect().await.unwrap();
        broker.create_marketdata_stream().await.unwrap();
        tokio::spawn(async move { broker.start().await });

        log::info!("Get new historical bars");
        for asset in self.asset_list.assets_mut().iter_mut() {
            let iid = asset.iid().clone();
            let tf = TimeFrame::M1;
            let from = asset.chart(tf).unwrap().last().unwrap().dt();
            let till = Utc::now();
            let (tx, rx) = tokio::sync::oneshot::channel();
            let action = GetBarsAction::new(iid, tf, from, till, tx);
            let action = Action::GetBars(action);
            broker_tx.send(action).unwrap();

            let bars = match rx.await {
                Ok(bars) => bars,
                Err(_) => todo!(),
            };
            for bar in bars.iter() {
                for tf in TimeFrame::all() {
                    let chart = asset.chart_mut(tf).unwrap();
                    chart.add_bar(*bar);
                }
            }
        }

        log::info!("Subscribe assets");
        let mut instruments = Vec::new();
        for asset in self.asset_list.assets().iter() {
            let iid = asset.iid().clone();
            instruments.push(iid);
        }
        let a = Action::Subscribe(StreamAction::new(
            instruments,
            vec![MarketData::BAR_1M, MarketData::TIC],
        ));
        broker_tx.send(a).unwrap();

        log::info!("Start main loop");
        // await events from broker -> send to asset
        while let Some(e) = event_rx.recv().await {
            let figi = e.figi();
            let asset = self.asset_list.find_figi_mut(figi).unwrap();

            match e {
                Event::Bar(e) => asset.bar_event(e),
                Event::Tic(e) => asset.tic_event(e),
                Event::OrderBook(_e) => todo!(),
                Event::Order(_) => continue,
            }

            // Тут теперь когда ассет обновлен можно применять к
            // нему условие и выдавать уведомление
            for condition in self.conditions.iter_mut() {
                if let Some(notice) = condition.apply(asset) {
                    log::info!("{notice:#?}");
                    Informer::notify(notice);
                }
            }
        }

        // цикл какого то хрена закончился...
        let notice =
            Notice::new("Цикл капут!", "Все пиздец!", NoticePriority::Critical);
        Informer::notify(notice);
    }
    pub fn asset_list(&self) -> &AssetList {
        &self.asset_list
    }
}
