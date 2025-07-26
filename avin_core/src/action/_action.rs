/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Trade;

use super::DataAction;
use super::OrderAction;

/// Comands or messages, that is sending from strategy to trader/broker.
///
/// # ru
/// Действие - это по сути команды или сообщения которые стратегия отправляет
/// трейдеру: выставить ордер, отменить ордер, подписаться на рыночные данные,
/// проинформировать что открыт трейд и тд.
#[derive(Debug)]
pub enum Action {
    Post(OrderAction),
    Cancel(OrderAction),
    Subscribe(DataAction),
    Unsubscribe(DataAction),
    TradeOpened(Trade),
    TradeClosed(Trade),
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Post(a) => write!(f, "Action={a}"),
            Action::Cancel(a) => write!(f, "Action={a}"),
            Action::Subscribe(a) => write!(f, "Action={a}"),
            Action::Unsubscribe(a) => write!(f, "Action={a}"),
            Action::TradeOpened(a) => write!(f, "Action={a}"),
            Action::TradeClosed(a) => write!(f, "Action={a}"),
        }
    }
}
