/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use polars::prelude::{DataType, Field, Schema};

// NOTE: старые схемы для MOEX SuperCandle, сейчас нигде не используются.

pub struct DataSchema;
impl DataSchema {
    /// Polars dataframe schema for trades stat (SuperCandle).
    ///
    /// # ru
    /// Возвращает polars схему датафрейма для trades stat (SuperCandle).
    pub fn trades() -> Schema {
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

    /// Polars dataframe schema for orders stat (SuperCandle).
    ///
    /// # ru
    /// Возвращает polars схему датафрейма для orders stat (SuperCandle).
    pub fn orders() -> Schema {
        Schema::from_iter(vec![
            Field::new("ts_nanos".into(), DataType::Int64),
            Field::new("put_orders_b".into(), DataType::Int64),
            Field::new("put_orders_s".into(), DataType::Int64),
            Field::new("put_val_b".into(), DataType::Float64),
            Field::new("put_val_s".into(), DataType::Float64),
            Field::new("put_vol_b".into(), DataType::Int64),
            Field::new("put_vol_s".into(), DataType::Int64),
            Field::new("put_vwap_b".into(), DataType::Float64),
            Field::new("put_vwap_s".into(), DataType::Float64),
            Field::new("put_vol".into(), DataType::Int64),
            Field::new("put_val".into(), DataType::Float64),
            Field::new("put_orders".into(), DataType::Int64),
            Field::new("cancel_orders_b".into(), DataType::Int64),
            Field::new("cancel_orders_s".into(), DataType::Int64),
            Field::new("cancel_val_b".into(), DataType::Float64),
            Field::new("cancel_val_s".into(), DataType::Float64),
            Field::new("cancel_vol_b".into(), DataType::Int64),
            Field::new("cancel_vol_s".into(), DataType::Int64),
            Field::new("cancel_vwap_b".into(), DataType::Float64),
            Field::new("cancel_vwap_s".into(), DataType::Float64),
            Field::new("cancel_vol".into(), DataType::Int64),
            Field::new("cancel_val".into(), DataType::Float64),
            Field::new("cancel_orders".into(), DataType::Int64),
        ])
    }

    /// Polars dataframe schema for order book stat (SuperCandle).
    ///
    /// # ru
    /// Возвращает polars схему датафрейма для order book stat (SuperCandle).
    pub fn ob() -> Schema {
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
}
