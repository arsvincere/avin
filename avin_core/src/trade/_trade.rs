/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};
use chrono::{DateTime, TimeDelta, Utc};

use crate::{Direction, Iid, Order, PostedStopOrder};

/// List for selecting the trade type.
///
/// # ru
/// Перечисление для выбора типа трейда.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum TradeKind {
    Long,
    Short,
}
impl TradeKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            TradeKind::Long => "l",
            TradeKind::Short => "s",
        }
    }
}
impl std::fmt::Display for TradeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TradeKind::Long => write!(f, "Long"),
            TradeKind::Short => write!(f, "Short"),
        }
    }
}

/// Group of orders and operations with one position.
///
/// # ru
/// Отдельные ордера и операции по ним объединяются в трейд. Трейд
/// считается открытым, когда совершается первая сделка по бумаге. Этот
/// трейд будет закрыт, когда на счету будет ноль бумаг. Трейд суммирует
/// все операции между открытием и закрытием позиции.
///
/// Статус трейда реализован идиоматичным для Rust путем - через
/// отдельные типы. Этим не очень удобно пользоваться, зато компилятор
/// следит за корректностью работы с трейдами. Например нельзя добавить
/// стоп или тейк к закрытому трейду. Получить результат трейда можно
/// только когда он закрыт и тп.
///
/// В реализации трейдов возможны изменения, поэтому подробной
/// документации по методам пока нет.
#[derive(Debug, PartialEq, Encode, Decode)]
pub enum Trade {
    New(NewTrade),
    Opened(OpenedTrade),
    Closed(ClosedTrade),
}
impl Trade {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        ts_nanos: i64,
        strategy: &str,
        kind: TradeKind,
        iid: Iid,
    ) -> NewTrade {
        NewTrade {
            ts_nanos,
            strategy: strategy.to_string(),
            kind,
            iid,
        }
    }

    pub fn as_new(self) -> Option<NewTrade> {
        match self {
            Trade::New(t) => Some(t),
            Trade::Opened(_) => None,
            Trade::Closed(_) => None,
        }
    }
    pub fn as_opened(self) -> Option<OpenedTrade> {
        match self {
            Trade::New(_) => None,
            Trade::Opened(t) => Some(t),
            Trade::Closed(_) => None,
        }
    }
    pub fn as_closed(self) -> Option<ClosedTrade> {
        match self {
            Trade::New(_) => None,
            Trade::Opened(_) => None,
            Trade::Closed(t) => Some(t),
        }
    }

    pub fn is_new(&self) -> bool {
        matches!(self, Trade::New(_))
    }
    pub fn is_opened(&self) -> bool {
        matches!(self, Trade::Opened(_))
    }
    pub fn is_closed(&self) -> bool {
        matches!(self, Trade::Closed(_))
    }
}
impl std::fmt::Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New(t) => write!(f, "{t}"),
            Self::Opened(t) => write!(f, "{t}"),
            Self::Closed(t) => write!(f, "{t}"),
        }
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct NewTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub kind: TradeKind,
    pub iid: Iid,
}
impl NewTrade {
    pub fn open(self, filled_order: Order) -> OpenedTrade {
        if !filled_order.is_filled() {
            panic!("order shoud be filled")
        }
        OpenedTrade {
            ts_nanos: self.ts_nanos,
            strategy: self.strategy,
            kind: self.kind,
            iid: self.iid,
            orders: vec![filled_order],

            stop_loss: None,
            take_profit: None,
        }
    }
}
impl std::fmt::Display for NewTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "NewTrade={} {} {} {}",
            self.ts_nanos, self.strategy, self.kind, self.iid
        )
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OpenedTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub kind: TradeKind,
    pub iid: Iid,
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
            kind: self.kind,
            iid: self.iid,
            orders: self.orders,
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
        };

        // NOTE: проверка что трейд действительно закрыт
        // количество активов в позиции = 0
        if trade.quantity() != 0 {
            panic!("in closed trade quantity != 0");
        }
        trade
    }

    pub fn is_long(&self) -> bool {
        self.kind == TradeKind::Long
    }
    pub fn is_short(&self) -> bool {
        self.kind == TradeKind::Short
    }

    pub fn lots(&self) -> i32 {
        self.quantity() / self.iid.lot() as i32
    }
    pub fn quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => continue,
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
                None => continue,
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
                None => continue,
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
                None => continue,
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
                None => continue,
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
                None => continue,
            };
            if *order.direction() == Direction::Sell {
                total += op.value
            }
        }

        total
    }

    pub fn avg(&self) -> f64 {
        if self.is_long() {
            self.buy_avg()
        } else {
            self.sell_avg()
        }
    }
    pub fn buy_avg(&self) -> f64 {
        self.buy_value() / self.buy_quantity() as f64
    }
    pub fn sell_avg(&self) -> f64 {
        self.sell_value() / self.sell_quantity() as f64
    }
}
impl std::fmt::Display for OpenedTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "OpenedTrade={} {} {} {}",
            self.ts_nanos, self.strategy, self.kind, self.iid
        )
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct ClosedTrade {
    pub ts_nanos: i64,
    pub strategy: String,
    pub kind: TradeKind,
    pub iid: Iid,
    pub orders: Vec<Order>,
    pub stop_loss: Option<PostedStopOrder>,
    pub take_profit: Option<PostedStopOrder>,
}
impl ClosedTrade {
    pub fn is_long(&self) -> bool {
        self.kind == TradeKind::Long
    }
    pub fn is_short(&self) -> bool {
        self.kind == TradeKind::Short
    }
    pub fn is_win(&self) -> bool {
        todo!();
    }
    pub fn is_loss(&self) -> bool {
        todo!();
    }

