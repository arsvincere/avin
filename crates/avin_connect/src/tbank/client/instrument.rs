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

        let mut shares_info = Vec::new();
        for share in response.instruments {
            if !supported_share(&share) {
                continue;
            }
            let info = InstrumentInfo::try_from(share)?;
            shares_info.push(info);
        }

        Ok(shares_info)
    }

    pub async fn futures(
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
            .futures(request)
            .await
            .map_err(|err| {
                let msg = "failed to get T-Bank futures";
                ConnectorError::request(msg, Some(err.into()))
            })?
            .into_inner();

        let mut futures_info = Vec::new();
        for future in response.instruments {
            if !supported_future(&future) {
                continue;
            }
            let info = InstrumentInfo::try_from(future)?;
            futures_info.push(info);
        }

        Ok(futures_info)
    }

    pub async fn bonds(&self) -> Result<Vec<InstrumentInfo>, ConnectorError> {
        let request = api::InstrumentsRequest {
            instrument_status: Some(api::InstrumentStatus::Base as i32),
            instrument_exchange: None,
        };

        let mut client = InstrumentsServiceClient::with_interceptor(
            self.channel.clone(),
            self.interceptor.clone(),
        );

        let response = client
            .bonds(request)
            .await
            .map_err(|err| {
                let msg = "failed to get T-Bank bonds";
                ConnectorError::request(msg, Some(err.into()))
            })?
            .into_inner();

        let mut bonds_info = Vec::new();
        for bond in response.instruments {
            if !supported_bond(&bond) {
                continue;
            }
            let info = InstrumentInfo::try_from(bond)?;
            bonds_info.push(info);
        }

        Ok(bonds_info)
    }
}

// отбрасываем всякую непонятную поебень:
// - внебиржевые инструменты
// - не торгующиеся больше инструменты у которых нет min price step
fn supported_share(share: &api::Share) -> bool {
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

// Пока поддерживаем только биржевые MOEX futures.
// Неоактивы T-Bank (RealExchange::Unspecified) пропускаем.
fn supported_future(future: &api::Future) -> bool {
    if future.min_price_increment.is_none() {
        return false;
    }

    if future.real_exchange() == api::RealExchange::Moex {
        return true;
    }

    false
}

// Пока поддерживаем только биржевые MOEX bonds.
fn supported_bond(bond: &api::Bond) -> bool {
    if bond.min_price_increment.is_none() {
        return false;
    }

    if bond.real_exchange() == api::RealExchange::Moex {
        return true;
    }

    false
}
