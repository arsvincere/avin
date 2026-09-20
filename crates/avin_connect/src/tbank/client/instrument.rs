// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::InstrumentInfo;

use crate::ConnectorError;
use crate::tbank::api::{
    self, instruments_service_client::InstrumentsServiceClient,
};

use super::TBankClient;

impl TBankClient {
    pub async fn shares(
        &self,
    ) -> Result<Vec<InstrumentInfo>, ConnectorError> {
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
                let msg = "failed to get T-Bank shares";
                ConnectorError::request(msg, Some(err.into()))
            })?
            .into_inner();

        let mut instruments = Vec::new();
        for share in response.instruments {
            if !supported(&share) {
                continue;
            }
            let info = InstrumentInfo::try_from(share)?;
            instruments.push(info);
        }

        Ok(instruments)
    }
}

// NOTE: отбрасываем всякую непонятную поебень:
// - внебиржевые инструменты
// - не торгующиеся больше инструменты у которых нет min price step
fn supported(share: &api::Share) -> bool {
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
