/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// Broker account.
///
/// # ru
/// Брокерский счет.
///
/// Содержит имя счета и id для брокера. Используется при выставлении ордеров.
#[derive(Debug, PartialEq, Clone)]
pub struct Account {
    name: String,
    broker_id: String,
}
impl Account {
    /// Create new account.
    ///
    /// # ru
    /// Конструктор.
    pub fn new(name: &str, broker_id: &str) -> Self {
        Self {
            name: name.to_string(),
            broker_id: broker_id.to_string(),
        }
    }

    /// Return account name.
    ///
    /// # ru
    /// Возвращает имя счета.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Return account id.
    ///
    /// # ru
    /// Возвращает id счета.
    pub fn id(&self) -> &String {
        &self.broker_id
    }
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Account={} (id={})", self.name, self.broker_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new() {
        let a = Account::new("Alex", "broker_id=100500");
        assert_eq!(a.name(), "Alex");
        assert_eq!(a.id(), "broker_id=100500");
    }
}
