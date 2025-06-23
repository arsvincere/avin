/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Account, Iid, Order};

/// That event sending from broker on order changed.
///
/// # ru
/// Это событие отправляется брокером при изменении ордера: выставлен,
/// отклонен, частично исполнен, исполнен, отменен. И на каждой транзакции
/// по ордеру тоже генерируется.
///
/// Содержит аккаунт на котором выставлен ордер, идентификатор инструмента,
/// имя владельца и сам ордер.
///
/// В боевом режиме и тестере, в качестве имени владельца, будет указано
/// название стратегии, которая создала `Action` для выставления этого ордер.
///
/// А в режиме терминала, когда ордер выставлен вручную пользователем,
/// автоматически указывается owner="User".
#[derive(Debug, Clone)]
pub struct OrderEvent {
    pub account: Account,
    pub iid: Iid,
    pub owner: String,
    pub order: Order,
}
impl OrderEvent {
    pub fn new(
        account: Account,
        iid: Iid,
        owner: String,
        order: Order,
    ) -> Self {
        Self {
            account,
            iid,
            owner,
            order,
        }
    }
}
impl std::fmt::Display for OrderEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "OrderEvent={} {} {} {}",
            self.account, self.iid, self.owner, self.order
        )
    }
}
