/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

use crate::{Direction, Operation, Transaction};

use super::limit_order::LimitOrder;
use super::market_order::MarketOrder;
use super::stop_order::{StopOrder, TriggeredStopOrder};

/// Wrapper for any order type.
///
/// # ru
/// Обертка для разных типов ордеров: рыночный, лимитный, стоп-ордер.
///
/// Предоставляет интерфейс общий для любого типа ордера.
#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
    Stop(StopOrder),
}
impl Order {
    /// Create order from bin format
    ///
    /// # ru
    /// Создает ордер из бинарного формата, который использует
    /// тестер и трейдер для сохранения на диске.
    pub fn from_bin(bytes: &[u8]) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    /// Create vector bytes from order, for saving.
    ///
    /// # ru
    /// Преобразует ордер в бинарный формат для сохранения на диске.
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }

    /// Unwrap as market order
    ///
    /// # ru
    /// Развернуть как маркет ордер
    pub fn as_market(self) -> Option<MarketOrder> {
        match self {
            Order::Market(o) => Some(o),
            Order::Limit(_) => None,
            Order::Stop(_) => None,
        }
    }
    /// Unwrap as limit order
    ///
    /// # ru
    /// Развернуть как лимит ордер
    pub fn as_limit(self) -> Option<LimitOrder> {
        match self {
            Order::Market(_) => None,
            Order::Limit(o) => Some(o),
            Order::Stop(_) => None,
        }
    }
    /// Unwrap as stop order
    ///
    /// # ru
    /// Развернуть как стоп ордер
    pub fn as_stop(self) -> Option<StopOrder> {
        match self {
            Order::Market(_) => None,
            Order::Limit(_) => None,
            Order::Stop(o) => Some(o),
        }
    }

    /// Check is order market
    pub fn is_market(&self) -> bool {
        matches!(self, Order::Market(_))
    }
    /// Check is order limit
    pub fn is_limit(&self) -> bool {
        matches!(self, Order::Limit(_))
    }
    /// Check is order stop
    pub fn is_stop(&self) -> bool {
        matches!(self, Order::Stop(_))
    }

    /// Check is order posted.
    ///
    /// # ru
    /// Проверка статуса ордера, если выставлен true, иначе false.
    pub fn is_posted(&self) -> bool {
        match self {
            Order::Market(m) => matches!(m, MarketOrder::Posted(_)),
            Order::Limit(l) => matches!(l, LimitOrder::Posted(_)),
            Order::Stop(s) => matches!(s, StopOrder::Posted(_)),
        }
    }
    /// Check is order filled.
    ///
    /// # ru
    /// Проверка статуса ордера, если исполнен true, иначе false.
    pub fn is_filled(&self) -> bool {
        match self {
            Order::Market(m) => matches!(m, MarketOrder::Filled(_)),
            Order::Limit(l) => matches!(l, LimitOrder::Filled(_)),
            Order::Stop(_) => panic!("Stop order can't be filled"),
        }
    }
    /// Return order direction.
    ///
    /// # ru
    /// Возвращает направление ордера (покупка/продажа).
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
    /// Return lots count.
    ///
    /// # ru
    /// Возвращает количество лотов по ордеру.
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
    /// Return transactions of order.
    ///
    /// # ru
    /// Возвращает вектор транзакций по ордеру. Если ордер не выставлен
    /// или отклонен вернет None. Если ордер выставлен, но транзакций
    /// еще нет вернет пустой вектор.
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
            Order::Stop(_) => panic!("Stop order can't have transactions"),
        }
    }
    /// Return operation of order.
    ///
    /// # ru
    /// Возвращает операцию по ордеру. Если ордер не полностью исполнен
    /// вернет None. Если вызвать на стоп ордере - panic!
    /// Стоп ордера не исполняются, они срабатывают (triggered),
    /// и в результате выставляется рыночный или лимитный ордер,
    /// вот он уже будет исполняться.
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
            Order::Stop(_) => panic!("Stop order can't have operation"),
        }
    }
    /// Return broker ID of order.
    ///
    /// # ru
    /// Возвращает ID ордера, который использует брокер.
    /// Если ордер не выставлен или отклонен вернет None.
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
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(_) => write!(f, "MarketOrder-New"),
                MarketOrder::Posted(_) => write!(f, "MarketOrder-Posted"),
                MarketOrder::Filled(_) => write!(f, "MarketOrder-Filled"),
                MarketOrder::Rejected(_) => write!(f, "MarketOrder-Rejected"),
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(_) => write!(f, "Limit-New"),
                LimitOrder::Posted(_) => write!(f, "Limit-Posted"),
                LimitOrder::Filled(_) => write!(f, "Limit-Filled"),
                LimitOrder::Rejected(_) => write!(f, "Limit-Rejected"),
                LimitOrder::Canceled(_) => write!(f, "Limit-Canceled"),
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(_) => write!(f, "Stop-New"),
                StopOrder::Posted(_) => write!(f, "Stop-Posted"),
                StopOrder::Rejected(_) => write!(f, "Stop-Rejected"),
                StopOrder::Canceled(_) => write!(f, "Stop-Canceled"),
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(_) => {
                        write!(f, "Stop-Triggered-Market")
                    }
                    TriggeredStopOrder::Limit(_) => {
                        write!(f, "Stop-Triggered-Limit")
                    }
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
