// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_data::TBankProvider;
use avin_domain::{
    Category, DataProvider, Exchange, InstrumentInfo, InstrumentList,
};
use avin_storage::InstrumentInfoStorage;

use DataProvider::{MoexAlgo, TBank};

use crate::ServiceError;

use super::catalog::InstrumentCatalog;

pub struct InstrumentService {}

impl InstrumentService {
    pub async fn cache(provider: DataProvider) -> Result<(), ServiceError> {
        match provider {
            DataProvider::TBank => cache_tbank().await,
            DataProvider::MoexAlgo => cache_moexalgo().await,
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
        exchange: Exchange,
        category: Category,
    ) -> Result<InstrumentList, ServiceError> {
        InstrumentCatalog::list(provider, exchange, category)
    }
}

async fn cache_tbank() -> Result<(), ServiceError> {
    let packs = TBankProvider::fetch_instruments().await.map_err(|err| {
        let msg = format!("failed to fetch instruments from {}", TBank);
        ServiceError::Fetch {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;

    if packs.is_empty() {
        return Err(ServiceError::Fetch {
            message: "T-Bank returned no instrument packs".into(),
            source: None,
        });
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
            ServiceError::Store {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;
    }

    Ok(())
}

async fn cache_moexalgo() -> Result<(), ServiceError> {
    let msg = format!("{} support is not implemented", MoexAlgo);

    Err(ServiceError::Fetch {
        message: msg,
        source: None,
    })
}
