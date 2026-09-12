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
    pub async fn cache(provider: DataProvider) -> Result<(), ServiceError> {
        match provider {
            DataProvider::TBank => cache_tbank().await,
            DataProvider::MoexAlgo => todo!(),
        }
    }

    pub fn clear(provider: DataProvider) -> Result<(), ServiceError> {
        println!("InstrumentService clear {provider}");

        todo!();
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

async fn cache_tbank() -> Result<(), ServiceError> {
    let packs = TBankProvider::fetch_instruments().await.map_err(|err| {
        let msg = "failed fetch instruments from T-Bank".to_string();
        ServiceError::Fetch {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;

    for pack in packs.into_iter() {
        InstrumentInfoStorage::save(
            pack.provider(),
            pack.exchange(),
            pack.category(),
            pack.instruments(),
        )
        .map_err(|err| {
            let msg = "failed save instruments cache".to_string();
            ServiceError::Store {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;
    }

    Ok(())
}
