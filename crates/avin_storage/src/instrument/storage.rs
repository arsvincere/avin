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

    pub fn inventory() -> Result<Vec<InstrumentInfoKey>, StorageError> {
        let mut keys = Vec::new();

        for p in DataProvider::all() {
            for e in Exchange::all() {
                for c in Category::all() {
                    let key = InstrumentInfoKey::category(*p, *e, *c);
                    let path = key.path()?;

                    if crate::helper::is_exists(&path)? {
                        keys.push(key);
                    }
                }
            }
        }

        Ok(keys)
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
        let key = InstrumentInfoKey::category(provider, exchange, category);
        let path = key.path()?;

        let df = crate::helper::read_pqt(&path)?;

        log::debug!("Loaded {}", path.display());

        Ok(df)
    }

    pub fn delete(key: &InstrumentInfoKey) -> Result<(), StorageError> {
        let path = key.path()?;

        if !(crate::helper::is_exists(&path)?) {
            log::debug!("Skip delete, path not exists {}", path.display());
            return Ok(());
        }

        if crate::helper::is_dir(&path)? {
            crate::helper::delete_dir(&path)?;
            log::debug!("Deleted {}", path.display());
            return Ok(());
        }

        if crate::helper::is_file(&path)? {
            crate::helper::delete_file(&path)?;
            log::debug!("Deleted {}", path.display());
            return Ok(());
        }

        let msg = format!(
            "failed to delete, not a file or directory {}",
            path.display()
        );
        Err(StorageError::delete(msg, None))
    }
}
