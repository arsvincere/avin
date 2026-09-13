// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after impl
#![allow(unused)]

use avin_domain::{Category, DataProvider, InstrumentInfo, InstrumentList};

use crate::ServiceError;

use super::catalog::InstrumentCatalog;

pub struct InstrumentService {}

impl InstrumentService {
    pub fn cache(provider: DataProvider) -> Result<(), ServiceError> {
        println!("InstrumentService cache {provider}");

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
