// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use polars::prelude::{DataType, Field, Schema};

pub(crate) struct StorageSchema {}

impl StorageSchema {
    pub(crate) fn bar() -> Schema {
        Schema::from_iter(vec![
            Field::new("datetime".into(), DataType::String),
            Field::new("open".into(), DataType::Float64),
            Field::new("high".into(), DataType::Float64),
            Field::new("low".into(), DataType::Float64),
            Field::new("close".into(), DataType::Float64),
            Field::new("volume".into(), DataType::Float64),
            Field::new("timestamp".into(), DataType::Int64),
        ])
    }
}
