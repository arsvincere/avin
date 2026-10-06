// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_core::{Quantity, Time, TimeRange, Year};
use avin_domain::{
    Bar, DataProvider, InstrumentId, MarketData, Tick, TimeFrame,
};

use crate::{DataFrameExt, MarketDataKey, StorageError};

use super::WriteOperation;

pub struct MarketDataStorage {}

impl MarketDataStorage {
    pub fn status() -> Result<StorageStatus, StorageError> {
        todo!()
    }

    pub fn exists(key: MarketDataKey) -> Result<bool, StorageError> {
        todo!()
    }

    pub fn inventory() -> Result<Vec<MarketDataKey>, StorageError> {
        todo!()
    }

    pub fn compact() -> Result<(), StorageError> {
        todo!()
    }

    pub fn write(key: MarketDataKey) -> Result<WriteOperation, StorageError> {
        // TODO: проверка на dirty сначала

        let write_operation = match key {
            MarketDataKey::Year {
                provider,
                iid,
                md,
                year,
            } => WriteOperation::new(provider, iid, md, year),
            other => todo!("err"),
        };

        Ok(write_operation)
    }

    pub fn load_range(
        key: MarketDataKey,
        range: TimeRange,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }

    pub fn load_latest(
        key: MarketDataKey,
        quantity: Quantity,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }

    pub fn delete(key: MarketDataKey) -> Result<(), StorageError> {
        todo!()
    }
}

pub enum StorageStatus {
    Clean,
    Dirty(WriteOperation),
}
