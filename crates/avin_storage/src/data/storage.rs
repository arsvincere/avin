// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::any::TypeId;

use polars::prelude::DataFrame;

use avin_core::{Quantity, Time, TimeRange, Year};
use avin_domain::{
    Bar, DataProvider, InstrumentId, MarketData, Tick, TimeFrame,
};

use crate::{DataFrameExt, MarketDataKey, StorageError};

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
        todo!()
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

pub struct WriteOperation {
    provider: DataProvider,
    iid: InstrumentId,
    md: MarketData,
    year: Year,
}

impl WriteOperation {
    pub fn add<T: DataFrameExt + 'static>(
        &self,
        range: TimeRange,
        data: &[T],
    ) -> Result<(), StorageError> {
        let valid = match self.md {
            MarketData::Tick => TypeId::of::<T>() == TypeId::of::<Tick>(),

            MarketData::Bar1M
            | MarketData::Bar5M
            | MarketData::Bar10M
            | MarketData::Bar15M
            | MarketData::Bar1H
            | MarketData::Bar4H
            | MarketData::BarDay
            | MarketData::BarWeek
            | MarketData::BarMonth => {
                TypeId::of::<T>() == TypeId::of::<Bar>()
            }

            MarketData::OrderBook => todo!(),
        };

        if !valid {
            todo!("err")
        }

        let df = T::to_df(data)?;

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
