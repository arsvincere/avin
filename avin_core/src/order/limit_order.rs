/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

use crate::{Direction, Operation, Transaction};

/// Wrapper for limit order statuses.
///
/// # ru
/// Обертка для статусов лимитных ордеров.
///
/// Состояние ордеров реализовано идиоматичным для Rust путем -  через
/// отдельные типы. Этим не очень удобно пользоваться, зато компилятор
/// следит за корректностью работы с ордерами. Например нельзя
/// отменить ордер который еще не выставлен, или уже исполнен. Отменить
/// можно только выставленный или частично исполненный ордер.
///
/// В реализации ордеров возможны изменения, поэтому подробной
/// документации по методам пока нет.
#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum LimitOrder {
    New(NewLimitOrder),
    Posted(PostedLimitOrder),
    Filled(FilledLimitOrder),
    Rejected(RejectedLimitOrder),
    Canceled(CanceledLimitOrder),
}
impl LimitOrder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(direction: Direction, lots: u32, price: f64) -> NewLimitOrder {
        NewLimitOrder {
            direction,
            lots,
            price,
        }
    }

    pub fn as_new(self) -> Option<NewLimitOrder> {
        match self {
            LimitOrder::New(o) => Some(o),
            LimitOrder::Posted(_) => None,
            LimitOrder::Filled(_) => None,
            LimitOrder::Rejected(_) => None,
            LimitOrder::Canceled(_) => None,
        }
    }
    pub fn as_posted(self) -> Option<PostedLimitOrder> {
        match self {
            LimitOrder::New(_) => None,
            LimitOrder::Posted(o) => Some(o),
            LimitOrder::Filled(_) => None,
            LimitOrder::Rejected(_) => None,
            LimitOrder::Canceled(_) => None,
        }
    }
    pub fn as_filled(self) -> Option<FilledLimitOrder> {
        match self {
            LimitOrder::New(_) => None,
            LimitOrder::Posted(_) => None,
            LimitOrder::Filled(o) => Some(o),
            LimitOrder::Rejected(_) => None,
            LimitOrder::Canceled(_) => None,
        }
    }
    pub fn as_rejected(self) -> Option<RejectedLimitOrder> {
        match self {
            LimitOrder::New(_) => None,
            LimitOrder::Posted(_) => None,
            LimitOrder::Filled(_) => None,
            LimitOrder::Rejected(o) => Some(o),
            LimitOrder::Canceled(_) => None,
        }
    }
    pub fn as_canceled(self) -> Option<CanceledLimitOrder> {
        match self {
            LimitOrder::New(_) => None,
            LimitOrder::Posted(_) => None,
            LimitOrder::Filled(_) => None,
            LimitOrder::Rejected(_) => None,
            LimitOrder::Canceled(o) => Some(o),
        }
    }

    pub fn is_new(&self) -> bool {
        matches!(self, LimitOrder::New(_))
    }
    pub fn is_posted(&self) -> bool {
        matches!(self, LimitOrder::Posted(_))
    }
    pub fn is_filled(&self) -> bool {
        matches!(self, LimitOrder::Filled(_))
    }
    pub fn is_rejected(&self) -> bool {
        matches!(self, LimitOrder::Rejected(_))
    }
    pub fn is_canceled(&self) -> bool {
        matches!(self, LimitOrder::Canceled(_))
    }
}
impl std::fmt::Display for LimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New(order) => write!(f, "{order}"),
            Self::Posted(order) => write!(f, "{order}"),
            Self::Filled(order) => write!(f, "{order}"),
            Self::Canceled(order) => write!(f, "{order}"),
            Self::Rejected(order) => write!(f, "{order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct NewLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
}
impl NewLimitOrder {
    pub fn post(self, broker_id: &str) -> PostedLimitOrder {
        PostedLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: broker_id.to_string(),
            transactions: Vec::new(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedLimitOrder {
        RejectedLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            meta: meta.to_string(),
        }
    }
}
impl std::fmt::Display for NewLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::New={} {}x{}",
            self.direction, self.lots, self.price
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct PostedLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl PostedLimitOrder {
    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.push(t);
    }
    pub fn fill(self, ts_nanos: i64, commission: f64) -> FilledLimitOrder {
        let operation =
            Operation::build(ts_nanos, &self.transactions, commission);
        FilledLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: self.broker_id,
            transactions: self.transactions,
            operation,
        }
    }
    pub fn cancel(self) -> CanceledLimitOrder {
        CanceledLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: self.broker_id,
            transactions: self.transactions,
        }
    }
}
impl std::fmt::Display for PostedLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Posted={} {}x{} id={} t={:?}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct FilledLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
    pub operation: Operation,
}
impl std::fmt::Display for FilledLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Filled={} {}x{} id={} t={:?} {}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions,
            self.operation
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct CanceledLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
}
impl std::fmt::Display for CanceledLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Canceled={} {}x{} id={} transactions={:?}",
            self.direction,
            self.lots,
            self.price,
            self.broker_id,
            self.transactions
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct RejectedLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub meta: String,
}
impl std::fmt::Display for RejectedLimitOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LimitOrder::Rejected={} {}x{} meta={}",
            self.direction, self.lots, self.price, self.meta
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn new_post_fill() {
        let new = LimitOrder::new(Direction::Buy, 2, 4500.0);

        let mut posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let t1 = Transaction::new(1, 4500.0);
        posted.add_transaction(t1);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 1);

        let t2 = Transaction::new(1, 4510.0);
        posted.add_transaction(t2);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 2);

        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let order = posted.fill(ts, 4.5);
        assert_eq!(order.operation.dt(), dt);
        assert_eq!(order.operation.ts_nanos, ts);
        assert_eq!(order.operation.quantity, 2);
        assert_eq!(order.operation.value, 9010.0);
        assert_eq!(order.operation.commission, 4.5);
    }
    #[test]
    fn reject() {
        let new = LimitOrder::new(Direction::Buy, 100, 400.0);
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 100);
        assert_eq!(new.price, 400.0);
        dbg!(&new);

        let reject = new.reject("not enought money");
        assert_eq!(reject.meta, "not enought money");
        dbg!(&reject);
    }
}
