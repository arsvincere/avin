// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_domain::{Category, DataProvider, Exchange, InstrumentInfo};

use crate::{DataFrameExt, InstrumentInfoKey, StorageError};

pub struct InstrumentInfoStorage {}

impl InstrumentInfoStorage {
    pub fn exists(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Result<bool, StorageError> {
        todo!()
    }

    pub fn save(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
        instruments: &[InstrumentInfo],
    ) -> Result<(), StorageError> {
        if instruments.is_empty() {
            return Err(StorageError::save("no instruments to save", None));
        }

        let mut df = InstrumentInfo::to_df(instruments)?;

        let mut df = df
            .sort(["ticker"], Default::default())
            .expect("failed to sort instruments by ticker");

        let key = InstrumentInfoKey::category(provider, exchange, category);
        let path = key.path()?;

        crate::helper::write_pqt(&mut df, &path)?;

        log::debug!("Saved {}", path.display());

        Ok(())
    }

    pub fn load(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Result<DataFrame, StorageError> {
        todo!()
    }

    pub fn delete(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Result<(), StorageError> {
        todo!()
    }
}
