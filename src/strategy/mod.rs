use std::panic;

use crate::{
    Account, Action, Asset, Direction, IID, MarketOrder, Order,
    PostOrderAction, Share, TimeFrame, Trade, TradeType,
};
use chrono::Timelike;

#[derive(Debug)]
pub struct Every {
    name: String,
    trader: tokio::sync::mpsc::UnboundedSender<Action>,
    account: Account,
    iid: IID,

    last_ts: i64,
    trade: Option<Trade>,
}
impl Every {
    pub fn new(
        trader: tokio::sync::mpsc::UnboundedSender<Action>,
        account: Account,
        iid: IID,
    ) -> Self {
        let name = "Every".to_string();

        let last_ts = 0;

        Self {
            name,
            trader,
            account,
            iid,

            last_ts,
            trade: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub async fn process(&mut self, share: &Share) {
        let chart = share.chart(&TimeFrame::new("1M")).unwrap();
        let bar = chart.now().unwrap();
        if bar.ts_nanos == self.last_ts {
            return;
        }

        // сохранить время последней обработки
        self.last_ts = bar.ts_nanos.clone();

        // если четное количество минут
        if bar.dt().minute() % 2 == 0 {
            self.create_trade();
            self.buy().await;
        }
        // если нечетное количество минут
        else {
            if self.trade.is_some() {
                self.sell().await;
            }
        }
    }

    // private
    fn create_trade(&mut self) {
        let trade = Trade::new(
            self.last_ts,
            self.name(),
            TradeType::Long,
            self.iid.clone(),
        );
        self.trade = Some(Trade::New(trade));
    }
    async fn post_buy_order(&self) -> Order {
        log::info!("Buy!");

        let order = MarketOrder::new(Direction::Buy, 1);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        let (tx, rx) = tokio::sync::oneshot::channel();

        let a = PostOrderAction::new(
            self.account.clone(),
            self.iid.clone(),
            order,
            tx,
        );
        let a = Action::Post(a);
        self.trader.send(a).unwrap();

        log::info!("Await buy!");
        match rx.await {
            Ok(order) => {
                log::info!("Strategy rx {}", order);
                order
            }
            Err(err) => {
                log::error!("Strategy rx {:?}", err);
                panic!()
            }
        }
    }
    async fn post_sell_order(&self) -> Order {
        log::info!("Sell!");

        let order = MarketOrder::new(Direction::Sell, 1);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        let (tx, rx) = tokio::sync::oneshot::channel();

        let a = PostOrderAction::new(
            self.account.clone(),
            self.iid.clone(),
            order,
            tx,
        );
        let a = Action::Post(a);

        self.trader.send(a).unwrap();

        log::info!("Await sell!");
        match rx.await {
            Ok(order) => {
                log::info!("Strategy rx {}", order);
                order
            }
            Err(err) => {
                log::error!("Strategy rx {:?}", err);
                panic!()
            }
        }
    }
    async fn buy(&mut self) {
        let order = self.post_buy_order().await;
        assert!(order.is_filled());

        let trade = self.trade.take().unwrap();
        match trade {
            Trade::New(trade) => {
                let trade = trade.open(order);
                let trade = Trade::Opened(trade);
                self.trade = Some(trade);
                log::info!("Trade opened!");
            }
            _ => panic!("????"),
        }
    }
    async fn sell(&mut self) {
        let order = self.post_sell_order().await;
        assert!(order.is_filled());

        let trade = self.trade.take().unwrap();
        match trade {
            Trade::Opened(mut trade) => {
                trade.add_order(order);
                let trade = trade.close();
                let trade = Trade::Closed(trade);
                let a = Action::TradeClosed(trade);

                self.trader.send(a).unwrap();
                self.trade = None;
                log::info!("Trade closed!");
            }
            _ => panic!("????"),
        }
    }
}
