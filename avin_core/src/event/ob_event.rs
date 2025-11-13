/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::OrderBook;

/// That event sending from broker on every new order book state.
///
/// # ru
/// Это событие отправляется брокером на каждом новом состоянии стакана.
///
/// Содержит FIGI инструмента и собственно стакан.
#[derive(Debug, Clone)]
pub struct OrderBookEvent {
    pub figi: String,
    pub ob: OrderBook,
}
impl OrderBookEvent {
    pub fn new(figi: String, ob: OrderBook) -> Self {
        Self { figi, ob }
    }
}
impl std::fmt::Display for OrderBookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OrderBookEvent={} {}", self.figi, self.ob)
    }
}
