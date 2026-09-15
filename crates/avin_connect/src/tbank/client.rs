// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::{InstrumentInfo, InstrumentList};

use crate::ConnectError;

use super::api;
use super::api::instruments_service_client::InstrumentsServiceClient;
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

        let channel = tonic::transport::Channel::from_static(ENDPOINT)
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
            let info = convert_share(share)?;
            instruments.add(info).map_err(|err| {
                let msg = "TODO msg".to_string();
                ConnectError::TBank {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?;
        }

        Ok(instruments)
    }
}

fn convert_share(share: api::Share) -> Result<InstrumentInfo, ConnectError> {
    dbg!(&share);

    todo!();
}
