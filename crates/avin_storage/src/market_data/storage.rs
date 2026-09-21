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
use avin_domain::{DataProvider, InstrumentId, MarketData};

use crate::{MarketDataKey, StorageError};

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

    pub fn write(
        key: MarketDataKey, // или provider, iid, md, year??? key может быть не валиден...
    ) -> Result<WriteOperation, StorageError> {
        todo!()
    }

    pub fn delete(key: MarketDataKey) -> Result<(), StorageError> {
        todo!()
    }

    pub fn load_range(
        provider: DataProvider,
        iid: &InstrumentId,
        md: MarketData,
        range: TimeRange,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }

    pub fn load_latest(
        provider: DataProvider,
        iid: &InstrumentId,
        md: MarketData,
        quantity: Quantity,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }
}

pub enum StorageStatus {
    Clean,
    Dirty(WriteOperation),
}

pub struct WriteOperation {
    provider: DataProvider,
    iid: InstrumentId,
    md: MarketData,
    year: Year,
}

impl WriteOperation {
    pub fn add(&self, chunk: DataChunk) -> Result<(), StorageError> {
        todo!()
    }
    pub fn finalize(self) -> Result<(), StorageError> {
        todo!()
    }
    pub fn abort(self) -> Result<(), StorageError> {
        todo!()
    }
    pub fn next_time(&self) -> Result<Option<Time>, StorageError> {
        todo!()
    }
}

pub struct DataChunk {
    coverage_range: TimeRange,
    df: DataFrame,
}
