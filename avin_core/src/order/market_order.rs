/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

use crate::{Direction, Operation, Transaction};

/// Wrapper for market order statuses.
///
/// # ru
/// Обертка для статусов лимитных ордеров.
///
/// Состояние ордеров реализовано идиоматичным для Rust путем - через
/// отдельные типы. Этим не очень удобно пользоваться, зато компилятор
/// следит за корректностью работы с ордерами. Например нельзя
/// отменить ордер который еще не выставлен, или уже исполнен. Отменить
/// можно только выставленный или частично исполненный ордер.
///
/// В реализации ордеров возможны изменения, поэтому подробной
/// документации по методам пока нет.
#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum MarketOrder {
    New(NewMarketOrder),
    Posted(PostedMarketOrder),
    Filled(FilledMarketOrder),
    Rejected(RejectedMarketOrder),
}
impl MarketOrder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(direction: Direction, lots: u32) -> NewMarketOrder {
        NewMarketOrder { direction, lots }
    }

    pub fn as_new(self) -> Option<NewMarketOrder> {
        match self {
            MarketOrder::New(o) => Some(o),
            MarketOrder::Posted(_) => None,
            MarketOrder::Filled(_) => None,
            MarketOrder::Rejected(_) => None,
        }
    }
    pub fn as_posted(self) -> Option<PostedMarketOrder> {
        match self {
            MarketOrder::New(_) => None,
            MarketOrder::Posted(o) => Some(o),
            MarketOrder::Filled(_) => None,
            MarketOrder::Rejected(_) => None,
        }
    }
    pub fn as_filled(self) -> Option<FilledMarketOrder> {
        match self {
            MarketOrder::New(_) => None,
            MarketOrder::Posted(_) => None,
            MarketOrder::Filled(o) => Some(o),
            MarketOrder::Rejected(_) => None,
        }
    }
    pub fn as_rejected(self) -> Option<RejectedMarketOrder> {
        match self {
            MarketOrder::New(_) => None,
            MarketOrder::Posted(_) => None,
            MarketOrder::Filled(_) => None,
            MarketOrder::Rejected(o) => Some(o),
        }
    }
}
impl std::fmt::Display for MarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New(order) => write!(f, "{order}"),
            Self::Posted(order) => write!(f, "{order}"),
            Self::Filled(order) => write!(f, "{order}"),
            Self::Rejected(order) => write!(f, "{order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct NewMarketOrder {
    pub direction: Direction,
    pub lots: u32,
}
impl NewMarketOrder {
    pub fn post(self, broker_id: &str) -> PostedMarketOrder {
        PostedMarketOrder {
            direction: self.direction,
            lots: self.lots,
            broker_id: broker_id.to_string(),
            transactions: Vec::new(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedMarketOrder {
        RejectedMarketOrder {
            direction: self.direction,
            lots: self.lots,
            meta: meta.to_string(),
        }
    }
}
impl std::fmt::Display for NewMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MarketOrder::New={} {}", self.direction, self.lots)
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct PostedMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl PostedMarketOrder {
    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.push(t);
    }
    pub fn fill(self, ts_nanos: i64, commission: f64) -> FilledMarketOrder {
        let operation =
            Operation::build(ts_nanos, &self.transactions, commission);
        FilledMarketOrder {
            direction: self.direction,
            lots: self.lots,
            broker_id: self.broker_id,
            transactions: self.transactions,
            operation,
        }
    }
}
impl std::fmt::Display for PostedMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Posted={} {} id={} t={:?}",
            self.direction, self.lots, self.broker_id, self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct FilledMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
    pub operation: Operation,
}
impl std::fmt::Display for FilledMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Filled={} {} id={} t={:?} {}",
            self.direction,
            self.lots,
            self.broker_id,
            self.transactions,
            self.operation
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct RejectedMarketOrder {
    pub direction: Direction,
    pub lots: u32,
    pub meta: String,
}
impl std::fmt::Display for RejectedMarketOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MarketOrder::Rejected={} {} meta={}",
            self.direction, self.lots, self.meta
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn new_post_fill() {
        let new = MarketOrder::new(Direction::Buy, 10);

        let mut posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let t1 = Transaction::new(5, 320.0);
        posted.add_transaction(t1);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 1);

        let t2 = Transaction::new(5, 320.0);
        posted.add_transaction(t2);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 2);

        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let order = posted.fill(ts, 3.2);
        assert_eq!(order.operation.dt(), dt);
        assert_eq!(order.operation.quantity, 10);
        assert_eq!(order.operation.value, 3200.0);
        assert_eq!(order.operation.commission, 3.2);
    }
    #[test]
    fn reject() {
        let new = MarketOrder::new(Direction::Sell, 10);
        assert_eq!(new.direction, Direction::Sell);
        assert_eq!(new.lots, 10);
        dbg!(&new);

        let reject = new.reject("market is closed");
        assert_eq!(reject.meta, "market is closed");
        dbg!(&reject);
    }
}
