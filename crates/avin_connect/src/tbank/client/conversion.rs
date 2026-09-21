// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use avin_core::{Price, Quantity, Time};
use avin_domain::{DataProvider, Exchange, InstrumentInfo, Ticker};

use crate::ConnectorError;
use crate::tbank::api;

// from TBank to AVIN
impl From<api::Quotation> for f64 {
    fn from(value: api::Quotation) -> Self {
        value.units as f64 + value.nano as f64 / 1_000_000_000.0
    }
}
impl From<api::Quotation> for Price {
    fn from(value: api::Quotation) -> Self {
        let p = value.units as f64 + value.nano as f64 / 1_000_000_000.0;

        Price::new(p).unwrap()
    }
}
impl From<api::MoneyValue> for Price {
    fn from(value: api::MoneyValue) -> Self {
        let p = value.units as f64 + value.nano as f64 / 1_000_000_000.0;

        Price::new(p).unwrap()
    }
}

impl TryFrom<api::RealExchange> for Exchange {
    type Error = ConnectorError;

    fn try_from(value: api::RealExchange) -> Result<Self, Self::Error> {
        match value {
            api::RealExchange::Moex => Ok(Exchange::Moex),
            api::RealExchange::Rts => Ok(Exchange::Spb),
            _ => {
                let msg = format!(
                    "exchange '{}' returned by T-Bank is not supported by AVIN",
                    value.as_str_name()
                );
                Err(ConnectorError::conversion(msg, None))
            }
        }
    }
}
impl TryFrom<api::Share> for InstrumentInfo {
    type Error = ConnectorError;

    fn try_from(share: api::Share) -> Result<Self, Self::Error> {
        let provider = DataProvider::TBank;

        let exchange = Exchange::try_from(share.real_exchange())?;

        let ticker = Ticker::new(share.ticker.clone())
            .expect("T-Bank returned invalid ticker");

        let name = share.name;

        let price_step = match share.min_price_increment {
            Some(value) => Price::from(value),
            None => {
                let msg = format!(
                    "{provider} share {} has no minimum price increment",
                    share.ticker
                );
                return Err(ConnectorError::conversion(msg, None));
            }
        };

        let lot_size = Quantity::new(share.lot as f64)
            .expect("T-Bank lot size must be non-negative");

        let mut extra_info = HashMap::new();
        extra_info.insert("country".to_string(), share.country_of_risk);
        extra_info.insert("currency".to_string(), share.currency);
        extra_info.insert("sector".to_string(), share.sector);
        extra_info.insert("exchange_section".to_string(), share.exchange);
        extra_info.insert("class_code".to_string(), share.class_code);
        extra_info.insert("figi".to_string(), share.figi);
        extra_info.insert("isin".to_string(), share.isin);
        extra_info.insert("uid".to_string(), share.uid);
        extra_info.insert(
            "short_enabled".to_string(),
            share.short_enabled_flag.to_string(),
        );
        extra_info.insert(
            "div_yield_flag".to_string(),
            share.div_yield_flag.to_string(),
        );

        if let Some(value) = share.dlong_client {
            extra_info.insert(
                "risk_long".to_string(),
                f64::from(value).to_string(),
            );
        }

        if let Some(value) = share.dshort_client {
            extra_info.insert(
                "risk_short".to_string(),
                f64::from(value).to_string(),
            );
        }

        if let Some(ts) = share.ipo_date {
            let time = time(ts);
            extra_info.insert("ipo_date".to_string(), time.to_string());
        };

        if let Some(ts) = share.first_1min_candle_date {
            let time = time(ts);
            extra_info.insert("first_1m".to_string(), time.to_string());
        };

        if let Some(ts) = share.first_1day_candle_date {
            let time = time(ts);
            extra_info.insert("first_d".to_string(), time.to_string());
        };

        let info = InstrumentInfo::new_share(
            provider, exchange, ticker, name, price_step, lot_size,
            extra_info,
        )
        .map_err(|err| {
            let msg = format!(
                "failed to convert {provider} share '{}' to InstrumentInfo",
                share.ticker
            );
            ConnectorError::conversion(msg, Some(err.into()))
        })?;

        Ok(info)
    }
}
impl TryFrom<api::Future> for InstrumentInfo {
    type Error = ConnectorError;

