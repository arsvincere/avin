// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after impl
#![allow(unused)]

use std::str::FromStr;

use cached::cached;

use avin_domain::{
    Category, DataProvider, Exchange, InstrumentId, InstrumentInfo,
};
use avin_storage::{DataFrameExt, InstrumentInfoStorage};

use crate::ServiceError;

pub(super) struct InstrumentCatalog {}

impl InstrumentCatalog {
    pub fn find_code(
        provider: DataProvider,
        code: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        let iid = InstrumentId::from_str(code).map_err(|err| {
            let msg = format!("failed parsing instrument code '{code}'");
            ServiceError::find(msg, Some(err.into()))
        })?;

        InstrumentCatalog::find_iid(provider, &iid)
    }

    pub fn find_iid(
        provider: DataProvider,
        iid: &InstrumentId,
    ) -> Result<InstrumentInfo, ServiceError> {
        // TODO: говнокод... надо решить что делать с case sensitive Ticker Eq
        let exchange = iid.exchange();
        let category = iid.category();
        let ticker = iid.ticker().to_string();

        let instruments = cached_load(provider, exchange, category)?;

        for i in instruments.iter() {
            if i.ticker().to_string().eq_ignore_ascii_case(&ticker) {
                return Ok(i.clone());
            }
        }

        let msg = format!("{iid} not found in {provider} instruments");

        Err(ServiceError::find(msg, None))
    }

    pub fn find_figi(
        provider: DataProvider,
        figi: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        println!("InstrumentCatalog find_figi {provider} {figi}");

        todo!()
    }

    // TODO: блять тут копирование вектора идет. надо возвращать слайс
    pub fn list(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Result<Vec<InstrumentInfo>, ServiceError> {
        cached_load(provider, exchange, category)
    }
}

#[cached]
fn cached_load(
    provider: DataProvider,
    exchange: Exchange,
    category: Category,
) -> Result<Vec<InstrumentInfo>, ServiceError> {
    let df = InstrumentInfoStorage::load(provider, exchange, category)
        .map_err(|err| {
            let msg = "failed to load instrument reference data";
            ServiceError::store(msg, Some(err.into()))
        })?;

    let instruments = InstrumentInfo::from_df(df).map_err(|err| {
        let msg = "failed to convert instrument reference data";
        ServiceError::store(msg, Some(err.into()))
    })?;

    Ok(instruments)
}
