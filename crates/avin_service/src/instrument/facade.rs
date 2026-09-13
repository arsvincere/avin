// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_data::TBankProvider;
use avin_domain::{Category, DataProvider, InstrumentInfo, InstrumentList};
use avin_storage::InstrumentInfoStorage;

use crate::ServiceError;

use super::catalog::InstrumentCatalog;

pub struct InstrumentService {}

impl InstrumentService {
    pub fn cache(provider: DataProvider) -> Result<(), ServiceError> {
        match provider {
            DataProvider::TBank => cache_tbank(),
            DataProvider::MoexAlgo => todo!(),
        };

        Ok(())
    }

    pub fn clear(provider: DataProvider) -> Result<(), ServiceError> {
        println!("InstrumentService clear {provider}");

        Ok(())
    }

    pub fn find(
        provider: DataProvider,
        code: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        InstrumentCatalog::find(provider, code)
    }

    pub fn find_figi(
        provider: DataProvider,
        figi: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        InstrumentCatalog::find_figi(provider, figi)
    }

    pub fn list(
        provider: DataProvider,
        category: Category,
    ) -> Result<InstrumentList, ServiceError> {
        InstrumentCatalog::list(provider, category)
    }
}

fn cache_tbank() -> Result<(), ServiceError> {
    for c in Category::all() {
        let list = TBankProvider::fetch_instruments(*c).map_err(|err| {
            let msg = "TODO msg".to_string();
            ServiceError::Fetch {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        dbg!(&list.len());

        // TODO: InstrumentList -> df
        let df = DataFrame::empty();
        dbg!(&df);

        InstrumentInfoStorage::save(DataProvider::TBank, *c, df).map_err(
            |err| {
                let msg = "TODO msg".to_string();
                ServiceError::Store {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            },
        )?;
    }

    Ok(())
}
