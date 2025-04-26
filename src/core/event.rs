/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::account::Account;
use crate::core::bar::Bar;
use crate::core::order::Order;
use crate::core::tic::Tic;
use crate::core::timeframe::TimeFrame;
use crate::data::IID;

#[derive(Debug, Clone)]
pub enum Event {
    Bar(BarEvent),
    Tic(TicEvent),
    Order(OrderEvent),
}
impl Event {
    pub fn figi(&self) -> &String {
        match self {
            Self::Bar(e) => &e.figi,
            Self::Tic(e) => &e.figi,
            Self::Order(e) => &e.iid.figi(),
        }
    }
}
impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Event::Bar(e) => write!(f, "Event={}", e),
            Event::Tic(e) => write!(f, "Event={}", e),
            Event::Order(e) => write!(f, "Event={}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BarEvent {
    pub figi: String,
    pub tf: TimeFrame,
    pub bar: Bar,
}
impl BarEvent {
    pub fn new(figi: String, tf: TimeFrame, bar: Bar) -> Self {
        Self { figi, tf, bar }
    }
}
impl std::fmt::Display for BarEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BarEvent={} {} {}", self.figi, self.tf, self.bar)
    }
}

#[derive(Debug, Clone)]
pub struct TicEvent {
    pub figi: String,
    pub tic: Tic,
}
impl TicEvent {
    pub fn new(figi: String, tic: Tic) -> Self {
        Self { figi, tic }
    }
}
impl std::fmt::Display for TicEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TicEvent={} {}", self.figi, self.tic)
    }
}

#[derive(Debug, Clone)]
pub struct OrderEvent {
    pub account: Account,
    pub iid: IID,
    pub strategy_name: String,
    pub order: Order,
}
impl OrderEvent {
    pub fn new(
        account: Account,
        iid: IID,
        strategy_name: String,
        order: Order,
    ) -> Self {
        Self {
            account,
            iid,
            strategy_name,
            order,
        }
    }
}
impl std::fmt::Display for OrderEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "OrderEvent={} {} {} {}",
            self.account, self.iid, self.strategy_name, self.order
        )
    }
}
