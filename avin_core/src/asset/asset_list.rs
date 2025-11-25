/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::Path;

use crate::Asset;
use avin_utils::{AvinError, CFG, Cmd};

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
    pub fn load(path: &Path) -> Result<Self, AvinError> {
        // check file existing
        if !Cmd::is_exist(path) {
            let msg = format!("file not found {}", path.display());
            let e = AvinError::NotFound(msg);
            return Err(e);
        };

        let text = Cmd::read(path).expect("Fail to read asset list file");
        let name = Cmd::name(path).unwrap();
        let mut assets = Vec::new();

        for (n, line) in text.lines().enumerate() {
            let result = Asset::new(line);
            match result {
                Ok(asset) => assets.push(asset),
                Err(why) => {
                    let msg = format!("line number {n}, {why}");
                    let e = AvinError::IOError(msg);
                    return Err(e);
                }
            };
        }

        let asset_list = AssetList { name, assets };

        Ok(asset_list)
    }
    pub fn load_name(name: &str) -> Result<Self, AvinError> {
        let mut path = CFG.dir.asset();
        path.push(name);

        AssetList::load(&path)
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
    pub fn assets_mut(&mut self) -> &mut Vec<Asset> {
        &mut self.assets
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
    /// Find asset in asset list by figi.
    ///
    /// # ru
    /// Возвращает ссылку на актив с заданным figi.
    pub fn find_figi(&self, figi: &str) -> Option<&Asset> {
        self.assets.iter().find(|&asset| asset.figi() == figi)
    }
    /// Find asset in asset list by figi.
    ///
    /// # ru
    /// Возвращает ссылку на актив с заданным figi.
    pub fn find_figi_mut(&mut self, figi: &str) -> Option<&mut Asset> {
        self.assets.iter_mut().find(|asset| asset.figi() == figi)
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
    fn load() {
        let path = Path::new("/home/alex/trading/usr/asset/xxx");
        let asset_list = AssetList::load(path).unwrap();

        assert_eq!(asset_list.name(), "xxx");

        assert_ne!(asset_list.assets().len(), 0);
    }
}
