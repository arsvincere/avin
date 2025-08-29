/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, Utc};

use crate::{Bar, TimeFrame};
use avin_data::Iid;

/// Message to get bars.
///
/// # ru
/// Сообщение о запросе баров у брокера за определенный период времени.
/// Их может формировать стратегия в ходе своей работы. Используются
/// в боевом режиме, тестере и терминале. В боевом режиме их принимает
/// `Trader` и передает брокеру. В тестере их принимает `Tester` и передает
/// `VirturalBroker`, в терминале они напрямую отправляются к брокеру.
///
/// Содержит идентификатор инструмента, таймфрейм, начало и конец периода,
/// закрытый диапазон DateTime Utc [from, till].
#[derive(Debug)]
pub struct GetBarsAction {
    pub iid: Iid,
    pub tf: TimeFrame,
    pub from: DateTime<Utc>,
    pub till: DateTime<Utc>,
    pub tx: tokio::sync::oneshot::Sender<Vec<Bar>>,
}
impl GetBarsAction {
    /// Create new get bars action
    ///
    /// # ru
    /// Создает новое действие с запросом баров
    pub fn new(
        iid: Iid,
        tf: TimeFrame,
        from: DateTime<Utc>,
        till: DateTime<Utc>,
        tx: tokio::sync::oneshot::Sender<Vec<Bar>>,
    ) -> Self {
        Self {
            iid,
            tf,
            from,
            till,
            tx,
        }
    }
}
impl std::fmt::Display for GetBarsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GetBarsAction={}-{} {} -> {}",
            self.iid, self.tf, self.from, self.till
        )
    }
}
