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
use avin_system::Workspace;

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
        let workspace = Workspace::get().map_err(|err| {
            let msg = "failed to resolve storage path";
            StorageError::path(msg, Some(err.into()))
        })?;

        let stage = workspace.dirs.market_data().join("stage");

        if crate::helper::is_exists(&stage)? {
            return Err(StorageError::save(
                "market data storage is dirty",
                None,
            ));
        }

        let write_operation = match key {
            MarketDataKey::Year {
                provider,
                iid,
                md,
                year,
            } => WriteOperation::new(provider, iid, md, year),

            _ => todo!("err"),
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