    fn try_from(future: api::Future) -> Result<Self, Self::Error> {
        let provider = DataProvider::TBank;

        let exchange = Exchange::try_from(future.real_exchange())?;

        let ticker = Ticker::new(future.ticker.clone())
            .expect("T-Bank returned invalid ticker");

        let name = future.name;

        let price_step = match future.min_price_increment {
            Some(value) => Price::from(value),
            None => {
                let msg = format!(
                    "{provider} future {} has no minimum price increment",
                    future.ticker
                );
                return Err(ConnectorError::conversion(msg, None));
            }
        };

        let lot_size = Quantity::new(future.lot as f64)
            .expect("T-Bank lot size must be non-negative");

        let mut extra_info: HashMap<String, String> = HashMap::new();
        extra_info.insert("country".into(), future.country_of_risk);
        extra_info.insert("currency".into(), future.currency);
        extra_info.insert("sector".into(), future.sector);
        extra_info.insert("exchange_section".into(), future.exchange);
        extra_info.insert("class_code".into(), future.class_code);
        extra_info.insert("figi".into(), future.figi);
        extra_info.insert("uid".into(), future.uid);
        extra_info.insert(
            "short_enabled".into(),
            future.short_enabled_flag.to_string(),
        );
        extra_info.insert("futures_type".into(), future.futures_type);
        extra_info.insert("asset_type".into(), future.asset_type);
        extra_info.insert("basic_asset".into(), future.basic_asset);

        if let Some(value) = future.dlong_client {
            extra_info
                .insert("risk_long".into(), f64::from(value).to_string());
        }

        if let Some(value) = future.dshort_client {
            extra_info
                .insert("risk_short".into(), f64::from(value).to_string());
        }

        if let Some(ts) = future.expiration_date {
            let t = time(ts);
            extra_info.insert("expiration_date".into(), t.to_string());
        };

        if let Some(ts) = future.first_trade_date {
            let t = time(ts);
            extra_info.insert("first_trade_date".into(), t.to_string());
        };

        if let Some(ts) = future.last_trade_date {
            let t = time(ts);
            extra_info.insert("last_trade_date".into(), t.to_string());
        };

        if let Some(value) = future.initial_margin_on_buy {
            extra_info
                .insert("buy_margin".into(), Price::from(value).to_string());
        }

        if let Some(value) = future.initial_margin_on_sell {
            extra_info
                .insert("sell_margin".into(), Price::from(value).to_string());
        }

        if let Some(ts) = future.first_1min_candle_date {
            let time = time(ts);
            extra_info.insert("first_1m".into(), time.to_string());
        };

        if let Some(ts) = future.first_1day_candle_date {
            let time = time(ts);
            extra_info.insert("first_d".into(), time.to_string());
        };

        let info = InstrumentInfo::new_future(
            provider, exchange, ticker, name, price_step, lot_size,
            extra_info,
        )
        .map_err(|err| {
            let msg = format!(
                "failed to convert {provider} future '{}' to InstrumentInfo",
                future.ticker
            );
            ConnectorError::conversion(msg, Some(err.into()))
        })?;

        Ok(info)
    }
}
impl TryFrom<api::Bond> for InstrumentInfo {
    type Error = ConnectorError;

    fn try_from(bond: api::Bond) -> Result<Self, Self::Error> {
        let provider = DataProvider::TBank;

        let exchange = Exchange::try_from(bond.real_exchange())?;

        let ticker = Ticker::new(bond.ticker.clone())
            .expect("T-Bank returned invalid ticker");

        let name = bond.name;

        let price_step = match bond.min_price_increment {
            Some(value) => Price::from(value),
            None => {
                let msg = format!(
                    "{provider} bond {} has no minimum price increment",
                    bond.ticker
                );
                return Err(ConnectorError::conversion(msg, None));
            }
        };

        let lot_size = Quantity::new(bond.lot as f64)
            .expect("T-Bank lot size must be non-negative");

        let mut extra_info: HashMap<String, String> = HashMap::new();
        extra_info.insert("country".into(), bond.country_of_risk);
        extra_info.insert("currency".into(), bond.currency);
        extra_info.insert("sector".into(), bond.sector);
        extra_info.insert("exchange_section".into(), bond.exchange);
        extra_info.insert("class_code".into(), bond.class_code);
        extra_info.insert("figi".into(), bond.figi);
        extra_info.insert("isin".into(), bond.isin);
        extra_info.insert("uid".into(), bond.uid);
        extra_info.insert(
            "short_enabled".into(),
            bond.short_enabled_flag.to_string(),
        );
        extra_info.insert(
            "coupon_quantity_per_year".into(),
            bond.coupon_quantity_per_year.to_string(),
        );

        if let Some(ts) = bond.state_reg_date {
            let time = time(ts);
            extra_info.insert("state_reg_date".into(), time.to_string());
        };

        if let Some(ts) = bond.placement_date {
            let time = time(ts);
            extra_info.insert("placement_date".into(), time.to_string());
        };

        if let Some(ts) = bond.maturity_date {
            let time = time(ts);
            extra_info.insert("maturity_date".into(), time.to_string());
        };

        if let Some(value) = bond.nominal {
            extra_info
                .insert("nominal".into(), Price::from(value).to_string());
        }

        if let Some(value) = bond.initial_nominal {
            extra_info.insert(
                "initial_nominal".into(),
                Price::from(value).to_string(),
            );
        }

        if let Some(value) = bond.placement_price {
            extra_info.insert(
                "placement_price".into(),
                Price::from(value).to_string(),
            );
        }

        if let Some(value) = bond.aci_value {
            extra_info
                .insert("aci_value".into(), Price::from(value).to_string());
        }

        if let Some(ts) = bond.call_date {
            let time = time(ts);
            extra_info.insert("call_date".into(), time.to_string());
        };

        if let Some(value) = bond.dlong_client {
            extra_info
                .insert("risk_long".into(), f64::from(value).to_string());
        }

        if let Some(value) = bond.dshort_client {
            extra_info
                .insert("risk_short".into(), f64::from(value).to_string());
        }

        if let Some(ts) = bond.first_1min_candle_date {
            let time = time(ts);
            extra_info.insert("first_1m".into(), time.to_string());
        };

        if let Some(ts) = bond.first_1day_candle_date {
            let time = time(ts);
            extra_info.insert("first_d".into(), time.to_string());
        };

        let info = InstrumentInfo::new_bond(
            provider, exchange, ticker, name, price_step, lot_size,
            extra_info,
        )
        .map_err(|err| {
            let msg = format!(
                "failed to convert {provider} bond '{}' to InstrumentInfo",
                bond.ticker
            );
            ConnectorError::conversion(msg, Some(err.into()))
        })?;

        Ok(info)
    }
}

// private
fn time(ts: prost_types::Timestamp) -> Time {
    let ts = ts.seconds * 1_000_000_000 + ts.nanos as i64;

    Time::new(ts)
}
