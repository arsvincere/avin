/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use avin_connect::Tinkoff;
use avin_core::{Action, Asset, Event, TimeFrame, TradeList};
use avin_strategy::{BigTrendShort, Strategy};
use avin_utils::CFG;

use super::work::Work;

pub struct Trader {
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    action_rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
    event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,

    works: HashMap<String, tokio::sync::mpsc::UnboundedSender<Event>>,
    trades: TradeList,
}
impl Default for Trader {
    fn default() -> Self {
        Trader::new()
    }
}
impl Trader {
    pub fn new() -> Self {
        let (action_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            action_tx,
            action_rx,
            event_tx,
            event_rx,
            works: HashMap::new(),
            trades: TradeList::new("Trader_unittest"),
        }
    }

    pub async fn start(&mut self) {
        log::info!(":: Trader start");

        log::info!("Load broker");
        let sender = self.event_tx.clone();
        let mut broker = Tinkoff::new(sender);
        broker.connect().await.unwrap();
        broker.create_marketdata_stream().await.unwrap();
        broker.create_transactions_stream().await.unwrap();
        let broker_tx = broker.get_sender();
        let sender = self.action_tx.clone();
        let account = broker.get_account("Agni").await.unwrap();

        for node in CFG.trader.work_list.iter() {
            // for (str_iid, strategy_names) in node {
            // log::info!("Load asset {str_iid}");
            let mut asset = Asset::new(&node.iid).unwrap();
            log::info!("Load asset {asset}");
            load_charts(&mut asset);
            broker
                .subscribe_bar(asset.iid(), &TimeFrame::M1)
                .await
                .unwrap();

            // load and init strategys
            let mut strategys = Vec::new();
            for name in &node.strategy {
                log::info!("- load strategy {name}");
                let mut strategy = BigTrendShort::default(); // TODO: фабрика
                strategy.init(sender.clone(), account.clone(), &mut asset);
                strategys.push(strategy);
            }

            // create work, add strategys
            let mut work = Work::new(asset);
            for strategy in strategys {
                work.add_strategy(strategy);
            }

            log::info!("- start work");
            self.works.insert(work.figi().clone(), work.get_sender());
            tokio::spawn(async move { work.start().await });
        }
        // }

        log::info!("Start broker loop");
        tokio::spawn(async move { start_broker(broker).await });

        log::info!("Start main loop");
        loop {
            // await events from broker -> send to work (asset & strategy)
            if let Some(e) = self.event_rx.recv().await {
                // log::debug!("Trader receive {e}");
                let work = self.works.get(e.figi()).unwrap();
                work.send(e).unwrap();
            };

            // process actions from strategys
            while let Ok(a) = self.action_rx.try_recv() {
                // log::debug!("Trader get {a}");
                match a {
                    Action::TradeClosed(trade) => {
                        self.trades.add(trade);
                    }
                    other => broker_tx.send(other).unwrap(),
                }
            }
        }
    }
}

fn load_charts(asset: &mut Asset) {
    log::info!("- load charts");

    for tf in TimeFrame::all() {
        asset.load_chart_empty(tf);
    }
}
async fn start_broker(mut broker: Tinkoff) {
    broker.start().await
}
