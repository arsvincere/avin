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
            Some(value) => Price::new(f64::from(value)).unwrap(),
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

// private
fn time(ts: prost_types::Timestamp) -> Time {
    let ts = ts.seconds * 1_000_000_000 + ts.nanos as i64;

    Time::new(ts)
}
