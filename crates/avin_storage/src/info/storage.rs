// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_core::DataProvider;
use avin_domain::Category;

use crate::StorageError;

pub struct InstrumentInfoStorage {}

impl InstrumentInfoStorage {
    pub fn exists(
        provider: DataProvider,
        category: Category,
    ) -> Result<bool, StorageError> {
        todo!()
    }

    pub fn save(
        provider: DataProvider,
        category: Category,
        df: DataFrame,
    ) -> Result<(), StorageError> {
        todo!()
    }

    pub fn load(
        provider: DataProvider,
        category: Category,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }

    pub fn delete(
        provider: DataProvider,
        category: Category,
    ) -> Result<(), StorageError> {
        todo!()
    }
}
