/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{
    Account, Action, Direction, IID, LimitOrder, Order, OrderEvent,
    PostOrderAction, Share, TimeFrame, Trade, TradeKind, utils,
};

#[derive(Debug)]
pub struct T1mT1 {
    name: String,
    trader: tokio::sync::mpsc::UnboundedSender<Action>,
    account: Account,
    iid: IID,

    last_ts: i64,
    trade: Option<Trade>,
}
impl T1mT1 {
    pub fn new(
        trader: tokio::sync::mpsc::UnboundedSender<Action>,
        account: Account,
        iid: &IID,
    ) -> Self {
        let name = "T1mT1".to_string();

        let last_ts = 0;

        Self {
            name,
            trader,
            account,
            iid: iid.clone(),

            last_ts,
            trade: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn process(&mut self, share: &Share) {
        if self.trade.is_some() {
            return;
        }

        let tf = TimeFrame::M1;
        let chart = share.chart(&tf).unwrap();
        let bar = chart.last();
        if bar.is_none() {
            return;
        }
        let bar = bar.unwrap();
        if bar.ts_nanos == self.last_ts {
            return;
        }
        // сохранить время последней обработки
        self.last_ts = bar.ts_nanos.clone();

        // create trade
        self.create_trade();

        let price = bar.c;
        let step = self.iid.step();
        let buy_price = utils::round_price(price * 0.999, step);
        let sell_price = utils::round_price(price * 1.001, step);

        self.post_buy_limit(buy_price);
        self.post_sell_limit(sell_price);
    }
    pub fn order_event(&mut self, _e: OrderEvent) {}

    // private
    fn create_trade(&mut self) {
        let trade = Trade::new(
            self.last_ts,
            self.name(),
            TradeKind::Long,
            self.iid.clone(),
        );
        self.trade = Some(Trade::New(trade));
    }
    fn post_buy_limit(&self, price: f64) {
        log::info!("Buy limit!");

        let order = LimitOrder::new(Direction::Buy, 1, price);
        let order = LimitOrder::New(order);
        let order = Order::Limit(order);

        let a = PostOrderAction::new(
            self.account.clone(),
            self.iid.clone(),
            self.name.clone(),
            order,
        );
        let a = Action::Post(a);
        self.trader.send(a).unwrap();

        todo!();
        // log::info!("Await buy!");
        // let result = match rx.blocking_recv() {
        //     Ok(result) => {
        //         log::info!("Strategy rx {:?}", result);
        //         result
        //     }
        //     Err(err) => {
        //         log::error!("Strategy rx {:?}", err);
        //         panic!()
        //     }
        // };

        // unwrap Order
        // let order = match result {
        //     Ok(order) => order,
        //     Err(err) => {
        //         log::error!("Strategy fail post buy: {:?}", err);
        //         todo!("И че делать?");
        //     }
        // };

        // if !order.is_posted() {
        //     todo!("И че делать?");
        // }
    }
    fn post_sell_limit(&self, price: f64) {
        log::info!("Sell!");

        let order = LimitOrder::new(Direction::Sell, 1, price);
        let order = LimitOrder::New(order);
        let order = Order::Limit(order);

        let a = PostOrderAction::new(
            self.account.clone(),
            self.iid.clone(),
            self.name.clone(),
            order,
        );
        let a = Action::Post(a);

        self.trader.send(a).unwrap();

        todo!();
        // log::info!("Await sell!");
        // let result = match rx.blocking_recv() {
        //     Ok(result) => {
        //         log::info!("Strategy rx {:?}", result);
        //         result
        //     }
        //     Err(err) => {
        //         log::error!("Strategy rx {:?}", err);
        //         todo!("И че делать?")
        //     }
        // };
        //
        // // unwrap Order
        // let order = match result {
        //     Ok(order) => order,
        //     Err(err) => {
        //         log::error!("Strategy fail post buy: {:?}", err);
        //         todo!("И че делать?");
        //     }
        // };
        //
        // if !order.is_posted() {
        //     todo!("И че делать?");
        // }
    }
}
