/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

use crate::{Direction, PostedLimitOrder, PostedMarketOrder};

/// List for select stop order kind.
///
/// # ru
/// Тип стоп ордера, в Тинькофф брокере нет как таковых отложенных ордеров,
/// только StopLoss/TakeProfit, и для того чтобы выставить отложенный ордер
/// на покупку нужно использовать StopLoss/TakeProfit в зависимости от текущей
/// цены бумаги. Если нужно купить по цене ниже текущей то нужно использовать
/// TakeProfit, а если выше текущей то StopLoss. Так себе логика, но как есть.
#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum StopOrderKind {
    StopLoss,
    TakeProfit,
}

/// Wrapper for stop order statuses.
///
/// # ru
/// Обертка для статусов стоп ордеров.
///
/// Состояние ордеров реализовано идиоматичным для Rust путем -  через
/// отдельные типы. Этим не очень удобно пользоваться, зато компилятор
/// следит за корректностью работы с ордерами. Например нельзя
/// отменить ордер который еще не выставлен.
///
/// В отличии от лимитных и рыночных ордеров, стоп ордера не могут
/// быть исполнены. Они срабатывают - в результате создается рыночный
/// или лимитный ордер, который будет исполняться. Такое поведение
/// стоп ордеров у брокера, поэтому в системе сделано так же.
///
/// В реализации ордеров возможны изменения, поэтому подробной
/// документации по методам пока нет.
#[derive(Debug, PartialEq, Decode, Encode, Clone)]
pub enum StopOrder {
    New(NewStopOrder),
    Posted(PostedStopOrder),
    Triggered(TriggeredStopOrder),
    Rejected(RejectedStopOrder),
    Canceled(CanceledStopOrder),
}
impl StopOrder {
    #[allow(clippy::new_ret_no_self)]
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

    pub fn as_new(self) -> Option<NewStopOrder> {
        match self {
            StopOrder::New(o) => Some(o),
            Self::Posted(_) => None,
            Self::Triggered(_) => None,
            Self::Rejected(_) => None,
            Self::Canceled(_) => None,
        }
    }
    pub fn as_posted(self) -> Option<PostedStopOrder> {
        match self {
            StopOrder::New(_) => None,
            Self::Posted(o) => Some(o),
            Self::Triggered(_) => None,
            Self::Rejected(_) => None,
            Self::Canceled(_) => None,
        }
    }
    pub fn as_triggered(self) -> Option<TriggeredStopOrder> {
        match self {
            StopOrder::New(_) => None,
            Self::Posted(_) => None,
            Self::Triggered(o) => Some(o),
            Self::Rejected(_) => None,
            Self::Canceled(_) => None,
        }
    }
    pub fn as_rejected(self) -> Option<RejectedStopOrder> {
        match self {
            StopOrder::New(_) => None,
            Self::Posted(_) => None,
            Self::Triggered(_) => None,
            Self::Rejected(o) => Some(o),
            Self::Canceled(_) => None,
        }
    }
    pub fn as_canceled(self) -> Option<CanceledStopOrder> {
        match self {
            StopOrder::New(_) => None,
            Self::Posted(_) => None,
            Self::Triggered(_) => None,
            Self::Rejected(_) => None,
            Self::Canceled(o) => Some(o),
        }
    }

    pub fn is_new(&self) -> bool {
        matches!(self, StopOrder::New(_))
    }
    pub fn is_posted(&self) -> bool {
        matches!(self, StopOrder::Posted(_))
    }
    pub fn is_triggered(&self) -> bool {
        matches!(self, StopOrder::Triggered(_))
    }
    pub fn is_rejected(&self) -> bool {
        matches!(self, StopOrder::Rejected(_))
    }
    pub fn is_canceled(&self) -> bool {
        matches!(self, StopOrder::Canceled(_))
    }

    pub fn broker_id(&self) -> Option<&String> {
        match self {
            Self::New(_) => None,
            Self::Posted(o) => Some(&o.broker_id),
            Self::Canceled(o) => Some(&o.broker_id),
            Self::Rejected(_) => None,
            Self::Triggered(_) => None,
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

/// Wrapper for triggered stop order.
///
/// # ru
/// Обертка для сработавшего стоп ордера.
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
