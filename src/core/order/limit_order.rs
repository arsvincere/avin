/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::operation::Operation;
use crate::core::transaction::Transaction;

#[derive(Debug)]
pub enum LimitOrder {
    New(NewLimitOrder),
    Posted(PostedLimitOrder),
    Filled(FilledLimitOrder),
    Rejected(RejectedLimitOrder),
}
impl LimitOrder {
    pub fn new(direction: Direction, lots: u32, price: f64) -> NewLimitOrder {
        NewLimitOrder {
            direction,
            lots,
            price,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn fill(self, commission: f64) -> FilledLimitOrder {
        let operation = Operation::from(&self.transactions, commission);
        FilledLimitOrder {
            direction: self.direction,
            lots: self.lots,
            price: self.price,
            broker_id: self.broker_id,
            transactions: self.transactions,
            operation,
        }
    }
}

#[derive(Debug)]
pub struct FilledLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub broker_id: String,
    pub transactions: Vec<Transaction>,
    pub operation: Operation,
}

#[derive(Debug)]
pub struct RejectedLimitOrder {
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub meta: String,
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

        let t1 = Transaction::new(Utc::now(), 1, 4500.0);
        posted.add_transaction(t1);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 1);

        let t2_dt = Utc::now();
        let t2 = Transaction::new(t2_dt.clone(), 1, 4510.0);
        posted.add_transaction(t2);
        assert_eq!(posted.broker_id, "order_id=100500");
        assert_eq!(posted.transactions.len(), 2);

        let order = posted.fill(4.5);
        assert_eq!(order.operation.dt, t2_dt);
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
