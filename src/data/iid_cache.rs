/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::CACHE_DIR;
use crate::data::category::Category;
use crate::data::iid::IID;
use crate::data::source::Source;
use crate::utils::Cmd;
use bitcode::{Decode, Encode};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct IidCache {
    source: Source,
    category: Category,
    iids: Vec<IID>,
}

impl IidCache {
    pub fn new(source: Source, category: Category, iids: Vec<IID>) -> Self {
        Self {
            source,
            category,
            iids,
        }
    }
    pub fn from_bin(bytes: &Vec<u8>) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }

    pub fn find(
        exchange: &str,
        category: &Category,
        ticker: &str,
    ) -> Option<IID> {
        assert_eq!(exchange, "MOEX");
        assert_eq!(category, &Category::SHARE);

        let source = Source::TINKOFF;
        let category = Category::SHARE;
        let cache = IidCache::load(&source, &category).unwrap();

        for i in cache.iids {
            if i.exchange() == exchange
                && i.category() == category
                && i.ticker() == ticker
            {
                return Some(i);
            }
        }

        None
    }
    pub fn find_figi(figi: &str) -> Option<IID> {
        let source = Source::TINKOFF;
        let category = Category::SHARE;
        let cache = IidCache::load(&source, &category).unwrap();

        for i in cache.iids {
            if i.figi() == figi {
                return Some(i);
            }
        }

        None
    }

    pub fn save(cache: &IidCache) -> Result<(), &'static str> {
        let bytes = cache.to_bin();
        let path = cache.path();

        Cmd::write_bin(&bytes, &path).unwrap();

        Ok(())
    }
    pub fn load(
        source: &Source,
        category: &Category,
    ) -> Result<IidCache, &'static str> {
        let file_name = format!("{}.bin", category.to_string());
        let mut p = std::path::PathBuf::new();
        p.push(&CACHE_DIR);
        p.push(source.to_string());
        p.push(file_name);

        if !Cmd::is_exist(&p) {
            return Err("cache file not found");
        }

        let bytes = Cmd::read_bin(&p).unwrap();
        let cache = IidCache::from_bin(&bytes);

        Ok(cache)
    }

    fn path(&self) -> PathBuf {
        let file_name = format!("{}.bin", self.category.to_string());

        let mut p = std::path::PathBuf::new();
        p.push(&CACHE_DIR);
        p.push(&self.source.to_string());

        if !Cmd::is_exist(&p) {
            Cmd::make_dirs(&p).unwrap();
        }

        p.push(file_name);

        return p;
    }
}

impl std::fmt::Display for IidCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "IidCache={} {}",
            self.source.to_string(),
            self.category.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn path() {
        let source = Source::TINKOFF;
        let category = Category::SHARE;
        let iids = Vec::new();
        let cache = IidCache::new(source, category, iids);

        let path = Path::new("/home/alex/avin/usr/cache/tinkoff/SHARE.bin");
        assert_eq!(cache.path(), path);
    }
}
