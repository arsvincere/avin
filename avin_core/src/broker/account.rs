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
    id: String,
}
impl Account {
    /// Create new account.
    ///
    /// # ru
    /// Конструктор.
    pub fn new(name: &str, id: &str) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
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
    /// Возвращает id счета у брокера.
    pub fn id(&self) -> &String {
        &self.id
    }
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Account={} id={}", self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new() {
        let a = Account::new("Alex", "id=100500");
        assert_eq!(a.name(), "Alex");
        assert_eq!(a.id(), "id=100500");
    }
}
