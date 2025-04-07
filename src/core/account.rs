/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#[derive(Debug, PartialEq)]
pub struct Account {
    name: String,
    broker_id: String,
}

impl Account {
    pub fn new(name: &str, broker_id: &str) -> Self {
        Self {
            name: name.to_string(),
            broker_id: broker_id.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
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
