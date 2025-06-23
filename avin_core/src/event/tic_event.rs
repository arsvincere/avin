/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Tic;

/// That event sending from broker on every new tic.
///
/// # ru
/// Это событие отправляется брокером на каждом новом тике.
///
/// Содержит FIGI инструмента и собственно тик.
#[derive(Debug, Clone)]
pub struct TicEvent {
    pub figi: String,
    pub tic: Tic,
}
impl TicEvent {
    pub fn new(figi: String, tic: Tic) -> Self {
        Self { figi, tic }
    }
}
impl std::fmt::Display for TicEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TicEvent={} {}", self.figi, self.tic)
    }
}
