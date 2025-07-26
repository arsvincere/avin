/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{Iid, MarketData};

/// Message to subscribe or unsubscribe market data.
///
/// # ru
/// Сообщение о необходимости подписаться или отписаться на рыночные
/// данные. Их может формировать стратегия в ходе своей работы. Используются
/// в боевом режиме, тестере и терминале. В боевом режиме их принимает
/// `Trader` и передает брокеру. В тестере их принимает `Tester` и передает
/// `VirturalBroker`, в терминале они напрямую отправляются к брокеру.
///
/// Содержит идентификатор инструмента и вектор из типов рыночных данных.
#[derive(Debug)]
pub struct DataAction {
    pub iid: Iid,
    pub market_data_kinds: Vec<MarketData>,
}
impl DataAction {
    /// Create new data action
    ///
    /// # ru
    /// Создает новое действие с маркет данными
    pub fn new(iid: Iid, market_data_kinds: Vec<MarketData>) -> Self {
        Self {
            iid,
            market_data_kinds,
        }
    }
}
impl std::fmt::Display for DataAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DataAction={} {:?}", self.iid, self.market_data_kinds)
    }
}
