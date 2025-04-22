use std::collections::HashMap;

use crate::Tinkoff;
use crate::core::{Action, Asset, Event, Share, TimeFrame, TradeList};
use crate::strategy::Every;
use crate::trader::work::Work;

pub struct Trader {
    in_tx: tokio::sync::mpsc::UnboundedSender<Action>,
    in_rx: tokio::sync::mpsc::UnboundedReceiver<Action>,

    works: HashMap<String, tokio::sync::mpsc::UnboundedSender<Event>>,
    trades: TradeList,
}

impl Trader {
    pub fn new() -> Self {
        let (in_tx, in_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            in_tx,
            in_rx,
            works: HashMap::new(),
            trades: TradeList::new("Trader_unittest"),
        }
    }

    pub async fn start(&mut self) {
        log::info!(":: Trader load trade list");

        log::info!(":: Trader load broker");
        let mut broker = Tinkoff::new().await;
        let mut broker_rx = broker.get_receiver();
        let broker_tx = broker.get_sender();

        log::info!(":: Trader load shares");
        let mut share = Share::from_str("moex_share_vtbr").unwrap();
        self.load_charts(&mut share);
        broker.subscribe_bar(&share.iid()).await.unwrap();

        log::info!(":: Trader load strategys");
        let account = broker.get_account("Agni").await.unwrap();
        let sender = self.in_tx.clone();
        let strategy = Every::new(sender, account, share.iid());

        log::info!(":: Trader start work");
        let mut work = Work::new(share);
        work.add_strategy(strategy);
        self.works.insert(work.figi().clone(), work.get_sender());
        let _ = tokio::spawn(async move { work.start().await });

        log::info!(":: Trader start");
        let _ =
            tokio::spawn(
                async move { broker.start_marketdata_stream().await },
            );

        // process events from broker
        while let Ok(e) = broker_rx.recv().await {
            log::info!("Trader receive {e}");
            self.works.get(e.figi()).unwrap().send(e).unwrap();

            // process actions from strategys
            while let Ok(a) = self.in_rx.try_recv() {
                match a {
                    Action::TradeClosed(trade) => {
                        log::info!("Trader get {}", trade);
                        self.trades.add_trade(trade);
                    }
                    other => broker_tx.send(other).unwrap(),
                }
            }
        }
    }

    // private
    // fn init(&mut self) {}
    fn load_charts(&mut self, share: &mut Share) {
        log::info!(":: Trader load charts {share}");

        let timeframes = vec![
            TimeFrame::new("1M"),
            TimeFrame::new("10M"),
            TimeFrame::new("1H"),
            TimeFrame::new("D"),
            TimeFrame::new("W"),
            TimeFrame::new("M"),
        ];

        for tf in timeframes.iter() {
            log::info!("   {}", tf.name());
            share.load_chart(&tf).unwrap();
        }
    }
}
