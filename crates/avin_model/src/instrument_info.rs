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
        AvinError::ParseError(format!(
            "failed parsing 'lot' as u32, got '{lot}'"
        ))
    })?;

    let step = info.get("step").unwrap();
    f64::from_str(step).map_err(|_| {
        AvinError::ParseError(format!(
            "failed parsing 'step' as f64, got '{step}'"
        ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_info() {
        let mut raw = HashMap::new();
        raw.insert("exchange".to_string(), "MOEX".to_string());
        raw.insert("instrument_kind".to_string(), "Stock".to_string());
        raw.insert("symbol".to_string(), "SBER".to_string());
        raw.insert("figi".to_string(), "BBG004730N88".to_string());
        raw.insert("name".to_string(), "Сбер Банк".to_string());
        raw.insert("lot".to_string(), "1".to_string());
        raw.insert("step".to_string(), "0.01".to_string());
        raw.insert(
            "uid".to_string(),
            "e6123145-9665-43e0-8413-cd61b8aa9b13".to_string(),
        );

        let info = InstrumentInfo::new(raw).unwrap();

        assert_eq!(info.exchange(), Exchange::MOEX);
        assert_eq!(info.kind(), InstrumentKind::Stock);
        assert_eq!(info.symbol(), Symbol::new("SBER").unwrap());
        assert_eq!(info.figi(), "BBG004730N88");
        assert_eq!(info.name(), "Сбер Банк");
        assert_eq!(info.lot(), 1);
        assert_eq!(info.step(), 0.01);

        let uid = info.raw_info().get("uid").unwrap();
        assert_eq!(uid, "e6123145-9665-43e0-8413-cd61b8aa9b13");
    }
}
