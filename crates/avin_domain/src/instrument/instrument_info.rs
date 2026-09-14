// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{collections::HashMap, str::FromStr};

use avin_core::{Price, Quantity};

use crate::DomainError;

use crate::{Category, Exchange, InstrumentId, Ticker};

/// Instrument reference data.
///
/// Represents a locally stored instrument description used for instrument
/// lookup, asset creation, and offline market research.
///
/// `InstrumentInfo` is not intended to be instantiated directly. Instances are
/// created by AVIN as part of concrete instrument objects such as futures,
/// shares, bonds, and options.
///
/// The underlying metadata is intentionally stored as raw string values to
/// provide a stable, provider-independent representation. Typed accessors
/// parse individual values on demand.
///
/// Reference data may be slightly outdated and must not be treated as
/// authoritative for live trading validation.
#[derive(Debug, Clone)]
pub struct InstrumentInfo {
    info: HashMap<String, String>,
}

impl InstrumentInfo {
    // TODO: вынести в "приватный трейт"
    /// Creates an `InstrumentInfo` from trusted raw key-value fields.
    ///
    /// No validation is performed. The caller must ensure that all canonical
    /// fields required by the accessors are present and contain valid values.
    pub fn new_unchecked(info: HashMap<String, String>) -> Self {
        Self { info }
    }

    /// Creates share instrument information from canonical and extra fields.
    ///
    /// Canonical share fields are built from the typed arguments.
    /// `extra_info` may contain arbitrary provider-specific metadata, but
    /// must not contain keys reserved by the canonical fields.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InstrumentInfo`] if:
    /// - `price_step` is not positive;
    /// - `lot_size` is zero;
    /// - `extra_info` contains a reserved canonical key.
    pub fn new_share(
        exchange: Exchange,
        ticker: Ticker,
        name: String,
        price_step: Price,
        lot_size: Quantity,
        extra_info: HashMap<String, String>,
    ) -> Result<Self, DomainError> {
        // check price step
        if price_step.value() <= 0.0 {
            let msg =
                format!("price step must be positive, got {price_step}");
            return Err(DomainError::InstrumentInfo {
                message: msg,
                source: None,
            });
        }

        // check lot size
        if lot_size.value() == 0.0 {
            let msg =
                format!("lot size must be greater than zero, got {lot_size}");
            return Err(DomainError::InstrumentInfo {
                message: msg,
                source: None,
            });
        }

        // fill required fields
        let category = Category::Share;
        let mut info = HashMap::new();

        info.insert("exchange".to_string(), exchange.key().to_string());
        info.insert("category".to_string(), category.key().to_string());
        info.insert("ticker".to_string(), ticker.to_string());
        info.insert("name".to_string(), name);
        info.insert("price_step".to_string(), price_step.to_string());
        info.insert("lot_size".to_string(), lot_size.to_string());

        // check collisions with canonical fields
        for key in extra_info.keys() {
            if info.contains_key(key) {
                let msg = format!("extra info contains reserved key '{key}'");
                return Err(DomainError::InstrumentInfo {
                    message: msg,
                    source: None,
                });
            }
        }

        // add extra info
        info.extend(extra_info);

        Ok(Self { info })
    }

    /// Returns the canonical instrument ID.
    pub fn iid(&self) -> InstrumentId {
        let exchange = self.exchange();
        let category = self.category();
        let ticker = self.ticker();

        InstrumentId::new(exchange, category, ticker)
    }

    /// Returns the instrument exchange.
    pub fn exchange(&self) -> Exchange {
        let exchange = self.info.get("exchange").unwrap();

        Exchange::from_str(exchange).unwrap()
    }

    /// Returns the category.
    pub fn category(&self) -> Category {
        let category = self.info.get("category").unwrap();

        Category::from_str(category).unwrap()
    }

    /// Returns the instrument ticker.
    pub fn ticker(&self) -> Ticker {
        let ticker = self.info.get("ticker").unwrap();

        Ticker::new(ticker).unwrap()
    }

    /// Returns the instrument name.
    pub fn name(&self) -> &str {
        self.info.get("name").unwrap()
    }

    /// Returns the minimum price step.
    pub fn price_step(&self) -> Price {
        let step: f64 = self.info.get("price_step").unwrap().parse().unwrap();

        Price::new(step).unwrap()
    }

    /// Returns the lot size.
    pub fn lot_size(&self) -> Quantity {
        let size: f64 = self.info.get("lot_size").unwrap().parse().unwrap();

        Quantity::new(size).unwrap()
    }

