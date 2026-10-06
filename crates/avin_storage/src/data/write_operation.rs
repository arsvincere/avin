// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::any::TypeId;
use std::path::PathBuf;

use avin_core::{Time, TimeRange, Year};
use avin_domain::{Bar, DataProvider, InstrumentId, MarketData, Tick};
use avin_system::Workspace;

use crate::{DataFrameExt, MarketDataKey, StorageError};

pub struct WriteOperation {
    provider: DataProvider,
    iid: InstrumentId,
    md: MarketData,
    year: Year,
}

impl WriteOperation {
    pub fn new(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        year: Year,
    ) -> Self {
        Self {
            provider,
            iid,
            md,
            year,
        }
    }

    pub fn add<T: DataFrameExt + 'static>(
        &self,
        range: TimeRange,
        data: &[T],
    ) -> Result<(), StorageError> {
        self.validate_type::<T>()?;
        self.validate_range(range)?;

        let mut df = T::to_df(data)?;
        let path = self.stage_path(range)?;

        crate::helper::write_pqt_atomic(&mut df, &path)?;

        log::debug!("Added market data chunk {} {}", range, path.display());

        Ok(())
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

    fn validate_type<T: 'static>(&self) -> Result<(), StorageError> {
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

            MarketData::OrderBook => false,
        };

        if !valid {
            let msg =
                format!("data type does not match market data '{}'", self.md);
            return Err(StorageError::save(msg, None));
        }

        Ok(())
    }

    fn validate_range(&self, range: TimeRange) -> Result<(), StorageError> {
        if range.is_empty() {
            return Err(StorageError::save(
                "market data chunk range is empty",
                None,
            ));
        }

        let year = self.year.time_range();

        if range.begin() < year.begin() || range.end() > year.end() {
            let msg = format!(
                "market data chunk range {} is outside year {}",
                range, self.year
            );
            return Err(StorageError::save(msg, None));
        }

        Ok(())
    }

    fn stage_path(&self, range: TimeRange) -> Result<PathBuf, StorageError> {
        let workspace = Workspace::get().map_err(|err| {
            let msg = "failed to resolve storage path";
            StorageError::path(msg, Some(err.into()))
        })?;

        let mut path = workspace.dirs.market_data().to_path_buf();

        path.push("stage");
        path.push(self.provider.key());
        path.push(self.iid.exchange().key());
        path.push(self.iid.category().key());
        path.push(self.iid.ticker().to_string());
        path.push(self.md.key());
        path.push(self.year.to_string());

        path.push(format!(
            "{}_{}.parquet",
            range.begin().ts(),
            range.end().ts(),
        ));

        Ok(path)
    }
}
