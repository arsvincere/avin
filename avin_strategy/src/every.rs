/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::panic;

use chrono::Timelike;

use avin_core::{
    Account, Action, Asset, Bar, Direction, Iid, MarketOrder, Order,
    OrderAction, OrderEvent, TimeFrame, Trade, TradeKind,
};

use crate::Strategy;

const NAME: &str = "Every";

type Trader = tokio::sync::mpsc::UnboundedSender<Action>;

#[derive(Debug, Default)]
enum Status {
    #[default]
    Observe,
    PostingBuy,
    Opening,
    Active,
    PostingSell,
    Closing,
}

#[derive(Debug, Default)]
pub struct Every {
    trader: Option<Trader>,
    account: Option<Account>,
    iid: Option<Iid>,

    status: Status,
    last_ts: i64,
    trade: Option<Trade>,
    buy_order: Option<Order>,
    sell_order: Option<Order>,
}
impl Strategy for Every {
    fn name(&self) -> &'static str {
        NAME
    }
    fn init(&mut self, trader: Trader, account: Account, iid: Iid) {
        self.trader = Some(trader);
        self.account = Some(account);
        self.iid = Some(iid);
        self.last_ts = 0;
    }
    fn process(&mut self, asset: &Asset) {
        let tf = TimeFrame::M1;
        let chart = asset.chart(&tf).unwrap();
        let bar = chart.now().unwrap();

        // log::debug!("Every.process {}", bar);
        if bar.ts_nanos == self.last_ts {
            return;
        }

        // сохранить время последней обработки
        self.last_ts = bar.ts_nanos;

        match self.status {
            Status::Observe => self.get_in(bar),
            Status::PostingBuy => (),
            Status::Opening => (),
            Status::Active => self.get_out(bar),
            Status::PostingSell => (),
            Status::Closing => (),
        }
    }
    fn order_event(&mut self, e: OrderEvent) {
        // log::debug!("Every.order_event: {}", e);

        match self.status {
            Status::PostingBuy => self.on_buy_event(e),
            Status::Opening => self.on_opening_event(e),
            Status::PostingSell => self.on_sell_event(e),
            Status::Closing => self.on_closing_event(e),
            Status::Observe => panic!(),
            Status::Active => panic!(),
        }
    }
}
impl Every {
    // private
    fn get_in(&mut self, bar: &Bar) {
        // log::debug!("Every.get_in {}", bar);

        // если четное количество минут
        if bar.dt().minute() % 2 == 0 {
            self.buy();
        }
    }
    fn get_out(&mut self, bar: &Bar) {
        // log::debug!("Every.get_out {}", bar);

        // если нечетное количество минут
        if bar.dt().minute() % 2 != 0 {
            self.sell();
        }
    }
    fn buy(&mut self) {
        // log::debug!("Buy!");
        self.create_trade();
        self.send_buy_order();
    }
    fn sell(&mut self) {
        // log::debug!("Sell!");
        if self.trade.is_none() {
            return;
        }
        self.send_sell_order();
    }
    fn create_trade(&mut self) {
        // log::debug!("Create trade!");
        let trade = Trade::new(
            self.last_ts,
            self.name(),
            TradeKind::Long,
            self.iid.clone().unwrap(),
        );
        self.trade = Some(Trade::New(trade));
    }
    fn send_buy_order(&mut self) {
        // log::debug!("Send buy order!");

        let order = MarketOrder::new(Direction::Buy, 1);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        self.buy_order = Some(order.clone());

        let a = OrderAction::new(
            self.account.clone().unwrap(),
            self.iid.clone().unwrap(),
            self.name(),
            order,
        );
        let a = Action::Post(a);

        self.trader.as_ref().unwrap().send(a).unwrap();
        self.status = Status::PostingBuy;
    }
    fn send_sell_order(&mut self) {
        // log::debug!("Send sell order!");

        let order = MarketOrder::new(Direction::Sell, 1);
        let order = MarketOrder::New(order);
        let order = Order::Market(order);
        self.sell_order = Some(order.clone());

        let a = OrderAction::new(
            self.account.clone().unwrap(),
            self.iid.clone().unwrap(),
            self.name(),
            order,
        );
        let a = Action::Post(a);

        self.trader.as_ref().unwrap().send(a).unwrap();
        self.status = Status::PostingSell;
    }
    fn on_buy_event(&mut self, e: OrderEvent) {
        // log::debug!("On buy event");

        let order = e.order;

        if order.is_posted() {
            self.buy_order = Some(order);
            self.status = Status::Opening;
            return;
        }

        if order.is_filled() {
            let trade = self.trade.take().unwrap();

            if let Trade::New(trade) = trade {
                let trade = trade.open(order);
                let trade = Trade::Opened(trade);
                self.trade = Some(trade);
                self.status = Status::Active;
                // log::debug!("Trade opened!");
            } else {
                panic!("WTF??? Трейд должен быть новым...");
            }
        }
    }
    fn on_opening_event(&mut self, e: OrderEvent) {
        // log::debug!("On opening event");

        let order = e.order;

        if order.is_filled() {
            let trade = self.trade.take().unwrap();

            if let Trade::New(trade) = trade {
                let trade = trade.open(order);
                let trade = Trade::Opened(trade);
                self.trade = Some(trade);
                self.status = Status::Active;
                // log::debug!("Trade opened!");
            } else {
                panic!("WTF??? Трейд должен быть новым...");
            }
        }
    }
    fn on_sell_event(&mut self, e: OrderEvent) {
        // log::debug!("On sell event");

        let order = e.order;

        if order.is_posted() {
            self.sell_order = Some(order);
            self.status = Status::Closing;
            return;
        }

        if order.is_filled() {
            let trade = self.trade.take().unwrap();
            match trade {
                Trade::Opened(mut trade) => {
                    trade.add_order(order);
                    let trade = trade.close();
                    let trade = Trade::Closed(trade);
                    let a = Action::TradeClosed(trade);

                    self.trader.as_ref().unwrap().send(a).unwrap();
                    self.trade = None;
                    self.status = Status::Observe;
                    // log::debug!("Trade closed!");
                }
                _ => panic!("????"),
            }
        }
    }
    fn on_closing_event(&mut self, e: OrderEvent) {
        // log::debug!("On closing event");

        let order = e.order;

        if order.is_filled() {
            let trade = self.trade.take().unwrap();
            match trade {
                Trade::Opened(mut trade) => {
                    trade.add_order(order);
                    let trade = trade.close();
                    let trade = Trade::Closed(trade);
                    let a = Action::TradeClosed(trade);

                    self.trader.as_ref().unwrap().send(a).unwrap();
                    self.trade = None;
                    self.status = Status::Observe;
                    // log::debug!("Trade closed!");
                }
                _ => panic!("????"),
            }
        }
    }
}
