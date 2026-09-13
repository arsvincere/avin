// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after impl
#![allow(unused)]

use avin_core::DataProvider;
use avin_domain::{Category, InstrumentId, InstrumentInfo, InstrumentList};

use crate::ServiceError;

pub(super) struct InstrumentCatalog {}

impl InstrumentCatalog {
    pub fn find(
        provider: DataProvider,
        code: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        println!("InstrumentCatalog find {provider} {code}");

        todo!()
    }

    pub fn find_iid(
        provider: DataProvider,
        iid: &InstrumentId,
    ) -> Result<InstrumentInfo, ServiceError> {
        println!("InstrumentCatalog find_figi {provider} {iid}");

        todo!()
    }

    pub fn find_figi(
        provider: DataProvider,
        figi: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        println!("InstrumentCatalog find_figi {provider} {figi}");

        todo!()
    }

    pub fn list(
        provider: DataProvider,
        category: Category,
    ) -> Result<InstrumentList, ServiceError> {
        println!("InstrumentCatalog list {provider} {category}");

        todo!()
    }
}
