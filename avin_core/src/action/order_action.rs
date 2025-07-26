/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Account, Iid, Order};

/// Message to post or cancel order.
///
/// # ru
/// Сообщение о необходимости выставить или отменить ордер.
/// Их формирует стратегия в ходе своей работы. Используются
/// в боевом режиме, тестере и терминале. В боевом режиме их принимает
/// `Trader` и передает брокеру. В тестере их принимает `Tester` и передает
/// `VirturalBroker`, в терминале они напрямую отправляются к брокеру.
///
/// Содержит аккаунт, идентификатор инструмента, имя владельца и сам ордер.
///
/// В боевом режиме и тестере, в качестве имени владельца, следует указывать
/// название стратегии, которая создала этот ордер. Чтобы при
/// исполнении/изменении ордера ответ (`OrderEvent`) передали обратно
/// в соответствующую стратегию.
///
/// А в режиме терминала автоматически указывается owner="User".
#[derive(Debug, Clone)]
pub struct OrderAction {
    pub account: Account,
    pub iid: Iid,
    pub owner: String,
    pub order: Order,
}
impl OrderAction {
    /// Create new order action
    ///
    /// # ru
    /// Создает новое действие с ордером
    pub fn new(
        account: Account,
        iid: Iid,
        owner: &str,
        order: Order,
    ) -> Self {
        Self {
            account,
            iid,
            owner: owner.to_string(),
            order,
        }
    }
}
impl std::fmt::Display for OrderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OrderAction={} {}", self.iid, self.order)
    }
}
