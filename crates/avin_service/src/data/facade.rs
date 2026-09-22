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
use avin_domain::{
    Bar, DataProvider, InstrumentId, InstrumentInfo, MarketData, TimeFrame,
};
use avin_storage::{
    DataChunk, MarketDataKey, MarketDataStorage, StorageStatus,
};

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
        let i = InstrumentService::find_iid(TBank, iid)?;
        let range = year.time_range();

        match md {
            MarketData::Tick => download_ticks(provider, i, year)?,
            MarketData::Bar1M => {
                download_bars(provider, i, TimeFrame::M1, year)?
            }
            _ => todo!(),
        };

        todo!()
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

fn download_bars(
    provider: DataProvider,
    instrument: InstrumentInfo,
    tf: TimeFrame,
    year: Year,
) -> Result<(), ServiceError> {
    let range = year.time_range();

    let result = match provider {
        TBank => TBankProvider::fetch_bars(instrument.clone(), tf, range),
        MoexAlgo => todo!(),
    };

    let packs = result.map_err(|err| {
        let msg = "";
        ServiceError::fetch(msg, Some(err.into()))
    })?;

    let iid = instrument.iid();
    let md = MarketData::Bar1M; // TODO: create it from tf
    let key = MarketDataKey::year(provider, iid, md, year);

    let operation = MarketDataStorage::write(key).map_err(|err| {
        let msg = "";
        ServiceError::store(msg, Some(err.into()))
    })?;

    for pack in packs {
        let pack = pack.map_err(|err| {
            let msg = "";
            ServiceError::fetch(msg, Some(err.into()))
        })?;

        // operation.add_bars(pack.range(), pack.bars()); ??? но при этом операция следит что добавление корректно
        //
        // или
        //
        // let df = Bar::to_df(pack.bars())?;
        // let chunk = DataChunk::new(range, df);
        // operation.add(chunk);            ???
    }

    operation.finalize().map_err(|err| {
        let msg = "";
        ServiceError::store(msg, Some(err.into()))
    })?;

    Ok(())
}

fn download_ticks(
    provider: DataProvider,
    instrument: InstrumentInfo,
    year: Year,
) -> Result<(), ServiceError> {
    todo!()
}
