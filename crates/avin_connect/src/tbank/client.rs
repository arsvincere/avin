// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: delete after impl
#![allow(unused)]

use avin_domain::InstrumentList;

use crate::ConnectError;

use super::api;
use super::api::instruments_service_client::InstrumentsServiceClient;
use super::interceptor::TBankInterceptor;

pub struct TBankClient {
    channel: tonic::transport::Channel,
    interceptor: TBankInterceptor,
}

impl TBankClient {
    pub async fn shares(&self) -> Result<InstrumentList, ConnectError> {
        let request = api::InstrumentsRequest {
            instrument_status: Some(api::InstrumentStatus::Base as i32),
            instrument_exchange: None,
        };

        todo!()

        // let response = self
        // .instruments
        // .shares(request)
        // .await
        // .map_err(/* tonic::Status -> ConnectError */)?
        // .into_inner();
        //
        // let mut instruments = InstrumentList::new();
        //
        // for share in response.instruments {
        //     let info = convert_share(share)?;
        //     instruments
        //     .add(info)
        //     .map_err(/* DomainError -> ConnectError */)?;
        // }
        //
        // Ok(instruments)
    }
}
