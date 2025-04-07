/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod limit_order;
mod market_order;
mod stop_order;

use crate::core::direction::Direction;
use crate::core::operation::Operation;
use crate::core::transaction::Transaction;
use bitcode::{Decode, Encode};
use std::collections::HashMap;

pub use limit_order::{
    CanceledLimitOrder, FilledLimitOrder, LimitOrder, NewLimitOrder,
    PostedLimitOrder, RejectedLimitOrder,
};
pub use market_order::{
    FilledMarketOrder, MarketOrder, NewMarketOrder, PostedMarketOrder,
    RejectedMarketOrder,
};
pub use stop_order::{
    NewStopOrder, PostedStopOrder, RejectedStopOrder, StopOrder,
    TriggeredStopOrder,
};

#[derive(Debug, PartialEq, Decode, Encode)]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
    Stop(StopOrder),
}
impl Order {
    pub fn from_bin(bytes: &Vec<u8>) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        todo!();
    }

    pub fn is_posted(&self) -> bool {
        match self {
            Order::Market(market) => match market {
                MarketOrder::Posted(_) => true,
                _ => false,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::Posted(_) => true,
                _ => false,
            },
            Order::Stop(stop) => match stop {
                StopOrder::Posted(_) => true,
                _ => false,
            },
        }
    }
    pub fn is_filled(&self) -> bool {
        match self {
            Order::Market(o) => match o {
                MarketOrder::Filled(_) => true,
                _ => false,
            },
            Order::Limit(o) => match o {
                LimitOrder::Filled(_) => true,
                _ => false,
            },
            Order::Stop(_) => panic!("Stop order can't be filled"),
        }
    }
    pub fn direction(&self) -> &Direction {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(o) => &o.direction,
                MarketOrder::Posted(o) => &o.direction,
                MarketOrder::Filled(o) => &o.direction,
                MarketOrder::Rejected(o) => &o.direction,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(o) => &o.direction,
                LimitOrder::Posted(o) => &o.direction,
                LimitOrder::Filled(o) => &o.direction,
                LimitOrder::Rejected(o) => &o.direction,
                LimitOrder::Canceled(o) => &o.direction,
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(o) => &o.direction,
                StopOrder::Posted(o) => &o.direction,
                StopOrder::Rejected(o) => &o.direction,
                StopOrder::Canceled(o) => &o.direction,
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(o) => &o.direction,
                    TriggeredStopOrder::Limit(o) => &o.direction,
                },
            },
        }
    }
    pub fn lots(&self) -> u32 {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(o) => o.lots,
                MarketOrder::Posted(o) => o.lots,
                MarketOrder::Filled(o) => o.lots,
                MarketOrder::Rejected(o) => o.lots,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(o) => o.lots,
                LimitOrder::Posted(o) => o.lots,
                LimitOrder::Filled(o) => o.lots,
                LimitOrder::Rejected(o) => o.lots,
                LimitOrder::Canceled(o) => o.lots,
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(o) => o.lots,
                StopOrder::Posted(o) => o.lots,
                StopOrder::Rejected(o) => o.lots,
                StopOrder::Canceled(o) => o.lots,
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(o) => o.lots,
                    TriggeredStopOrder::Limit(o) => o.lots,
                },
            },
        }
    }
    pub fn transactions(&self) -> Option<&Vec<Transaction>> {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(_) => None,
                MarketOrder::Posted(o) => Some(&o.transactions),
                MarketOrder::Filled(o) => Some(&o.transactions),
                MarketOrder::Rejected(_) => None,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(_) => None,
                LimitOrder::Posted(o) => Some(&o.transactions),
                LimitOrder::Filled(o) => Some(&o.transactions),
                LimitOrder::Rejected(_) => None,
                LimitOrder::Canceled(o) => Some(&o.transactions),
            },
            Order::Stop(stop) => match stop {
                _ => None,
            },
        }
    }
    pub fn operation(&self) -> Option<&Operation> {
        match self {
            Order::Market(market) => match market {
                MarketOrder::Filled(o) => Some(&o.operation),
                _ => None,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::Filled(o) => Some(&o.operation),
                _ => None,
            },
            Order::Stop(stop) => match stop {
                _ => None,
            },
        }
    }
    pub fn broker_id(&self) -> Option<&String> {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(_) => None,
                MarketOrder::Posted(o) => Some(&o.broker_id),
                MarketOrder::Filled(o) => Some(&o.broker_id),
                MarketOrder::Rejected(_) => None,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(_) => None,
                LimitOrder::Posted(o) => Some(&o.broker_id),
                LimitOrder::Filled(o) => Some(&o.broker_id),
                LimitOrder::Rejected(_) => None,
                LimitOrder::Canceled(o) => Some(&o.broker_id),
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(_) => None,
                StopOrder::Posted(o) => Some(&o.broker_id),
                StopOrder::Rejected(_) => None,
                StopOrder::Canceled(o) => Some(&o.broker_id),
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(o) => Some(&o.broker_id),
                    TriggeredStopOrder::Limit(o) => Some(&o.broker_id),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn bin() {
        let new = LimitOrder::new(Direction::Buy, 2, 4500.0);

        let mut posted = new.post("order_id=100500");

        let t1 = Transaction::new(1, 4500.0);
        posted.add_transaction(t1);

        let t2 = Transaction::new(1, 4510.0);
        posted.add_transaction(t2);

        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let order = posted.fill(ts, 4.5);

        // wrap
        let order = Order::Limit(LimitOrder::Filled(order));

        let encoded = order.to_bin();
        let decoded = Order::from_bin(&encoded);
        assert_eq!(order, decoded);
    }
}
