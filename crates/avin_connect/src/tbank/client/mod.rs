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
            .map_err(|err| ConnectorError::Connection {
                message: "failed to configure TBank TLS".to_string(),
                source: Some(Box::new(err)),
            })?
            .connect()
            .await
            .map_err(|err| ConnectorError::Connection {
                message: "failed to connect to TBank API".to_string(),
                source: Some(Box::new(err)),
            })?;

        Ok(Self {
            channel,
            interceptor,
        })
    }
}
