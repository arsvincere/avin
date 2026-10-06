// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::any::TypeId;

use avin_core::{Time, TimeRange, Year};
use avin_domain::{Bar, DataProvider, InstrumentId, MarketData, Tick};

use crate::{DataFrameExt, StorageError};

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
