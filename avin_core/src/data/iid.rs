/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;
use std::path::PathBuf;

use bitcode::{Decode, Encode};
use polars::frame::DataFrame;

use avin_utils::{AvinError, CFG};

/// Iid - Instrument ID
///
/// # ru
/// Идентификатор инструмента. Обертка над HashMap содержащим информацию
/// по инструменту: биржа, категория, тикер, название компании, размер лота,
/// минимальный шаг цены и тп.
///
/// Не предполагается ручное создание объектов этой структуры. Воспользуйтесь
/// методом [`crate::Manager::find_iid`]. Так же iid автоматически находится
/// и включается в актив при создании: [`crate::Asset::new`]
#[derive(Debug, PartialEq, Eq, Encode, Decode, Clone)]
pub struct Iid {
    info: HashMap<String, String>,
}
impl Iid {
    /// Create Instrument ID, don't use this method directly
    ///
    /// # ru
    /// Конструктор. Не используйте этот метод напрямую.
    pub fn new(info: HashMap<String, String>) -> Iid {
        debug_assert_ne!(info.get("exchange"), None);
        debug_assert_ne!(info.get("category"), None);
        debug_assert_ne!(info.get("ticker"), None);
        debug_assert_ne!(info.get("figi"), None);
        debug_assert_ne!(info.get("name"), None);
        debug_assert_ne!(info.get("lot"), None);
        debug_assert_ne!(info.get("step"), None);

        Iid { info }
    }
    /// Create Instrument ID from dataframe with one row.
    /// Don't use this method directly.
    ///
    /// # ru
    /// Конструктор. Не используйте этот метод напрямую.
    pub fn from_df(df: &DataFrame) -> Result<Iid, AvinError> {
        debug_assert_eq!(df.height(), 1);

        let columns = [
            "exchange",
            "exchange_specific",
            "category",
            "ticker",
            "figi",
            "country",
            "currency",
            "sector",
            "class_code",
            "isin",
            "uid",
            "name",
            "lot",
            "step",
            "long",
            "short",
            "long_qual",
            "short_qual",
            "first_1m",
            "first_d",
        ];

        let mut info = HashMap::new();
        for i in columns {
            info.insert(i.to_string(), df.get_as_str(i));
        }

        Ok(Iid::new(info))
    }

    /// Return reference to HashMap with instrument info.
    ///
    /// # ru
    /// Возвращает ссылку на HashMap со всей имеющейся информацией
    /// об инструменте.
    pub fn info(&self) -> &HashMap<String, String> {
        &self.info
    }
    /// Return exchange.
    ///
    /// # ru
    /// Возвращает название биржи на которой торгуется инструмент.
    pub fn exchange(&self) -> &String {
        self.info.get("exchange").unwrap()
    }
    /// Return category.
    ///
    /// # ru
    /// Возвращает название категории инструмента: акция, облигация,
    /// индекс, фьючерс и тп.
    pub fn category(&self) -> &String {
        self.info.get("category").unwrap()
    }
    /// Return ticker.
    ///
    /// # ru
    /// Возвращает тикер инструмента.
    pub fn ticker(&self) -> &String {
        self.info.get("ticker").unwrap()
    }
    /// Return FIGI - Financial Instrument Global Identifier.
    ///
    /// # ru
    /// Возвращает FIGI - глобальный финансовый идентификатор
    /// инструмента. Используется брокером при выставлении ордера,
    /// так как тикер не является уникальным идентификатором, однозначно
    /// определяющим актив.
    pub fn figi(&self) -> &String {
        self.info.get("figi").unwrap()
    }
    /// Return instrument name.
    ///
    /// # ru
    /// Возвращает название инструмента
    pub fn name(&self) -> &String {
        self.info.get("name").unwrap()
    }
    /// Return lot size.
    ///
    /// # ru
    /// Возвращает размер лота.
    pub fn lot(&self) -> u32 {
        self.info.get("lot").unwrap().parse().unwrap()
    }
    /// Return the minimum price increment.
    ///
    /// # ru
    /// Возвращает минимальный шаг цены.
    pub fn step(&self) -> f64 {
        self.info.get("step").unwrap().parse().unwrap()
    }
    /// Return the dir path with market data of instrument.
    ///
    /// # ru
    /// Возвращает путь к каталогу с рыночными данными инструмента.
    pub fn path(&self) -> PathBuf {
        let mut p = CFG.dir.data();
        p.push(self.exchange());
        p.push(self.category());
        p.push(self.ticker());

        p
    }
}
impl std::fmt::Display for Iid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Iid={}_{}_{}",
            self.exchange(),
            self.category(),
            self.ticker()
        )
    }
}
impl std::hash::Hash for Iid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.figi().hash(state);
    }
}

// private
trait Getter {
    fn get_as_str(&self, col: &str) -> String;
}
impl Getter for DataFrame {
    fn get_as_str(&self, column_name: &str) -> String {
        self.column(column_name)
            .unwrap()
            .str()
            .unwrap()
            .first()
            .unwrap()
            .to_string()
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
        info.insert("name".to_string(), "Сбер Банк".to_string());
        info.insert("lot".to_string(), "10".to_string());
        info.insert("step".to_string(), "0.01".to_string());

        let iid = Iid::new(info);
        assert_eq!(iid.exchange(), "MOEX");
        assert_eq!(iid.category(), "SHARE");
        assert_eq!(iid.ticker(), "SBER");
        assert_eq!(iid.figi(), "BBG004730N88");
        assert_eq!(iid.name(), "Сбер Банк");
        assert_eq!(iid.lot(), 10);
        assert_eq!(iid.step(), 0.01);
    }
    #[test]
    fn to_string() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "SHARE".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        info.insert("name".to_string(), "Сбер Банк".to_string());
        info.insert("lot".to_string(), "10".to_string());
        info.insert("step".to_string(), "0.01".to_string());

        let iid = Iid::new(info);
        let s = iid.to_string();
        assert_eq!("Iid=MOEX_SHARE_SBER", s);
    }
    #[test]
    fn path() {
        let mut info = HashMap::new();
        info.insert("exchange".to_string(), "MOEX".to_string());
        info.insert("category".to_string(), "SHARE".to_string());
        info.insert("ticker".to_string(), "SBER".to_string());
        info.insert("figi".to_string(), "BBG004730N88".to_string());
        info.insert("name".to_string(), "Сбер Банк".to_string());
        info.insert("lot".to_string(), "10".to_string());
        info.insert("step".to_string(), "0.01".to_string());

        let iid = Iid::new(info);
        let path = Path::new("/home/alex/trading/data/MOEX/SHARE/SBER");
        assert_eq!(iid.path(), path);
    }
}
