/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DATA_DIR;
use crate::data::category::Category;
use bitcode::{Decode, Encode};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct IID {
    info: HashMap<String, String>,
}
/// Instrument ID
impl IID {
    pub fn new(info: HashMap<String, String>) -> IID {
        // TODO: проверка присутствия необходимых полей:
        // exchange, category, ticker, figi, name, lot, step
        IID { info }
    }
    pub fn to_string(&self) -> String {
        format!(
            "{}_{}_{}",
            self.exchange(),
            self.category().to_string(),
            self.ticker()
        )
    }

    pub fn info(&self) -> &HashMap<String, String> {
        &self.info
    }
    pub fn exchange(&self) -> &String {
        self.info.get("exchange").unwrap()
    }
    pub fn category(&self) -> Category {
        let category = self.info.get("category").unwrap();

        Category::from(&category).unwrap()
    }
    pub fn ticker(&self) -> &String {
        self.info.get("ticker").unwrap()
    }
    pub fn figi(&self) -> &String {
        self.info.get("figi").unwrap()
    }
    pub fn name(&self) -> &String {
        self.info.get("name").unwrap()
    }
    pub fn lot(&self) -> u32 {
        let min_price_step = self.info.get("lot").unwrap().parse().unwrap();

        min_price_step
    }
    pub fn step(&self) -> f64 {
        let min_price_step = self.info.get("step").unwrap().parse().unwrap();

        min_price_step
    }
    pub fn path(&self) -> PathBuf {
        let mut p = std::path::PathBuf::new();
        p.push(&DATA_DIR);
        p.push(self.exchange());
        p.push(self.category().to_string());
        p.push(self.ticker());

        return p;
    }
}
impl std::fmt::Display for IID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "IID={}_{}_{}",
            self.exchange(),
            self.category().to_string(),
            self.ticker()
        )
    }
}
impl std::hash::Hash for IID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let s = self.to_string();
        s.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn new() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "SHARE".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());

        let iid = IID::new(info);
        assert_eq!(iid.exchange(), "MOEX");
        assert_eq!(iid.category(), Category::SHARE);
        assert_eq!(iid.ticker(), "SBER");
    }
    #[test]
    fn to_string() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());

        let iid = IID::new(info);
        let s = iid.to_string();
        assert_eq!("MOEX_SHARE_SBER", s);
    }
    #[test]
    fn path() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "Share".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());

        let iid = IID::new(info);
        let path = Path::new("/home/alex/avin/usr/data/MOEX/SHARE/SBER");
        assert_eq!(iid.path(), path);
    }
}
