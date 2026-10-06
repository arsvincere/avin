// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::PathBuf;

use avin_domain::{Category, DataProvider, Exchange};
use avin_system::Workspace;

use crate::StorageError;

pub enum InstrumentInfoKey {
    Provider {
        provider: DataProvider,
    },

    Exchange {
        provider: DataProvider,
        exchange: Exchange,
    },

    Category {
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    },
}

impl InstrumentInfoKey {
    pub fn provider(provider: DataProvider) -> Self {
        Self::Provider { provider }
    }

    pub fn exchange(provider: DataProvider, exchange: Exchange) -> Self {
        Self::Exchange { provider, exchange }
    }

    pub fn category(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Self {
        Self::Category {
            provider,
            exchange,
            category,
        }
    }

    pub(super) fn path(&self) -> Result<PathBuf, StorageError> {
        let workspace = Workspace::get().map_err(|err| {
            let msg = "failed to resolve storage path";
            StorageError::path(msg, Some(err.into()))
        })?;

        let mut path = workspace.dirs.instruments().to_path_buf();

        match self {
            Self::Provider { provider } => {
                path.push(provider.key());
            }

            Self::Exchange { provider, exchange } => {
                path.push(provider.key());
                path.push(exchange.key());
            }

            Self::Category {
                provider,
                exchange,
                category,
            } => {
                path.push(provider.key());
                path.push(exchange.key());
                path.push(format!("{}.parquet", category.key()));
            }
        };

        Ok(path)
    }
}
