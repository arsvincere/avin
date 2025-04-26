/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use crate::core::order::limit_order::PostedLimitOrder;
use crate::core::order::market_order::PostedMarketOrder;
use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum StopOrderKind {
    StopLoss,
    TakeProfit,
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum StopOrder {
    New(NewStopOrder),
    Posted(PostedStopOrder),
    Triggered(TriggeredStopOrder),
    Rejected(RejectedStopOrder),
    Canceled(CanceledStopOrder),
}

impl StopOrder {
    pub fn new(
        kind: StopOrderKind,
        direction: Direction,
        lots: u32,
        stop_price: f64,
        exec_price: Option<f64>,
    ) -> NewStopOrder {
        NewStopOrder {
            kind,
            direction,
            lots,
            stop_price,
            exec_price,
        }
    }
}
impl std::fmt::Display for StopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New(order) => write!(f, "{order}"),
            Self::Posted(order) => write!(f, "{order}"),
            Self::Triggered(order) => write!(f, "{order}"),
            Self::Rejected(order) => write!(f, "{order}"),
            Self::Canceled(order) => write!(f, "{order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct NewStopOrder {
    pub kind: StopOrderKind,
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
}
impl NewStopOrder {
    pub fn post(self, broker_id: &str) -> PostedStopOrder {
        PostedStopOrder {
            kind: self.kind,
            direction: self.direction,
            lots: self.lots,
            stop_price: self.stop_price,
            exec_price: self.exec_price,
            broker_id: broker_id.to_string(),
        }
    }
    pub fn reject(self, meta: &str) -> RejectedStopOrder {
        RejectedStopOrder {
            kind: self.kind,
            direction: self.direction,
            lots: self.lots,
            stop_price: self.stop_price,
            exec_price: self.exec_price,
            meta: meta.to_string(),
        }
    }
}
impl std::fmt::Display for NewStopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StopOrder::New={} {} stop_price={} exec_price={}",
            self.direction,
            self.lots,
            self.stop_price,
            self.exec_price.unwrap_or(0.0),
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct PostedStopOrder {
    pub kind: StopOrderKind,
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
    pub broker_id: String,
}
impl PostedStopOrder {
    pub fn trigger(self, broker_id: &str) -> TriggeredStopOrder {
        match self.exec_price {
            Some(exec_price) => {
                let order = PostedLimitOrder {
                    direction: self.direction,
                    lots: self.lots,
                    price: exec_price,
                    broker_id: broker_id.to_string(),
                    transactions: Vec::new(),
                };
                TriggeredStopOrder::Limit(order)
            }
            None => {
                let order = PostedMarketOrder {
                    direction: self.direction,
                    lots: self.lots,
                    broker_id: broker_id.to_string(),
                    transactions: Vec::new(),
                };
                TriggeredStopOrder::Market(order)
            }
        }
    }
    pub fn cancel(self) -> CanceledStopOrder {
        CanceledStopOrder {
            kind: self.kind,
            direction: self.direction,
            lots: self.lots,
            stop_price: self.stop_price,
            exec_price: self.exec_price,
            broker_id: self.broker_id,
        }
    }
}
impl std::fmt::Display for PostedStopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StopOrder::Posted={} {} stop_price={} exec_price={} id={}",
            self.direction,
            self.lots,
            self.stop_price,
            self.exec_price.unwrap_or(0.0),
            self.broker_id
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum TriggeredStopOrder {
    Market(PostedMarketOrder),
    Limit(PostedLimitOrder),
}
impl std::fmt::Display for TriggeredStopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Market(order) => write!(f, "Triggered={order}"),
            Self::Limit(order) => write!(f, "Triggered={order}"),
        }
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct RejectedStopOrder {
    pub kind: StopOrderKind,
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
    pub meta: String,
}
impl std::fmt::Display for RejectedStopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StopOrder::Posted={} {} stop_price={} exec_price={} meta={}",
            self.direction,
            self.lots,
            self.stop_price,
            self.exec_price.unwrap_or(0.0),
            self.meta
        )
    }
}

#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub struct CanceledStopOrder {
    pub kind: StopOrderKind,
    pub direction: Direction,
    pub lots: u32,
    pub stop_price: f64,
    pub exec_price: Option<f64>,
    pub broker_id: String,
}
impl std::fmt::Display for CanceledStopOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StopOrder::Posted={} {} stop_price={} exec_price={} id={}",
            self.direction,
            self.lots,
            self.stop_price,
            self.exec_price.unwrap_or(0.0),
            self.broker_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_market() {
        let new = StopOrder::new(
            StopOrderKind::TakeProfit,
            Direction::Buy,
            2,
            4500.0,
            None,
        );
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 2);
        assert_eq!(new.stop_price, 4500.0);
        assert_eq!(new.exec_price, None);

        let posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let triggered_order = posted.trigger("market_order_id=100501");
        if let TriggeredStopOrder::Market(order) = triggered_order {
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
        let new = StopOrder::new(
            StopOrderKind::TakeProfit,
            Direction::Buy,
            2,
            4500.0,
            Some(4510.0),
        );
        assert_eq!(new.direction, Direction::Buy);
        assert_eq!(new.lots, 2);
        assert_eq!(new.stop_price, 4500.0);
        assert_eq!(new.exec_price, Some(4510.0));

        let posted = new.post("order_id=100500");
        assert_eq!(posted.broker_id, "order_id=100500");

        let triggered_order = posted.trigger("limit_order_id=100501");
        if let TriggeredStopOrder::Limit(order) = triggered_order {
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
        let new = StopOrder::new(
            StopOrderKind::StopLoss,
            Direction::Sell,
            4,
            444.000003,
            Some(444.0),
        );
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
