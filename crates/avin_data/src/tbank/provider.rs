// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use avin_connect::TBankClient;
use avin_core::TimeRange;
use avin_domain::{
    Bar, Category, DataProvider, Exchange, InstrumentId, InstrumentInfo,
    MarketData, Tick, TimeFrame,
};
use avin_system::Workspace;

use crate::{DataError, InstrumentPack};

use Category::Share;
use DataProvider::TBank;
use Exchange::{Moex, Spb};

pub struct TBankProvider {}

impl TBankProvider {
    pub fn available_market_data() -> &'static [MarketData] {
        &[MarketData::Bar1M, MarketData::Tick]
    }

    pub async fn fetch_instruments() -> Result<Vec<InstrumentPack>, DataError>
    {
        let tbank = connect_tbank().await?;

        let shares = tbank.shares().await.map_err(|err| {
            let msg = "failed to fetch T-Bank instrument reference data";
            DataError::connect(msg, Some(err.into()))
        })?;

        // T-Bank returns shares from MOEX and SPB, separate them by exchange.
        let mut moex = Vec::new();
        let mut spb = Vec::new();
        for share in shares {
            match share.exchange() {
                Moex => moex.push(share),
                Spb => spb.push(share),
                other => unreachable!("unsupported T-Bank exchange: {other}"),
            }
        }

        let moex = InstrumentPack::new(TBank, Moex, Share, moex)?;
        let spb = InstrumentPack::new(TBank, Spb, Share, spb)?;

        Ok(vec![moex, spb])
    }

    pub fn fetch_bars(
        instrument: InstrumentInfo,
        tf: TimeFrame,
        range: TimeRange,
    ) -> Result<Vec<Bar>, DataError> {
        if tf != TimeFrame::M1 {
            let msg = format!(
                "T-Bank doesn't provide {tf} bars, available=[{}]",
                TimeFrame::M1,
            );
            return Err(DataError::unavailable(msg, None));
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

async fn connect_tbank() -> Result<TBankClient, DataError> {
    let ws = Workspace::get().map_err(|err| {
        DataError::connect("failed to access workspace", Some(err.into()))
    })?;

    let token = ws.secret.tbank_token();

    TBankClient::connect(token).await.map_err(|err| {
        DataError::connect("failed to connect to T-Bank", Some(err.into()))
    })
}
