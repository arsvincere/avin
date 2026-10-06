// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::any::TypeId;
use std::fs;
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
        let chunks = self.stage_chunks()?;

        self.validate_coverage(&chunks)?;

        let mut df = crate::helper::read_pqt(&chunks[0].1)?;

        for (_, path) in chunks.iter().skip(1) {
            let chunk = crate::helper::read_pqt(path)?;

            df.vstack_mut(&chunk).map_err(|err| {
                let msg = format!(
                    "failed to merge market data chunk '{}'",
                    path.display()
                );
                StorageError::save(msg, Some(err.into()))
            })?;
        }

        let key = MarketDataKey::year(
            self.provider,
            self.iid.clone(),
            self.md,
            self.year,
        );

        let path = key.path()?;

        crate::helper::write_pqt_atomic(&mut df, &path)?;

        let stage = self.stage_root()?;
        crate::helper::delete_dir(&stage)?;

        log::debug!("Finalized market data {}", path.display());

        Ok(())
    }

    pub fn abort(self) -> Result<(), StorageError> {
        todo!()
    }

    pub fn next_time(&self) -> Result<Option<Time>, StorageError> {
        todo!()
    }

    // private
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

    fn validate_coverage(
        &self,
        chunks: &[(TimeRange, PathBuf)],
    ) -> Result<(), StorageError> {
        let year = self.year.time_range();
        let mut expected = year.begin();

        for (range, _) in chunks {
            if range.begin() > expected {
                let missing =
                    TimeRange::new(expected, range.begin()).unwrap();

                let msg = format!("market data coverage gap {}", missing);

                return Err(StorageError::save(msg, None));
            }

            if range.begin() < expected {
                let msg = format!(
                    "market data coverage overlap at {}, expected begin {}",
                    range,
                    expected.dt(),
                );

                return Err(StorageError::save(msg, None));
            }

            expected = range.end();
        }

        if expected < year.end() {
            let missing = TimeRange::new(expected, year.end()).unwrap();

            let msg = format!("market data coverage gap {}", missing);

            return Err(StorageError::save(msg, None));
        }

        if expected > year.end() {
            let msg = format!(
                "market data coverage exceeds year {}, covered until {}",
                self.year,
                expected.dt(),
            );

            return Err(StorageError::save(msg, None));
        }

        Ok(())
    }

    fn stage_chunks(
        &self,
    ) -> Result<Vec<(TimeRange, PathBuf)>, StorageError> {
        let dir = self.stage_dir()?;

        if !crate::helper::is_exists(&dir)? {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&dir).map_err(|err| {
            let msg =
                format!("failed to read stage directory '{}'", dir.display());
            StorageError::load(msg, Some(err.into()))
        })?;

        let mut chunks = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|err| {
                let msg = format!(
                    "failed to read stage directory '{}'",
                    dir.display()
                );
                StorageError::load(msg, Some(err.into()))
            })?;

            let path = entry.path();

            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                let msg = format!("invalid stage file '{}'", path.display());
                StorageError::load(msg, None)
            })?;

            let stem =
                file_name.strip_suffix(".parquet").ok_or_else(|| {
                    let msg =
                        format!("invalid stage file '{}'", path.display());
                    StorageError::load(msg, None)
                })?;

            let (begin, end) = stem.split_once('_').ok_or_else(|| {
                let msg = format!("invalid stage file '{}'", path.display());
                StorageError::load(msg, None)
            })?;

            let begin = begin.parse::<i64>().map_err(|err| {
                let msg = format!("invalid stage file '{}'", path.display());
                StorageError::load(msg, Some(err.into()))
            })?;

            let end = end.parse::<i64>().map_err(|err| {
                let msg = format!("invalid stage file '{}'", path.display());
                StorageError::load(msg, Some(err.into()))
            })?;

            let range = TimeRange::new(Time::new(begin), Time::new(end))
                .map_err(|err| {
                    let msg = format!(
                        "invalid range in stage file '{}'",
                        path.display()
                    );
                    StorageError::load(msg, Some(err.into()))
                })?;

            chunks.push((range, path));
        }

        chunks.sort_by_key(|(range, _)| (range.begin(), range.end()));

        Ok(chunks)
    }

    fn stage_root(&self) -> Result<PathBuf, StorageError> {
        let workspace = Workspace::get().map_err(|err| {
            let msg = "failed to resolve storage path";
            StorageError::path(msg, Some(err.into()))
        })?;

        Ok(workspace.dirs.market_data().join("stage"))
    }

    fn stage_dir(&self) -> Result<PathBuf, StorageError> {
        let mut path = self.stage_root()?;

        path.push(self.provider.key());
        path.push(self.iid.exchange().key());
        path.push(self.iid.category().key());
        path.push(self.iid.ticker().to_string());
        path.push(self.md.key());
        path.push(self.year.to_string());

        Ok(path)
    }

    fn stage_path(&self, range: TimeRange) -> Result<PathBuf, StorageError> {
        let mut path = self.stage_dir()?;

        path.push(format!(
            "{}_{}.parquet",
            range.begin().ts(),
            range.end().ts(),
        ));

        Ok(path)
    }
}
