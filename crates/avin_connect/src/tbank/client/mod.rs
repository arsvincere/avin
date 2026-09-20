// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod conversion;
mod instrument;
mod interceptor;

// ───────────────────────────────────────────────────────────────────────────

use tonic::transport::{Channel, ClientTlsConfig};

use crate::ConnectorError;

use self::interceptor::TBankInterceptor;

const ENDPOINT: &str = "https://invest-public-api.tbank.ru:443";

pub struct TBankClient {
    channel: Channel,
    interceptor: TBankInterceptor,
}

impl TBankClient {
    pub async fn connect(token: &str) -> Result<Self, ConnectorError> {
        let interceptor = TBankInterceptor::new(token)?;

        let tls = ClientTlsConfig::new().with_native_roots();

        let channel = Channel::from_static(ENDPOINT)
            .tls_config(tls)
            .map_err(|err| {
                let msg = "failed to configure TBank TLS";
                ConnectorError::connection(msg, Some(err.into()))
            })?
            .connect()
            .await
            .map_err(|err| {
                let msg = "failed to connect to TBank API";
                ConnectorError::connection(msg, Some(err.into()))
            })?;

        Ok(Self {
            channel,
            interceptor,
        })
    }
}
