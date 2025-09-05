/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::Account;

/// Message to get account.
///
/// # ru
/// Сообщение о запросе аккаунта у брокера.
/// Их может формировать стратегия в ходе своей работы. Используются
/// в боевом режиме, тестере и терминале. В боевом режиме их принимает
/// `Trader` и передает брокеру. В тестере их принимает `Tester` и передает
/// `VirturalBroker`, в терминале они напрямую отправляются к брокеру.
///
/// Содержит имя аккаунта и канал для передачи ответа.
#[derive(Debug)]
pub struct GetAccountAction {
    pub name: String,
    pub tx: tokio::sync::oneshot::Sender<Account>,
}
impl GetAccountAction {
    /// Create new get account action.
    ///
    /// # ru
    /// Создает новое действие с запросом аккаунта у брокера.
    pub fn new(
        name: &str,
        tx: tokio::sync::oneshot::Sender<Account>,
    ) -> Self {
        Self {
            name: name.to_string(),
            tx,
        }
    }
}
impl std::fmt::Display for GetAccountAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GetAccountAction={}", self.name)
    }
}
