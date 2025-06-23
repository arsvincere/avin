/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use cached::proc_macro::cached;
use polars::prelude::*;

use avin_utils::{AvinError, CFG, Cmd};

use crate::{Category, Exchange, Iid, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct IidCache {}
impl IidCache {
    pub fn find_iid(s: &str) -> Result<Iid, AvinError> {
        cached_find_iid(s.to_string())
    }
    pub fn find_figi(figi: &str) -> Result<Iid, AvinError> {
        cached_find_figi(figi.to_string())
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
    path.push(format!("{}.pqt", category.name()));

    // load parquet
    let df = Cmd::read_pqt(&path).unwrap();

    Ok(df)
}
