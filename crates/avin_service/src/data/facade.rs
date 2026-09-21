// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_core::{TimeRange, Year};
use avin_domain::{DataProvider, InstrumentId, MarketData};
use avin_storage::{MarketDataKey, StorageStatus};

use crate::ServiceError;

pub struct DataService {}

impl DataService {
    pub fn download(
        provider: DataProvider,
        code: String,
        md: MarketData,
        year: Year,
    ) -> Result<(), ServiceError> {
        todo!();
    }

    pub fn load(
        provider: DataProvider,
        iid: &InstrumentId,
        md: MarketData,
        range: TimeRange,
    ) -> Result<DataFrame, ServiceError> {
        todo!();
    }

    pub fn compact() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn prune() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn delete(key: MarketDataKey) -> Result<(), ServiceError> {
        todo!();
    }

    pub fn sync(
        provider: DataProvider,
        code: Option<String>,
        md: Option<MarketData>,
        year: Option<Year>,
        force: bool,
    ) -> Result<(), ServiceError> {
        todo!();
    }

    pub fn resume() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn abort() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn status() -> Result<StorageStatus, ServiceError> {
        todo!();
    }
}
