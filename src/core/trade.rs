/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::asset::Asset;
use crate::core::direction::Direction;
use crate::core::order::{Order, PostedStopOrder};
use bitcode::{Decode, Encode};
use chrono::{DateTime, TimeDelta, Utc};

#[derive(Debug, PartialEq, Encode, Decode)]
pub enum Trade {
    New(NewTrade),
    Opened(OpenedTrade),
    Closed(ClosedTrade),
}
impl Trade {
    pub fn new(
        ts_nanos: i64,
        strategy: &str,
        typ: TradeType,
        asset: &Asset,
    ) -> NewTrade {
        NewTrade {
            ts_nanos,
            strategy: strategy.to_string(),
            typ,
            asset: asset.copy_id(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum TradeType {
    Long,
    Short,
}
impl TradeType {
    pub fn to_str(&self) -> &'static str {
        match self {
            TradeType::Long => "l",
            TradeType::Short => "s",
        }
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct NewTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
}
impl NewTrade {
    pub fn open(self, filled_order: Order) -> OpenedTrade {
        if !filled_order.is_filled() {
            panic!("order shoud be filled")
        }
        OpenedTrade {
            ts_nanos: self.ts_nanos,
            strategy: self.strategy,
            typ: self.typ,
            asset: self.asset,
            orders: vec![filled_order],

            stop_loss: None,
            take_profit: None,
        }
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OpenedTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
    pub orders: Vec<Order>,

    pub stop_loss: Option<PostedStopOrder>,
    pub take_profit: Option<PostedStopOrder>,
}
impl OpenedTrade {
    pub fn add_order(&mut self, filled_order: Order) {
        if !filled_order.is_filled() {
            panic!("order shoud be filled")
        }

        self.orders.push(filled_order)
    }
    pub fn set_stop(&mut self, stop_order: PostedStopOrder) {
        self.stop_loss = Some(stop_order);
    }
    pub fn set_take(&mut self, stop_order: PostedStopOrder) {
        self.take_profit = Some(stop_order);
    }
    pub fn close(self) -> ClosedTrade {
        let trade = ClosedTrade {
            ts_nanos: self.ts_nanos,
            strategy: self.strategy,
            typ: self.typ,
            asset: self.asset,
            orders: self.orders,
        };

        // INFO: проверка что трейд действительно закрыт
        // количество активов в позиции = 0
        if trade.quantity() != 0 {
            panic!("in closed trade quantity != 0");
        }
        trade
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ClosedTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
    pub orders: Vec<Order>,
}
impl ClosedTrade {
    pub fn is_long(&self) -> bool {
        self.typ == TradeType::Long
    }
    pub fn is_short(&self) -> bool {
        self.typ == TradeType::Short
    }
    pub fn is_win(&self) -> bool {
        todo!();
    }
    pub fn is_loss(&self) -> bool {
        todo!();
    }

    pub fn lots(&self) -> i32 {
        todo!();
    }

    pub fn quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.quantity
            } else {
                total -= op.quantity
            }
        }

        total
    }
    pub fn buy_quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.quantity
            }
        }

        total
    }
    pub fn sell_quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.quantity
            }
        }

        total
    }

    pub fn value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.value
            } else {
                total -= op.value
            }
        }

        total
    }
    pub fn buy_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.value
            }
        }

        total
    }
    pub fn sell_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.value
            }
        }

        total
    }

    pub fn commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            total += op.commission
        }

        total
    }
    pub fn buy_commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.commission
            }
        }

        total
    }
    pub fn sell_commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.commission
            }
        }

        total
    }

    pub fn buy_avg(&self) -> f64 {
        self.buy_value() / self.buy_quantity() as f64
    }
    pub fn sell_avg(&self) -> f64 {
        self.sell_value() / self.sell_quantity() as f64
    }

    pub fn open_dt(&self) -> DateTime<Utc> {
        let o = self.orders.first().unwrap();
        match o.transactions() {
            Some(transactions) => transactions.first().unwrap().dt(),
            None => panic!("closed trade without transactions in order"),
        }
    }
    pub fn close_dt(&self) -> DateTime<Utc> {
        let o = self.orders.last().unwrap();
        match o.transactions() {
            Some(transactions) => transactions.last().unwrap().dt(),
            None => panic!("closed trade without transactions in order"),
        }
    }
    pub fn timedelta(&self) -> TimeDelta {
        self.close_dt() - self.open_dt()
    }
    pub fn result(&self) -> f64 {
        self.sell_value() - self.buy_value() - self.commission()
    }
    pub fn result_p(&self) -> f64 {
        self.result() / self.buy_value() * 100.0
    }
    pub fn speed(&self) -> f64 {
        // INFO: если таймдельту перевести сразу в дни то
        // для трейдов короче одного дня там будет 0.
        // поэтому смотрю на количество минут трейда, делю на 60 и 24
        // получается например 600 / 60 / 24 = 0.42 дня.
        // Беру результат трейда в процентах и делю на это число
        // в итоге получается количество рублей в день
        // используется для сравнения эффективности трейдов с учетом
        // времени которое деньги были заняты в этом трейде.
        self.result() / (self.timedelta().num_minutes() as f64 / 60.0 / 24.0)
    }
    pub fn speed_p(&self) -> f64 {
        // INFO: если таймдельту перевести сразу в дни то
        // для трейдов короче одного дня там будет 0.
        // поэтому смотрю на количество минут трейда, делю на 60 и 24
        // получается например 600 / 60 / 24 = 0.42 дня.
        // Беру результат трейда в процентах и делю на это число
        // в итоге получается количество процентов в день
        // используется для сравнения эффективности трейдов с учетом
        // времени которое деньги были заняты в этом трейде.
        self.result_p()
            / (self.timedelta().num_minutes() as f64 / 60.0 / 24.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn statuses() {
        // create trade
        let asset = Asset::from("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let trade =
            Trade::new(ts, "Trend T3 Posterior v1", TradeType::Long, &asset);
        assert_eq!(trade.ts_nanos, ts);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.asset.ticker, "SBER");

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 5, 14, 57, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            100,
            301.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        let mut trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // add second filled order
        let order = LimitOrder::new(Direction::Sell, 10, 311.0);
        let mut order = order.post("broker_id=100501");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 6, 14, 57, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            100,
            311.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        trade.add_order(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 2);

        // close trade
        let trade = trade.close();
        assert_eq!(trade.result(), 994.0);
        assert!(trade.result_p() > 3.3);
        assert_eq!(trade.timedelta().num_seconds(), 86400); // сутки
        assert!(trade.speed() > 990.0);
        assert!(trade.speed_p() > 3.3);
    }
    #[test]
    #[should_panic]
    fn close_unclosed_trade() {
        // create trade
        let asset = Asset::from("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let trade =
            Trade::new(ts, "Trend T3 Posterior v1", TradeType::Long, &asset);
        assert_eq!(trade.ts_nanos, ts);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.asset.ticker, "SBER");

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 5, 14, 57, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            100,
            301.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        let trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // try close opened trade - should_panic
        let _ = trade.close();
    }
}
