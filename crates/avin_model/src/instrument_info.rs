// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{collections::HashMap, str::FromStr};

use avin_utils::AvinError;

use crate::{Exchange, InstrumentId, InstrumentKind, Symbol};

const REQUIRED_KEYS: [&str; 7] = [
    "exchange",
    "instrument_kind",
    "symbol",
    "figi",
    "name",
    "lot",
    "step",
];

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
        AvinError::InstrumentInfo {
            message: "failed parsing 'exchange'".to_string(),
            source: Some(Box::new(err)),
        }
    })?;

    let kind = info.get("instrument_kind").unwrap();
    InstrumentKind::from_str(kind).map_err(|err| {
        AvinError::InstrumentInfo {
            message: "failed parsing 'instrument_kind'".to_string(),
            source: Some(Box::new(err)),
        }
    })?;

    let symbol = info.get("symbol").unwrap();
    Symbol::from_str(symbol).map_err(|err| AvinError::InstrumentInfo {
        message: "failed parsing 'symbol'".to_string(),
        source: Some(Box::new(err)),
    })?;

    let lot = info.get("lot").unwrap();
    let lot = u32::from_str(lot).map_err(|err| {
        AvinError::Parse(format!(
            "failed parsing 'lot' as u32, got '{lot}': {err}"
        ))
    })?;
    if lot == 0 {
        return Err(AvinError::Value(
            "'lot' must be greater than zero".to_string(),
        ));
    }

    let step = info.get("step").unwrap();
    let step = f64::from_str(step).map_err(|err| {
        AvinError::Parse(format!(
            "failed parsing 'step' as f64, got '{step}': {err}"
        ))
    })?;
    if !step.is_finite() || step <= 0.0 {
        return Err(AvinError::Value(
            "'step' must be finite and greater than zero".to_string(),
        ));
    }

    Ok(())
}

fn validate_info_keys_complete(
    info: &HashMap<String, String>,
) -> Result<(), AvinError> {
    for key in REQUIRED_KEYS {
        if !info.contains_key(key) {
            return Err(AvinError::Key(format!("missing key '{key}'")));
        }

        if info.get(key).unwrap().is_empty() {
            return Err(AvinError::Missing(format!(
                "missing value for '{key}'"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_raw_info() -> HashMap<String, String> {
        [
            ("exchange", "MOEX"),
            ("instrument_kind", "Stock"),
            ("symbol", "SBER"),
            ("figi", "BBG004730N88"),
            ("name", "Сбер Банк"),
            ("lot", "1"),
            ("step", "0.01"),
            ("uid", "e6123145-9665-43e0-8413-cd61b8aa9b13"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    #[test]
    fn required_keys() {
        assert_eq!(
            REQUIRED_KEYS,
            [
                "exchange",
                "instrument_kind",
                "symbol",
                "figi",
                "name",
                "lot",
                "step",
            ]
        );
    }

    #[test]
    fn valid_info() {
        let raw_info = valid_raw_info();
        let info = InstrumentInfo::new(raw_info).unwrap();

        assert_eq!(info.exchange(), Exchange::MOEX);
        assert_eq!(info.kind(), InstrumentKind::Stock);
        assert_eq!(info.symbol(), Symbol::new("SBER").unwrap());
        assert_eq!(info.figi(), "BBG004730N88");
        assert_eq!(info.name(), "Сбер Банк");
        assert_eq!(info.lot(), 1);
        assert_eq!(info.step(), 0.01);

        let uid = info.raw_info().get("uid").unwrap();
        assert_eq!(uid, "e6123145-9665-43e0-8413-cd61b8aa9b13");

        assert_eq!(
            info.iid(),
            InstrumentId::new(
                Exchange::MOEX,
                InstrumentKind::Stock,
                Symbol::new("SBER").unwrap(),
            )
        );
    }

    #[test]
    fn missing_required_key() {
        for key in REQUIRED_KEYS {
            let mut raw_info = valid_raw_info();
            raw_info.remove(key);

            let err = InstrumentInfo::new(raw_info).unwrap_err();

            assert!(matches!(err, AvinError::Key(_)));
        }
    }

    #[test]
    fn missing_required_value() {
        for key in REQUIRED_KEYS {
            let mut raw_info = valid_raw_info();
            raw_info.insert(key.to_string(), String::new());

            let err = InstrumentInfo::new(raw_info).unwrap_err();

            assert!(matches!(err, AvinError::Missing(_)));
        }
    }

    #[test]
    fn invalid_exchange() {
        let mut raw_info = valid_raw_info();
        raw_info.insert("exchange".to_string(), "*/=-:;".to_string());

        let err = InstrumentInfo::new(raw_info).unwrap_err();

        assert!(matches!(err, AvinError::InstrumentInfo { .. }));
    }

    #[test]
    fn invalid_lot() {
        let mut raw_info = valid_raw_info();
        raw_info.insert("lot".to_string(), "abc".to_string());

        let err = InstrumentInfo::new(raw_info).unwrap_err();

        assert!(matches!(err, AvinError::Parse(_)));
    }

    #[test]
    fn zero_lot() {
        let mut raw_info = valid_raw_info();
        raw_info.insert("lot".to_string(), "0".to_string());

        let err = InstrumentInfo::new(raw_info).unwrap_err();

        assert!(matches!(err, AvinError::Value(_)));
    }

    #[test]
    fn invalid_step() {
        let mut raw_info = valid_raw_info();
        raw_info.insert("step".to_string(), "abc".to_string());

        let err = InstrumentInfo::new(raw_info).unwrap_err();

        assert!(matches!(err, AvinError::Parse(_)));
    }

    #[test]
    fn invalid_step_value() {
        for step in ["0", "-0.05", "NaN", "inf", "-inf"] {
            let mut raw_info = valid_raw_info();
            raw_info.insert("step".to_string(), step.to_string());

            let err = InstrumentInfo::new(raw_info).unwrap_err();

            assert!(matches!(err, AvinError::Value(_)));
        }
    }
}
