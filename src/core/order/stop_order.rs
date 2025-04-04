/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::order::limit_order::PostedLimitOrder;
use crate::core::order::market_order::PostedMarketOrder;

pub enum TriggeredOrder {
    Market(PostedMarketOrder),
    Limit(PostedLimitOrder),
}

#[derive(Debug)]
pub struct StopOrder {
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
}
impl StopOrder {
    pub fn new(
        direction: Direction,
        lots: u32,
        stop_price: f64,
        exec_price: Option<f64>,
    ) -> NewStopOrder {
        NewStopOrder {
            direction,
            lots,
            stop_price,
            exec_price,
        }
    }
}

#[derive(Debug)]
pub struct NewStopOrder {
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
}
impl NewStopOrder {
    pub fn post(self, broker_id: &str) -> PostedStopOrder {
        PostedStopOrder {
            direction: self.direction,
            lots: self.lots,
            stop_price: self.stop_price,
            exec_price: self.exec_price,
            broker_id: broker_id.to_string(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedStopOrder {
        RejectedStopOrder {
            direction: self.direction,
            lots: self.lots,
            stop_price: self.stop_price,
            exec_price: self.exec_price,
            meta: meta.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct PostedStopOrder {
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
    pub broker_id: String,
}
impl PostedStopOrder {
    pub fn trigger(self, broker_id: &str) -> TriggeredOrder {
        match self.exec_price {
            Some(exec_price) => {
                let order = PostedLimitOrder {
                    direction: self.direction,
                    lots: self.lots,
                    price: exec_price,
                    broker_id: broker_id.to_string(),
                    transactions: Vec::new(),
                };
                TriggeredOrder::Limit(order)
            }
            None => {
                let order = PostedMarketOrder {
                    direction: self.direction,
                    lots: self.lots,
                    broker_id: broker_id.to_string(),
                    transactions: Vec::new(),
                };
                TriggeredOrder::Market(order)
            }
        }
    }
}

#[derive(Debug)]
pub struct RejectedStopOrder {
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
    pub meta: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_market() {
        let new = StopOrder::new(Direction::Buy, 2, 4500.0, None);
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 2);
        assert_eq!(new.stop_price, 4500.0);
        assert_eq!(new.exec_price, None);

        let posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let triggered_order = posted.trigger("market_order_id=100501");
        if let TriggeredOrder::Market(order) = triggered_order {
            assert_eq!(order.direction, Direction::Buy);
            assert_eq!(order.lots, 2);
            assert_eq!(order.broker_id, "market_order_id=100501");
            assert_eq!(order.transactions.len(), 0);
        } else {
            panic!("WTF???")
        }
    }
    #[test]
    fn trigger_limit() {
        let new = StopOrder::new(Direction::Buy, 2, 4500.0, Some(4510.0));
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 2);
        assert_eq!(new.stop_price, 4500.0);
        assert_eq!(new.exec_price, Some(4510.0));

        let posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let triggered_order = posted.trigger("limit_order_id=100501");
        if let TriggeredOrder::Limit(order) = triggered_order {
            assert_eq!(order.direction, Direction::Buy);
            assert_eq!(order.lots, 2);
            assert_eq!(order.price, 4510.0);
            assert_eq!(order.broker_id, "limit_order_id=100501");
            assert_eq!(order.transactions.len(), 0);
        } else {
            panic!("WTF???")
        }
    }
    #[test]
    fn reject() {
        let new = StopOrder::new(Direction::Sell, 4, 444.000003, Some(444.0));
        assert_eq!(new.direction, Direction::Sell);
        assert_eq!(new.lots, 4);
        assert_eq!(new.stop_price, 444.000003);
        assert_eq!(new.exec_price, Some(444.0));

        let rejected = new.reject("invalid stop price!");
        assert_eq!(rejected.direction, Direction::Sell);
        assert_eq!(rejected.lots, 4);
        assert_eq!(rejected.stop_price, 444.000003);
        assert_eq!(rejected.exec_price, Some(444.0));
        assert_eq!(rejected.meta, "invalid stop price!");
    }
}
