/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DATA_DIR;
use bitcode::{Decode, Encode};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct IID {
    pub exchange: String,
    pub category: String,
    pub ticker: String,
}
/// Instrument ID
impl IID {
    pub fn from(s: &str) -> Result<IID, &'static str> {
        let parts: Vec<&str> = s.split("_").collect();
        if parts.len() != 3 {
            eprintln!("Fail to create IID from str: {s}");
            return Err("Invalid IID");
        };

        // TODO: пока работает только биржа MOEX
        let exchange = parts[0].to_uppercase();
        assert_eq!(exchange, "MOEX");

        // TODO: пока работает только тип инструмента SHARE
        let category = parts[1].to_uppercase();
        assert_eq!(category, "SHARE");

        // TODO: пока не сделал кэширование информации о доступных
        // инструментах работает только ниже указанные тикеры
        let ticker = parts[2].to_uppercase();
        assert!("GAZP LKOH MOEX ROSN SBER VTBR YDEX".contains(&ticker));

        let iid = IID {
            exchange,
            category,
            ticker,
        };
        Ok(iid)
    }
    pub fn to_string(&self) -> String {
        format!("{}_{}_{}", self.exchange, self.category, self.ticker)
    }

    pub fn path(&self) -> PathBuf {
        let mut p = std::path::PathBuf::new();
        p.push(&DATA_DIR);
        p.push(&self.exchange);
        p.push(&self.category);
        p.push(&self.ticker);

        return p;
    }
}
impl std::fmt::Display for IID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IID={} {} {}", self.exchange, self.category, self.ticker)
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
        let iid = IID::from("moex_share_sber").unwrap();
        assert_eq!(iid.exchange, "MOEX");
        assert_eq!(iid.category, "SHARE");
        assert_eq!(iid.ticker, "SBER");
    }
    #[test]
    fn to_string() {
        let iid = IID::from("moex_share_sber").unwrap();
        let s = iid.to_string();
        assert_eq!("MOEX_SHARE_SBER", s);
    }
    #[test]
    fn path() {
        let iid = IID::from("moex_share_sber").unwrap();
        let path = Path::new("/home/alex/avin/usr/data/MOEX/SHARE/SBER");
        assert_eq!(iid.path(), path);
    }
}
