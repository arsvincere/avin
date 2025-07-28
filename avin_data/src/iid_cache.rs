/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

use cached::proc_macro::cached;
use polars::prelude::*;

use avin_utils::{AvinError, CFG, Cmd};

use crate::category::Category;
use crate::exchange::Exchange;
use crate::iid::Iid;
use crate::source::Source;

#[derive(Debug, PartialEq, Clone)]
pub struct IidCache {
    source: Source,
    category: Category,
    iid_df: DataFrame,
}

impl IidCache {
    pub fn find_iid(s: &str) -> Result<Iid, AvinError> {
        cached_find_iid(s.to_string())
    }
    pub fn find_figi(figi: &str) -> Result<Iid, AvinError> {
        cached_find_figi(figi.to_string())
    }

    // NOTE: эта функция не нужна по факту, загружать кэш имеет
    // смысл ради нахождения инструмента. В последней верссии
    // я инкапсулировал эту работу в эту структуру. Так что менеджер
    // дергает только методы find_figi find_iid а не капается в кэше
    // сам. Смысл - Manager - фасадный класс, он предоставляет интерфейс
    // к методам модуля. Ну может еще проверяет корректность аргументов
    // на входе. И все.
    // В будущем вообще эта работа будет переложена на сам источник.
    // Когда их будет несколько.
    // Потому что у каждого источника разные форматы, и пусть они
    // сами в своих ковыряются а на ружу выдают стандартизированный.
    // Пока тут захардкоден источник TINKOFF тип SHARE. Это попозже
    // выпилить надо будет, когда будут реализованы и SourceMoex
    // и SourceTinkoff. Им нужно будет наружу одинаковый интерфейс сделать.
    // Сейчас источника два, но можно считать что один. Потому что
    // моэкс используется только при загрузке данных. А так оно нифига
    // инфы об инструменте не передает достаточно. А брокер один
    // сейчас Т, и всю инфу с него и тащат инструменты когда с ним
    // работают. Будет другой брокер, тогда и наводить здесь обобщения...
    // Решать с какого источника грузить инфу. Пока это можно хардкодить.
    // pub fn load(source: Source, category: Category) -> Self {
    //     let path = ();
    //     let df = match Cmd::read_pqt(path) {
    //             Ok(df) => df,
    //             Err(err) => panic!("{err}"),
    //     }
    //     Self { source, category, iid_df: df }
    // }

    pub fn save(cache: IidCache) {
        // create file path
        let mut path = CFG.dir.cache();
        path.push(cache.source.name());
        path.push(format!("{}.parquet", cache.category.name()));

        // save parquet
        let mut df = cache.iid_df;
        Cmd::write_pqt(&mut df, &path);
    }
}

#[cached]
fn cached_find_iid(s: String) -> Result<Iid, AvinError> {
    // parse str
    let parts: Vec<&str> = s.split('_').collect();
    if parts.len() != 3 {
        return Err(AvinError::InvalidValue(s));
    };

    // convert values
    let _exchange = Exchange::from(parts[0]);
    let category = Category::from(parts[1]);
    let ticker = parts[2].to_uppercase();

    // load instrument info df
    let source = Source::TINKOFF;
    let df = cached_load_df(source, category).unwrap();

    // find row
    let mask = df
        .column("ticker")
        .unwrap()
        .str()
        .unwrap()
        .equal(ticker.as_str());
    let row = df.filter(&mask).unwrap();

    Iid::from_df(&row)
}
#[cached]
fn cached_find_figi(figi: String) -> Result<Iid, AvinError> {
    // load instrument info df
    let source = Source::TINKOFF;
    let category = Category::SHARE;
    let df = cached_load_df(source, category).unwrap();

    // find row
    let mask = df
        .column("figi")
        .unwrap()
        .str()
        .unwrap()
        .equal(figi.as_str());
    let row = df.filter(&mask).unwrap();

    Iid::from_df(&row)
}
#[cached]
fn cached_load_df(
    source: Source,
    category: Category,
) -> Result<DataFrame, AvinError> {
    // create file path
    let mut path = CFG.dir.cache();
    path.push(source.name());
    path.push(format!("{}.parquet", category.name()));

    // load parquet
    let df = Cmd::read_pqt(&path).unwrap();

    Ok(df)
}