    pub fn lots(&self) -> i32 {
        self.quantity() / self.iid.lot() as i32
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

    pub fn avg(&self) -> f64 {
        match self.kind {
            TradeKind::Long => self.buy_avg(),
            TradeKind::Short => self.sell_avg(),
        }
    }
    pub fn buy_avg(&self) -> f64 {
        self.buy_value() / self.buy_quantity() as f64
    }
    pub fn sell_avg(&self) -> f64 {
        self.sell_value() / self.sell_quantity() as f64
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn open_dt(&self) -> DateTime<Utc> {
        let o = self.orders.first().unwrap();
        match o.operation() {
            Some(operation) => operation.dt(),
            None => panic!("closed trade without operation in order"),
        }
    }
    pub fn open_ts(&self) -> i64 {
        let o = self.orders.first().unwrap();
        match o.operation() {
            Some(operation) => operation.ts_nanos,
            None => panic!("closed trade without operation in order"),
        }
    }
    pub fn close_dt(&self) -> DateTime<Utc> {
        let o = self.orders.last().unwrap();
        match o.operation() {
            Some(operation) => operation.dt(),
            None => panic!("closed trade without operation in order"),
        }
    }
    pub fn close_ts(&self) -> i64 {
        let o = self.orders.last().unwrap();
        match o.operation() {
            Some(operation) => operation.ts_nanos,
            None => panic!("closed trade without operation in order"),
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
        // NOTE: если таймдельту перевести сразу в дни то
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
        // NOTE: если таймдельту перевести сразу в дни то
        // для трейдов короче одного дня там будет 0.
        // поэтому смотрю на количество минут трейда, делю на 60 и 24
        // получается например 600m трейд / 60 / 24 = 0.42 дня.
        // Беру результат трейда в процентах и делю на это число
        // в итоге получается количество процентов в день
        // используется для сравнения эффективности трейдов с учетом
        // времени которое деньги были заняты в этом трейде.
        self.result_p()
            / (self.timedelta().num_minutes() as f64 / 60.0 / 24.0)
    }
}
impl std::fmt::Display for ClosedTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ClosedTrade={} {} {} {} = {}",
            self.dt(),
            self.strategy,
            self.kind,
            self.iid.ticker(),
            self.result()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn statuses() {
        // create trade
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let trade =
            Trade::new(ts, "Trend T3 Posterior v1", TradeKind::Long, iid);
        assert_eq!(trade.ts_nanos, ts);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.iid.ticker(), "SBER");

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(100, 301.0);
        order.add_transaction(tr);
        let ts = 0;
        let order = order.fill(ts, 3.0);
        let mut trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // add second filled order
        let order = LimitOrder::new(Direction::Sell, 10, 311.0);
        let mut order = order.post("broker_id=100501");
        let tr = Transaction::new(100, 311.0);
        order.add_transaction(tr);
        let ts = time_unit::TimeUnit::Days.get_unit_nanoseconds() as i64;
        let order = order.fill(ts, 3.0);
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
        let iid = Manager::find_iid("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let trade =
            Trade::new(ts, "Trend T3 Posterior v1", TradeKind::Long, iid);
        assert_eq!(trade.ts_nanos, ts);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.iid.ticker(), "SBER");

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(100, 301.0);
        order.add_transaction(tr);
        let order = order.fill(100500, 3.0);
        let trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // try close opened trade - should_panic
        let _ = trade.close();
    }
}
