// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::InstrumentList;

use crate::ConnectError;

use super::api::instruments_service_client::InstrumentsServiceClient;

pub struct TBankClient {}

impl TBankClient {
    pub fn shares() -> Result<InstrumentList, ConnectError> {
        todo!()
    }
}
