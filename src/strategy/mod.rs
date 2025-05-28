/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod every;
mod t_1m_t1;

pub use every::Every;
pub use t_1m_t1::T1mT1;

use tokio::sync::mpsc::UnboundedSender;

use crate::{Account, Action, IID, OrderEvent, Share};

#[derive(Debug)]
pub enum Strategy {
    Every(Every),
    T1mT1(T1mT1),
}

impl Strategy {
    pub fn new(
        name: &str,
        trader: UnboundedSender<Action>,
        account: Account,
        iid: &IID,
    ) -> Strategy {
        match name {
            "Every" => Strategy::Every(Every::new(trader, account, iid)),
            "T1mT1" => Strategy::T1mT1(T1mT1::new(trader, account, iid)),
            other => todo!("TODO_ME: {}", other),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Self::Every(s) => s.name(),
            Self::T1mT1(s) => s.name(),
        }
    }
    pub fn process(&mut self, share: &Share) {
        match self {
            Self::Every(s) => s.process(share),
            Self::T1mT1(s) => s.process(share),
        }
    }
    pub fn order_event(&mut self, e: OrderEvent) {
        match self {
            Self::Every(s) => s.order_event(e),
            Self::T1mT1(s) => s.order_event(e),
        }
    }
}
