// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{collections::HashMap, str::FromStr};

use avin_utils::AvinError;

use crate::{Exchange, InstrumentId, InstrumentKind, Symbol};

#[derive(Debug, Clone)]
pub struct InstrumentInfo {
    info: HashMap<String, String>,
}

impl InstrumentInfo {
    pub fn new(info: HashMap<String, String>) -> Result<Self, AvinError> {
        validate_info(&info)?;

        Ok(Self { info })
    }

    pub fn iid(&self) -> InstrumentId {
        let exchange = self.exchange();
        let kind = self.kind();
        let symbol = self.symbol();

        InstrumentId::new(exchange, kind, symbol)
    }

    pub fn exchange(&self) -> Exchange {
        let exchange = self.info.get("exchange").unwrap();

        Exchange::from_str(exchange).unwrap()
    }

    pub fn kind(&self) -> InstrumentKind {
        let kind = self.info.get("instrument_kind").unwrap();

        InstrumentKind::from_str(kind).unwrap()
    }

    pub fn symbol(&self) -> Symbol {
        let symbol = self.info.get("symbol").unwrap();

        Symbol::from_str(symbol).unwrap()
    }

    pub fn figi(&self) -> &str {
        self.info.get("figi").unwrap()
    }

    pub fn name(&self) -> &str {
        self.info.get("name").unwrap()
    }

    pub fn lot(&self) -> u32 {
        self.info.get("lot").unwrap().parse().unwrap()
    }

    pub fn step(&self) -> f64 {
        self.info.get("step").unwrap().parse().unwrap()
    }

    pub fn raw_info(&self) -> &HashMap<String, String> {
        &self.info
    }
}

fn validate_info(info: &HashMap<String, String>) -> Result<(), AvinError> {
    validate_info_keys_complete(info)?;

    let exchange = info.get("exchange").unwrap();
    Exchange::from_str(exchange).map_err(|err| {
        AvinError::InvalidInstrumentInfo {
            message: "failed parsing 'exchange'".to_string(),
            source: Some(Box::new(err)),
        }
    })?;

    let kind = info.get("instrument_kind").unwrap();
    InstrumentKind::from_str(kind).map_err(|err| {
        AvinError::InvalidInstrumentInfo {
            message: "failed parsing 'instrument_kind'".to_string(),
            source: Some(Box::new(err)),
        }
    })?;

    let symbol = info.get("symbol").unwrap();
    Symbol::from_str(symbol).map_err(|err| {
        AvinError::InvalidInstrumentInfo {
            message: "failed parsing 'symbol'".to_string(),
            source: Some(Box::new(err)),
        }
    })?;

    let lot = info.get("lot").unwrap();
    u32::from_str(lot).map_err(|_| {
        AvinError::ParseError(format!("failed parsing 'lot', got '{lot}'"))
    })?;

    let step = info.get("step").unwrap();
    f64::from_str(step).map_err(|_| {
        AvinError::ParseError(format!("failed parsing 'step', got '{step}'"))
    })?;

    Ok(())
}

fn validate_info_keys_complete(
    info: &HashMap<String, String>,
) -> Result<(), AvinError> {
    let expected_keys = [
        "exchange",
        "instrument_kind",
        "symbol",
        "figi",
        "name",
        "lot",
        "step",
    ];

    for key in expected_keys {
        if !info.contains_key(key) {
            return Err(AvinError::InvalidInstrumentInfo {
                message: format!("missing key '{key}'"),
                source: None,
            });
        }

        if info.get(key).unwrap().is_empty() {
            return Err(AvinError::InvalidInstrumentInfo {
                message: format!("empty key '{key}'"),
                source: None,
            });
        }
    }

    Ok(())
}
