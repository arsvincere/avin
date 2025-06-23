/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use super::{BarEvent, OrderEvent, TicEvent};

/// Market events, that is sending from broker to trader/tester/terminal.
///
/// # ru
/// Рыночные события: новый бар, новый тик, ордер исполнен,
/// ордер отклонен и тп. Передаются от брокера трейдеру, тестеру или в
/// терминал.
#[derive(Debug, Clone)]
pub enum Event {
    Bar(BarEvent),
    Tic(TicEvent),
    Order(OrderEvent),
}
impl Event {
    /// Return FIGI - Financial Instrument Global Identifier.
    ///
    /// # ru
    /// Возвращает FIGI - глобальный финансовый идентификатор
    /// инструмента по которому произошло событие.
    pub fn figi(&self) -> &String {
        match self {
            Self::Bar(e) => &e.figi,
            Self::Tic(e) => &e.figi,
            Self::Order(e) => e.iid.figi(),
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
