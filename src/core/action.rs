/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::account::Account;
use crate::core::order::Order;
use crate::core::trade::Trade;
use crate::data::IID;

pub enum Action {
    Post(PostOrderAction),
    TradeClosed(Trade),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Post(a) => write!(f, "Action={}", a),
            Action::TradeClosed(a) => write!(f, "Action={}", a),
        }
    }
}

#[derive(Debug)]
pub struct PostOrderAction {
    pub account: Account,
    pub iid: IID,
    pub order: Order,
    pub tx: tokio::sync::oneshot::Sender<Order>,
}
impl PostOrderAction {
    pub fn new(
        account: Account,
        iid: IID,
        order: Order,
        tx: tokio::sync::oneshot::Sender<Order>,
    ) -> Self {
        Self {
            account,
            iid,
            order,
            tx,
        }
    }
}
impl std::fmt::Display for PostOrderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PostOrderAction={} {}", self.iid, self.order)
    }
}
