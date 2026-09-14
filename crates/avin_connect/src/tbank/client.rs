// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_domain::InstrumentList;

use crate::ConnectError;

pub struct TBankClient {}

impl TBankClient {
    pub fn shares() -> Result<InstrumentList, ConnectError> {
        todo!()
    }
}
