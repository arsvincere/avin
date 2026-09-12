// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::collections::BTreeSet;
use std::fs::{self, File};

use polars::prelude::{Column, DataFrame, ParquetWriter};

use avin_domain::{Category, DataProvider, Exchange, InstrumentInfo};
use avin_system::Workspace;

use crate::StorageError;

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
            return Ok(());
        }

        // InstrumentInfo -> DataFrame
        let mut keys = BTreeSet::new();

        for instrument in instruments {
            keys.extend(instrument.raw_info().keys().cloned());
        }

        let mut columns = Vec::with_capacity(keys.len());

        for key in keys {
            let values = instruments
                .iter()
                .map(|instrument| {
                    instrument.raw_info().get(&key).map(String::as_str)
                })
                .collect::<Vec<_>>();

            columns.push(Column::new(key.into(), values));
        }

        let mut df =
            DataFrame::new(instruments.len(), columns).map_err(|err| {
                StorageError::Instrument {
                    message: "failed to create instruments DataFrame"
                        .to_string(),
                    source: Some(Box::new(err)),
                }
            })?;

        // Build path:
        // <instruments>/tbank/moex/share.parquet
        let workspace =
            Workspace::get().map_err(|err| StorageError::Instrument {
                message: "failed to get workspace".to_string(),
                source: Some(Box::new(err)),
            })?;

        let mut path = workspace.dirs.instruments().to_path_buf();
        path.push(provider.key());
        path.push(exchange.key());

        fs::create_dir_all(&path).map_err(|err| {
            StorageError::Instrument {
                message: format!(
                    "failed to create instruments directory '{}'",
                    path.display()
                ),
                source: Some(Box::new(err)),
            }
        })?;

        path.push(format!("{}.parquet", category.key()));

        // Save parquet
        let file =
            File::create(&path).map_err(|err| StorageError::Instrument {
                message: format!(
                    "failed to create instruments file '{}'",
                    path.display()
                ),
                source: Some(Box::new(err)),
            })?;

        ParquetWriter::new(file).finish(&mut df).map_err(|err| {
            StorageError::Instrument {
                message: format!(
                    "failed to save instruments file '{}'",
                    path.display()
                ),
                source: Some(Box::new(err)),
            }
        })?;

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
