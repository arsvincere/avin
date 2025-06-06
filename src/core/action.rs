/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::MarketData;
use crate::core::account::Account;
use crate::core::order::Order;
use crate::core::trade::Trade;
use crate::data::IID;

#[derive(Debug)]
pub enum Action {
    Post(PostOrderAction),
    Subscribe(SubscribeAction),
    TradeClosed(Trade),
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Post(a) => write!(f, "Action={}", a),
            Action::Subscribe(a) => write!(f, "Action={}", a),
            Action::TradeClosed(a) => write!(f, "Action={}", a),
        }
    }
}

#[derive(Debug)]
pub struct PostOrderAction {
    pub account: Account,
    pub iid: IID,
    pub strategy_name: String,
    pub order: Order,
}
impl PostOrderAction {
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
impl std::fmt::Display for PostOrderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PostOrderAction={} {}", self.iid, self.order)
    }
}

#[derive(Debug)]
pub struct SubscribeAction {
    pub iid: IID,
    pub market_data: MarketData,
}
impl SubscribeAction {
    pub fn new(iid: IID, market_data: MarketData) -> Self {
        Self { iid, market_data }
    }
}
impl std::fmt::Display for SubscribeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SubscribeAction={} {}", self.iid, self.market_data)
    }
}
