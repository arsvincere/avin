// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_data::TBankProvider;
use avin_domain::{
    Category, DataProvider, Exchange, InstrumentId, InstrumentInfo,
};
use avin_storage::{InstrumentInfoKey, InstrumentInfoStorage};

use DataProvider::{MoexAlgo, TBank};

use crate::ServiceError;

use super::catalog::InstrumentCatalog;

pub struct InstrumentService {}

impl InstrumentService {
    pub async fn cache(provider: DataProvider) -> Result<(), ServiceError> {
        match provider {
            TBank => cache_tbank().await,
            MoexAlgo => cache_moexalgo().await,
        }
    }

    pub fn clear(provider: DataProvider) -> Result<(), ServiceError> {
        let key = InstrumentInfoKey::provider(provider);

        InstrumentInfoStorage::delete(&key).map_err(|err| {
            let msg = format!("failed to delete cache {provider}");
            ServiceError::store(msg, Some(err.into()))
        })
    }

    pub fn inventory()
    -> Result<Vec<(DataProvider, Exchange, Category)>, ServiceError> {
        let keys = InstrumentInfoStorage::inventory().map_err(|err| {
            let msg = "failed to inventory instrument cache";
            ServiceError::store(msg, Some(err.into()))
        })?;

        let mut inventory = Vec::with_capacity(keys.len());

        for key in keys {
            let (provider, exchange, category) = match key {
                InstrumentInfoKey::Category {
                    provider,
                    exchange,
                    category,
                } => (provider, exchange, category),

                _ => {
                    unreachable!("inventory must contain only category keys")
                }
            };

            inventory.push((provider, exchange, category));
        }

        Ok(inventory)
    }

    pub fn find_code(
        provider: DataProvider,
        code: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        InstrumentCatalog::find_code(provider, code)
    }

    pub fn find_iid(
        provider: DataProvider,
        iid: &InstrumentId,
    ) -> Result<InstrumentInfo, ServiceError> {
        InstrumentCatalog::find_iid(provider, iid)
    }

    pub fn find_figi(
        provider: DataProvider,
        figi: &str,
    ) -> Result<InstrumentInfo, ServiceError> {
        InstrumentCatalog::find_figi(provider, figi)
    }

    pub fn list(
        provider: DataProvider,
        exchange: Exchange,
        category: Category,
    ) -> Result<Vec<InstrumentInfo>, ServiceError> {
        InstrumentCatalog::list(provider, exchange, category)
    }
}

async fn cache_tbank() -> Result<(), ServiceError> {
    let packs = TBankProvider::fetch_instruments().await.map_err(|err| {
        let msg = format!("failed to fetch instruments from {}", TBank);
        ServiceError::fetch(msg, Some(err.into()))
    })?;

    if packs.is_empty() {
        let msg = "T-Bank returned no instrument packs";
        return Err(ServiceError::fetch(msg, None));
    }

    for pack in packs.into_iter() {
        InstrumentInfoStorage::save(
            pack.provider(),
            pack.exchange(),
            pack.category(),
            pack.instruments(),
        )
        .map_err(|err| {
            let msg = "failed to save instruments cache".to_string();
            ServiceError::store(msg, Some(err.into()))
        })?;
    }

    Ok(())
}

async fn cache_moexalgo() -> Result<(), ServiceError> {
    let msg = format!("{} support is not implemented", MoexAlgo);

    Err(ServiceError::fetch(msg, None))
}
