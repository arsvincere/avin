// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use std::path::PathBuf;

use avin_core::Year;
use avin_domain::{DataProvider, InstrumentId, MarketData};
use avin_system::Workspace;

use crate::StorageError;

pub enum MarketDataKey {
    Provider {
        provider: DataProvider,
    },

    Instrument {
        provider: DataProvider,
        iid: InstrumentId,
    },

    MarketData {
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
    },

    Year {
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        year: Year,
    },
}

impl MarketDataKey {
    pub fn provider(provider: DataProvider) -> Self {
        Self::Provider { provider }
    }

    pub fn instrument(provider: DataProvider, iid: InstrumentId) -> Self {
        Self::Instrument { provider, iid }
    }

    pub fn market_data(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
    ) -> Self {
        Self::MarketData { provider, iid, md }
    }

    pub fn year(
        provider: DataProvider,
        iid: InstrumentId,
        md: MarketData,
        year: Year,
    ) -> Self {
        Self::Year {
            provider,
            iid,
            md,
            year,
        }
    }

    pub(super) fn path(&self) -> Result<PathBuf, StorageError> {
        let workspace = Workspace::get().map_err(|err| {
            let msg = "failed to resolve storage path";
            StorageError::path(msg, Some(err.into()))
        })?;

        let mut path = workspace.dirs.market_data().to_path_buf();

        match self {
            Self::Provider { provider } => {
                path.push(provider.key());
            }

            Self::Instrument { provider, iid } => {
                path.push(provider.key());
                path.push(iid.exchange().key());
                path.push(iid.category().key());
                path.push(iid.ticker().to_string());
            }

            Self::MarketData { provider, iid, md } => {
                path.push(provider.key());
                path.push(iid.exchange().key());
                path.push(iid.category().key());
                path.push(iid.ticker().to_string());
                path.push(md.key());
            }

            Self::Year {
                provider,
                iid,
                md,
                year,
            } => {
                path.push(provider.key());
                path.push(iid.exchange().key());
                path.push(iid.category().key());
                path.push(iid.ticker().to_string());
                path.push(md.key());
                path.push(format!("{}.parquet", year));
            }
        };

        Ok(path)
    }
}
