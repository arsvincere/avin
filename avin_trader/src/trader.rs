/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use avin_connect::Tinkoff;
use avin_core::{
    Action, Asset, Event, GetAccountAction, MarketData, StreamAction,
    TimeFrame, TradeList,
};
use avin_strategy::{BigTrendShort, Strategy};
use avin_utils::CFG;

use super::work::Work;

pub struct Trader {
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
        Self {
            works: HashMap::new(),
            trades: TradeList::new("Trader_unittest"),
        }
    }

    pub async fn start(&mut self) {
        log::info!(":: Trader start");

        // channel from trader to broker (Action)
        let (trader_broker_action_tx, trader_broker_action_rx) =
            tokio::sync::mpsc::unbounded_channel();
        // channel from broker to trader (Event)
        let (broker_trader_event_tx, mut broker_trader_event_rx) =
            tokio::sync::mpsc::unbounded_channel();
        // channel from strategy to trader (Action)
        let (strategy_trader_action_tx, mut strategy_trader_action_rx) =
            tokio::sync::mpsc::unbounded_channel();

        log::info!("- load broker");
        let mut broker =
            Tinkoff::new(trader_broker_action_rx, broker_trader_event_tx);
        broker.connect().await.unwrap();
        tokio::spawn(async move { start_broker(broker).await });

        let (tx, rx) = tokio::sync::oneshot::channel();
        let a = Action::GetAccount(GetAccountAction::new("Agni", tx));
        trader_broker_action_tx.send(a).unwrap();
        let account = match rx.await {
            Ok(account) => account,
            Err(_) => todo!(),
        };

        for node in CFG.trader.work_list.iter() {
            log::info!("Load asset");
            let mut asset = Asset::new(&node.iid).unwrap();
            load_charts(&mut asset);

            // subscribe data stream
            let a = Action::Subscribe(StreamAction::new(
                asset.iid().clone(),
                vec![MarketData::BAR_1M],
            ));
            trader_broker_action_tx.send(a).unwrap();

            // load and init strategys
            let mut strategys = Vec::new();
            for name in &node.strategy {
                log::info!("- load strategy {name}");
                let mut strategy = BigTrendShort::default();
                strategy.init(
                    strategy_trader_action_tx.clone(),
                    account.clone(),
                    &mut asset,
                );
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

        log::info!("Start main loop");
        loop {
            // await events from broker -> send to work (asset & strategy)
            if let Some(e) = broker_trader_event_rx.recv().await {
                let work = self.works.get(e.figi()).unwrap();
                work.send(e).unwrap();
            };

            // process actions from strategys
            while let Ok(a) = strategy_trader_action_rx.try_recv() {
                // log::debug!("Trader get {a}");
                match a {
                    Action::TradeOpened(trade) => {
                        log::info!(":: Trade opened: {trade}")
                    }
                    Action::TradeClosed(trade) => {
                        self.trades.add(trade);
                    }
                    other => trader_broker_action_tx.send(other).unwrap(),
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
