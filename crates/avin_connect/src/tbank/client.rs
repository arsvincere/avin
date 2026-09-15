// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use std::collections::HashMap;

use avin_core::{Price, Quantity};
use avin_domain::{Exchange, InstrumentInfo, InstrumentList, Ticker};

use crate::ConnectError;

use super::api;
use super::api::instruments_service_client::InstrumentsServiceClient;
use super::conversion;
use super::interceptor::TBankInterceptor;

const ENDPOINT: &str = "https://invest-public-api.tbank.ru:443";

pub struct TBankClient {
    channel: tonic::transport::Channel,
    interceptor: TBankInterceptor,
}

impl TBankClient {
    pub async fn connect(token: &str) -> Result<Self, ConnectError> {
        let interceptor = TBankInterceptor::new(token).map_err(|err| {
            ConnectError::Authorization {
                message: "invalid TBank authorization token".to_string(),
                source: Some(Box::new(err)),
            }
        })?;

        let tls =
            tonic::transport::ClientTlsConfig::new().with_native_roots();

        let channel = tonic::transport::Channel::from_static(ENDPOINT)
            .tls_config(tls)
            .map_err(|err| ConnectError::TBank {
                message: "failed to configure TBank TLS".to_string(),
                source: Some(Box::new(err)),
            })?
            .connect()
            .await
            .map_err(|err| ConnectError::TBank {
                message: "failed to connect to TBank API".to_string(),
                source: Some(Box::new(err)),
            })?;

        Ok(Self {
            channel,
            interceptor,
        })
    }

    pub async fn shares(&self) -> Result<InstrumentList, ConnectError> {
        let request = api::InstrumentsRequest {
            instrument_status: Some(api::InstrumentStatus::Base as i32),
            instrument_exchange: None,
        };

        let mut client = InstrumentsServiceClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        );

        let response = client
            .shares(request)
            .await
            .map_err(|err| {
                let msg = "TODO msg".to_string();
                ConnectError::TBank {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?
            .into_inner();

        let mut instruments = InstrumentList::new();

        for share in response.instruments {
            if !supported(&share) {
                continue;
            }

            let info = InstrumentInfo::try_from(share)?;

            instruments.add(info).map_err(|err| {
                let msg = "TBank failed add instrument".to_string();
                ConnectError::TBank {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?;
        }

        Ok(instruments)
    }
}

fn supported(share: &api::Share) -> bool {
    // бывает для инструментов которые уже не торгуются
    if share.min_price_increment.is_none() {
        return false;
    }

    match share.real_exchange() {
        api::RealExchange::Unspecified => false,
        api::RealExchange::Moex => true,
        api::RealExchange::Rts => true,
        api::RealExchange::Otc => false,
        api::RealExchange::Dealer => false,
    }
}
