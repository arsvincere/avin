/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::Path;

use crate::Asset;
use avin_utils::Cmd;

/// Users asset list.
///
/// # ru
/// Пользовательский список активов. Содержит имя и вектор активов.
/// Списки хранятся в csv формате, в директории указанной в конфиге
/// пользователя, в папке "asset".
///
/// Используется в терминале. Или при создании тестов для стратегии,
/// можно указать целый список активов.
pub struct AssetList {
    name: String,
    assets: Vec<Asset>,
}
impl AssetList {
    /// Create new empty asset list.
    ///
    /// # ru
    /// Создает новый пустой список активов, с заданным именем.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            assets: Vec::new(),
        }
    }
    /// Load asset list.
    ///
    /// # ru
    /// Загружает пользовательский список активов.
    pub fn load(path: &Path) -> Result<Self, String> {
        // check file existing
        if !Cmd::is_exist(path) {
            let msg = format!("file not found {}", path.display());
            return Err(msg);
        };

        let text = Cmd::read(path).expect("Fail to read asset list file");
        let name = Cmd::name(path).unwrap();

        let result = Self::from_csv(&name, &text);
        match result {
            Err(why) => {
                let msg = format!("file {}, {}", path.display(), why);
                Err(msg)
            }
            ok => ok,
        }
    }
    /// Create asset list from csv.
    ///
    /// # ru
    /// Создает список активов с заданным именем и активами.
    ///
    /// Пример списка активов в csv формате:
    /// MOEX;SHARE;SBER;
    /// MOEX;SHARE;GAZP;
    /// MOEX;FUTURE;USDRUBF;
    /// MOEX;INDEX;IMOEX2;
    pub fn from_csv(name: &str, csv: &str) -> Result<Self, String> {
        let mut assets = Vec::new();

        for (n, line) in csv.lines().enumerate() {
            // line example: 'MOEX;SHARE;SBER;'
            let result = Asset::from_csv(line);
            match result {
                Ok(asset) => assets.push(asset),
                Err(why) => {
                    let msg = format!("line number {}, {}", n, why);
                    return Err(msg);
                }
            };
        }

        let asset_list = AssetList {
            name: name.into(),
            assets,
        };

        Ok(asset_list)
    }

    /// Return asset list name.
    ///
    /// # ru
    /// Возвращает имя списка.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Check for asset list is empty.
    ///
    /// # ru
    /// Проверка если ли в списке активы.
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
    /// Return assets count.
    ///
    /// # ru
    /// Возвращает количество активов в списке.
    pub fn len(&self) -> usize {
        self.assets().len()
    }
    /// Return reference to vector of assets.
    ///
    /// # ru
    /// Возвращает ссылку на вектор активов.
    pub fn assets(&self) -> &Vec<Asset> {
        &self.assets
    }
    /// Return asset by index.
    ///
    /// # ru
    /// Возвращает актив по индексу. Отсчет начинается с нуля. Если
    /// индекс больше чем количество активов в списке, или если список
    /// пуст, вернет None.
    pub fn get(&self, index: usize) -> Option<&Asset> {
        self.assets.get(index)
    }
    /// Return mutable reference to asset by index.
    ///
    /// # ru
    /// Возвращает мутабельную ссылку на актив по индексу.
    /// Отсчет начинается с нуля. Если индекс больше чем количество активов
    /// в списке, или если список пуст, вернет None.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Asset> {
        self.assets.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::*;

    #[test]
    fn new() {
        let asset_list = AssetList::new("xxx");
        assert_eq!(asset_list.name(), "xxx");
        assert_eq!(asset_list.assets().len(), 0);
    }

    #[test]
    fn from_csv() {
        let csv = "MOEX;SHARE;SBER;\n\
                   MOEX;SHARE;GAZP;\n\
                   MOEX;SHARE;AFKS;";

        let name = "My asset list";
        let asset_list = AssetList::from_csv(name, csv).unwrap();
        assert_eq!(asset_list.name(), "My asset list");
        assert_eq!(asset_list.assets().len(), 3);
    }

    #[test]
    #[should_panic]
    fn from_incorrect_csv() {
        let csv = "MOEX;SHARE;SBER;\n\
                   */-_(_{}#_()$#)(_);\n\
                   MOEX;SHARE;AFKS;";

        let name = "My asset list";
        let _ = AssetList::from_csv(name, csv).unwrap();
    }

    #[test]
    fn load() {
        let path = Path::new("/home/alex/trading/asset/xxx.csv");
        let asset_list = AssetList::load(path).unwrap();

        assert_eq!(asset_list.name(), "xxx");

        assert_ne!(asset_list.assets().len(), 0);
    }
}
