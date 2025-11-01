/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Trade;

use super::GetAccountAction;
use super::GetBarsAction;
use super::OrderAction;
use super::StreamAction;

/// Comands or messages, that is sending from strategy to trader/broker.
///
/// # ru
/// Действие - это по сути сообщения, которые стратегия отправляет
/// трейдеру: выставить ордер, отменить ордер, подписаться на рыночные данные,
/// проинформировать что открыт трейд и тд.
#[derive(Debug)]
pub enum Action {
    Post(OrderAction),
    Cancel(OrderAction),

    TradeClosed(Trade),
    TradeOpened(Trade),

    Subscribe(StreamAction),
    Unsubscribe(StreamAction),

    GetAccount(GetAccountAction),
    GetBars(GetBarsAction),
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::GetAccount(a) => write!(f, "Action={a}"),
            Action::GetBars(a) => write!(f, "Action={a}"),
            Action::Post(a) => write!(f, "Action={a}"),
            Action::Cancel(a) => write!(f, "Action={a}"),
            Action::Subscribe(a) => write!(f, "Action={a}"),
            Action::Unsubscribe(a) => write!(f, "Action={a}"),
            Action::TradeOpened(a) => write!(f, "Action={a}"),
            Action::TradeClosed(a) => write!(f, "Action={a}"),
        }
    }
}
