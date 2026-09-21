// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use polars::prelude::DataFrame;

use avin_core::{TimeRange, Year};
use avin_data::TBankProvider;
use avin_domain::{DataProvider, InstrumentId, MarketData, TimeFrame};
use avin_storage::{MarketDataKey, StorageStatus};

use DataProvider::{MoexAlgo, TBank};

use crate::{InstrumentService, ServiceError};

pub struct DataService {}

impl DataService {
    pub fn download(
        provider: DataProvider,
        iid: &InstrumentId,
        md: MarketData,
        year: Year,
    ) -> Result<(), ServiceError> {
        match provider {
            TBank => download_tbank(iid, md, year),
            MoexAlgo => download_moexalgo(iid, md, year),
        }
    }

    pub fn load(
        provider: DataProvider,
        iid: &InstrumentId,
        md: MarketData,
        range: TimeRange,
    ) -> Result<DataFrame, ServiceError> {
        todo!();
    }

    pub fn compact() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn prune() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn delete(key: MarketDataKey) -> Result<(), ServiceError> {
        todo!();
    }

    pub fn sync(
        provider: DataProvider,
        iid: Option<InstrumentId>,
        md: Option<MarketData>,
        year: Option<Year>,
        force: bool,
    ) -> Result<(), ServiceError> {
        todo!();
    }

    pub fn resume() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn abort() -> Result<(), ServiceError> {
        todo!();
    }

    pub fn status() -> Result<StorageStatus, ServiceError> {
        todo!();
    }
}

fn download_tbank(
    iid: &InstrumentId,
    md: MarketData,
    year: Year,
) -> Result<(), ServiceError> {
    let i = InstrumentService::find_iid(TBank, iid)?;
    let range = year.time_range();

    // match md {
    //     MarketData::Tick => TBankProvider::fetch_ticks(i, range),
    //     MarketData::Bar1M => {
    //         TBankProvider::fetch_bars(i, TimeFrame::M1, range)
    //     }
    //     _ => todo!(),
    // };

    todo!();
}

fn download_moexalgo(
    _iid: &InstrumentId,
    _md: MarketData,
    _year: Year,
) -> Result<(), ServiceError> {
    let msg = format!("{} support is not implemented", MoexAlgo);

    Err(ServiceError::fetch(msg, None))
}
