/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::{DataType, Field, Schema};

/// Polars dataframe schema for bars
///
/// # ru
/// Возвращает polars схему датафрейма для баров
pub fn bar_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("open".into(), DataType::Float64),
        Field::new("high".into(), DataType::Float64),
        Field::new("low".into(), DataType::Float64),
        Field::new("close".into(), DataType::Float64),
        Field::new("volume".into(), DataType::Int64),
        Field::new("value".into(), DataType::Float64),
    ])
}

/// Polars dataframe schema for tics
///
/// # ru
/// Возвращает polars схему датафрейма для тиков
#[allow(unused)]
pub fn tic_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("direction".into(), DataType::String),
        Field::new("lots".into(), DataType::Int64),
        Field::new("price".into(), DataType::Float64),
        Field::new("value".into(), DataType::Float64),
        Field::new("session".into(), DataType::Int8),
        Field::new("tradeno".into(), DataType::Int64),
    ])
}

/// Polars dataframe schema for trades stat (SuperCandle)
///
/// # ru
/// Возвращает polars схему датафрейма для trades stat (SuperCandle)
pub fn trades_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("open".into(), DataType::Float64),
        Field::new("high".into(), DataType::Float64),
        Field::new("low".into(), DataType::Float64),
        Field::new("close".into(), DataType::Float64),
        Field::new("std".into(), DataType::Float64),
        Field::new("vol".into(), DataType::Int64),
        Field::new("val".into(), DataType::Float64),
        Field::new("trades".into(), DataType::Int64),
        Field::new("vwap".into(), DataType::Float64),
        Field::new("change".into(), DataType::Float64),
        Field::new("trades_b".into(), DataType::Int64),
        Field::new("trades_s".into(), DataType::Int64),
        Field::new("val_b".into(), DataType::Float64),
        Field::new("val_s".into(), DataType::Float64),
        Field::new("vol_b".into(), DataType::Int64),
        Field::new("vol_s".into(), DataType::Int64),
        Field::new("disb".into(), DataType::Float64),
        Field::new("vwap_b".into(), DataType::Float64),
        Field::new("vwap_s".into(), DataType::Float64),
    ])
}

/// Polars dataframe schema for order book stat (SuperCandle)
///
/// # ru
/// Возвращает polars схему датафрейма для order book stat (SuperCandle)
pub fn ob_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new("ts_nanos".into(), DataType::Int64),
        Field::new("spread_bbo".into(), DataType::Float64),
        Field::new("spread_lv10".into(), DataType::Float64),
        Field::new("spread_1mio".into(), DataType::Float64),
        Field::new("levels_b".into(), DataType::Int64),
        Field::new("levels_s".into(), DataType::Int64),
        Field::new("vol_b".into(), DataType::Int64),
        Field::new("vol_s".into(), DataType::Int64),
        Field::new("val_b".into(), DataType::Int64),
        Field::new("val_s".into(), DataType::Int64),
        Field::new("imbalance_vol_bbo".into(), DataType::Float64),
        Field::new("imbalance_val_bbo".into(), DataType::Float64),
        Field::new("imbalance_vol".into(), DataType::Float64),
        Field::new("imbalance_val".into(), DataType::Float64),
        Field::new("vwap_b".into(), DataType::Float64),
        Field::new("vwap_s".into(), DataType::Float64),
        Field::new("vwap_b_1mio".into(), DataType::Float64),
        Field::new("vwap_s_1mio".into(), DataType::Float64),
    ])
}
