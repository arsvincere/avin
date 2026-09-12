// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use avin_connect::TBankClient;
use avin_core::TimeRange;
use avin_domain::{
    Bar, Category, DataProvider, Exchange, InstrumentId, InstrumentInfo,
    MarketData, Tick, TimeFrame,
};
use avin_system::Workspace;

use crate::{DataError, InstrumentPack};

pub struct TBankProvider {}

impl TBankProvider {
    pub fn available_market_data() -> &'static [MarketData] {
        &[MarketData::Bar1M, MarketData::Tick]
    }

    pub async fn fetch_instruments() -> Result<Vec<InstrumentPack>, DataError>
    {
        let ws = Workspace::get().map_err(|err| {
            let msg = "TODO msg".to_string();
            DataError::Connect {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        let token = ws.secret.tbank_token();
        let tbank = TBankClient::connect(token).await.map_err(|err| {
            let msg = "TODO msg".to_string();
            DataError::Connect {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        let shares = tbank.shares().await.map_err(|err| {
            let msg = "TODO msg".to_string();
            DataError::Connect {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        // NOTE: T-Bank get shares from MOEX and SBP, separate it
        let mut moex = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Moex,
            Category::Share,
        );
        let mut spb = InstrumentPack::new(
            DataProvider::TBank,
            Exchange::Spb,
            Category::Share,
        );

        for share in shares.into_iter() {
            match share.exchange() {
                Exchange::Moex => moex.add(share).expect("TODO msg"),
                Exchange::Spb => spb.add(share).expect("TODO msg"),
                other => unreachable!("TODO msg"),
            }
        }

        Ok(vec![moex, spb])
    }

    pub fn fetch_bars(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
    ) -> Result<Vec<Bar>, DataError> {
        if tf != TimeFrame::M1 {
            let msg = format!(
                "{} doesn't provide {tf} bars, available=[{}]",
                DataProvider::TBank,
                TimeFrame::M1,
            );
            return Err(DataError::Unavailable {
                message: msg,
                source: None,
            });
        }

        todo!()
    }

    pub fn fetch_ticks(
        instrument: InstrumentInfo,
        range: TimeRange,
    ) -> Result<Vec<Tick>, DataError> {
        todo!()
    }
}
