/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use crate::core::{Action, Event, Share, TimeFrame, TradeList};
use crate::strategy::Strategy;
use crate::{Tinkoff, Work};

pub struct Trader {
    action_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    action_rx: tokio::sync::mpsc::UnboundedReceiver<Action>,
    event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,

    works: HashMap<String, tokio::sync::mpsc::UnboundedSender<Event>>,
    trades: TradeList,
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
        log::info!(":: Trader load broker");
        let sender = self.event_tx.clone();
        let mut broker = Tinkoff::new(sender);
        broker.connect().await.unwrap();
        broker.create_marketdata_stream().await.unwrap();
        broker.create_transactions_stream().await.unwrap();
        let broker_tx = broker.get_sender();

        log::info!(":: Trader load shares");
        let mut share = Share::new("moex_share_vtbr").unwrap();
        self.load_charts(&mut share);
        broker
            .subscribe_bar(&share.iid(), &TimeFrame::M1)
            .await
            .unwrap();

        log::info!(":: Trader load strategys");
        let sender = self.action_tx.clone();
        let account = broker.get_account("Agni").await.unwrap();
        let strategy = Strategy::new("Every", sender, account, share.iid());

        log::info!(":: Trader start work");
        let mut work = Work::new(share);
        work.add_strategy(strategy);
        self.works.insert(work.figi().clone(), work.get_sender());
        let _ = tokio::spawn(async move { work.start().await });

        log::info!(":: Trader start broker");
        let _ = tokio::spawn(async move { start_broker(broker) });

        log::info!(":: Trader start main loop");
        loop {
            // await events from broker -> send to work (asset & strategy)
            if let Some(e) = self.event_rx.recv().await {
                log::debug!("Trader receive {e}");
                let work = self.works.get(e.figi()).unwrap();
                work.send(e).unwrap();
            };

            // process actions from strategys
            while let Ok(a) = self.action_rx.try_recv() {
                log::debug!("Trader get {a}");
                match a {
                    Action::TradeClosed(trade) => {
                        self.trades.add(trade);
                    }
                    other => broker_tx.send(other).unwrap(),
                }
            }
        }
    }

    // private
    fn load_charts(&mut self, share: &mut Share) {
        log::info!(":: Trader load charts {share}");

        for tf in TimeFrame::all() {
            share.load_chart_empty(&tf);
        }
    }
}

async fn start_broker(mut broker: Tinkoff) {
    broker.start().await
}
