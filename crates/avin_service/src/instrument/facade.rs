// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use avin_core::DataProvider;
use avin_domain::{Category, InstrumentInfo, InstrumentList};

use crate::ServiceError;

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
        println!("InstrumentService find {provider} {code}");

        todo!()
    }

    pub fn find_figi(
        provider: DataProvider,
        figi: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        println!("InstrumentService find_figi {provider} {figi}");

        todo!()
    }

    pub fn list(
        provider: DataProvider,
        category: Category,
    ) -> Result<InstrumentList, ServiceError> {
        println!("InstrumentService list {provider} {category}");

        todo!()
    }
}