    /// Returns the original instrument metadata.
    ///
    /// This includes both required AVIN fields and any additional
    /// provider-specific fields.
    pub fn raw_info(&self) -> &HashMap<String, String> {
        &self.info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_raw_info() -> HashMap<String, String> {
        [
            ("exchange", "moex"),
            ("category", "share"),
            ("ticker", "SBER"),
            ("name", "Сбер Банк"),
            ("price_step", "0.01"),
            ("lot_size", "10"),
            ("uid", "e6123145-9665-43e0-8413-cd61b8aa9b13"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    fn new_share(
        price_step: f64,
        lot_size: f64,
        extra_info: HashMap<String, String>,
    ) -> Result<InstrumentInfo, DomainError> {
        InstrumentInfo::new_share(
            Exchange::Moex,
            Ticker::new("SBER").unwrap(),
            "Сбер Банк".to_string(),
            Price::new(price_step).unwrap(),
            Quantity::new(lot_size).unwrap(),
            extra_info,
        )
    }

    #[test]
    fn new_unchecked() {
        let info = InstrumentInfo::new_unchecked(valid_raw_info());

        assert_eq!(info.exchange(), Exchange::Moex);
        assert_eq!(info.category(), Category::Share);
        assert_eq!(info.ticker(), Ticker::new("SBER").unwrap());
        assert_eq!(info.name(), "Сбер Банк");
        assert_eq!(info.price_step(), Price::new(0.01).unwrap());
        assert_eq!(info.lot_size(), Quantity::new(10.0).unwrap());

        assert_eq!(
            info.iid(),
            InstrumentId::new(
                Exchange::Moex,
                Category::Share,
                Ticker::new("SBER").unwrap(),
            )
        );

        assert_eq!(
            info.raw_info().get("uid").unwrap(),
            "e6123145-9665-43e0-8413-cd61b8aa9b13"
        );
    }

    #[test]
    fn new_share_valid() {
        let extra_info = HashMap::from([
            (
                "uid".to_string(),
                "e6123145-9665-43e0-8413-cd61b8aa9b13".to_string(),
            ),
            ("figi".to_string(), "BBG004730N88".to_string()),
        ]);

        let info = new_share(0.01, 10.0, extra_info).unwrap();

        assert_eq!(info.exchange(), Exchange::Moex);
        assert_eq!(info.category(), Category::Share);
        assert_eq!(info.ticker(), Ticker::new("SBER").unwrap());
        assert_eq!(info.name(), "Сбер Банк");
        assert_eq!(info.price_step(), Price::new(0.01).unwrap());
        assert_eq!(info.lot_size(), Quantity::new(10.0).unwrap());

        assert_eq!(info.raw_info().get("exchange").unwrap(), "moex");
        assert_eq!(info.raw_info().get("category").unwrap(), "share");
        assert_eq!(info.raw_info().get("ticker").unwrap(), "SBER");
        assert_eq!(info.raw_info().get("name").unwrap(), "Сбер Банк");
        assert_eq!(info.raw_info().get("price_step").unwrap(), "0.01");
        assert_eq!(info.raw_info().get("lot_size").unwrap(), "10");

        assert_eq!(
            info.raw_info().get("uid").unwrap(),
            "e6123145-9665-43e0-8413-cd61b8aa9b13"
        );
        assert_eq!(info.raw_info().get("figi").unwrap(), "BBG004730N88");
    }

    #[test]
    fn new_share_price_step_positive() {
        for price_step in [0.0, -0.01] {
            let err =
                new_share(price_step, 10.0, HashMap::new()).unwrap_err();

            assert!(matches!(err, DomainError::InstrumentInfo { .. }));
        }
    }

    #[test]
    fn new_share_lot_size_positive() {
        let err = new_share(0.01, 0.0, HashMap::new()).unwrap_err();

        assert!(matches!(err, DomainError::InstrumentInfo { .. }));
    }

    #[test]
    fn new_share_reserved_extra_info() {
        for key in [
            "exchange",
            "category",
            "ticker",
            "name",
            "price_step",
            "lot_size",
        ] {
            let extra_info =
                HashMap::from([(key.to_string(), "override".to_string())]);

            let err = new_share(0.01, 10.0, extra_info).unwrap_err();

            assert!(matches!(err, DomainError::InstrumentInfo { .. }));
        }
    }
}
