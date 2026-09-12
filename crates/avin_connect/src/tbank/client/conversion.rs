// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use avin_core::{Price, Quantity, Time};
use avin_domain::{Exchange, InstrumentInfo, Ticker};

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
                Err(ConnectorError::Conversion {
                    message: msg,
                    source: None,
                })
            }
        }
    }
}
impl TryFrom<api::Share> for InstrumentInfo {
    type Error = ConnectorError;

    fn try_from(share: api::Share) -> Result<Self, Self::Error> {
        // Share {
        //     figi: "BBG004730N88",
        //     ticker: "SBER",
        //     class_code: "TQBR",
        //     isin: "RU0009029540",
        //     lot: 1,
        //     currency: "rub",
        //     klong: None,
        //     kshort: None,
        //     dlong: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 199900000,
        //         },
        //     ),
        //     dshort: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 200000000,
        //         },
        //     ),
        //     dlong_min: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 142800000,
        //         },
        //     ),
        //     dshort_min: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 142800000,
        //         },
        //     ),
        //     short_enabled_flag: true,
        //     name: "Сбербанк",
        //     exchange: "moex_mrng_evng_e_wknd_dlr",
        //     ipo_date: Some(
        //         Timestamp {
        //             seconds: 1184112000,
        //             nanos: 0,
        //         },
        //     ),
        //     issue_size: 21586948000,
        //     country_of_risk: "RU",
        //     country_of_risk_name: "Российская Федерация",
        //     sector: "financial",
        //     issue_size_plan: 21586948000,
        //     nominal: Some(
        //         MoneyValue {
        //             currency: "rub",
        //             units: 3,
        //             nano: 0,
        //         },
        //     ),
        //     trading_status: NormalTrading,
        //     otc_flag: false,
        //     buy_available_flag: true,
        //     sell_available_flag: true,
        //     div_yield_flag: true,
        //     share_type: Common,
        //     min_price_increment: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 10000000,
        //         },
        //     ),
        //     api_trade_available_flag: true,
        //     uid: "e6123145-9665-43e0-8413-cd61b8aa9b13",
        //     real_exchange: Moex,
        //     position_uid: "41eb2102-5333-4713-bf15-72b204c4bf7b",
        //     asset_uid: "40d89385-a03a-4659-bf4e-d3ecba011782",
        //     instrument_exchange: InstrumentExchangeUnspecified,
        //     required_tests: [],
        //     for_iis_flag: true,
        //     for_qual_investor_flag: false,
        //     weekend_flag: true,
        //     blocked_tca_flag: false,
        //     liquidity_flag: true,
        //     first_1min_candle_date: Some(
        //         Timestamp {
        //             seconds: 1520447580,
        //             nanos: 0,
        //         },
        //     ),
        //     first_1day_candle_date: Some(
        //         Timestamp {
        //             seconds: 946969200,
        //             nanos: 0,
        //         },
        //     ),
        //     brand: Some(
        //         BrandData {
        //             logo_name: "sber3.png",
        //             logo_base_color: "#309c0b",
        //             text_color: "#ffffff",
        //         },
        //     ),
        //     dlong_client: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 142800000,
        //         },
        //     ),
        //     dshort_client: Some(
        //         Quotation {
        //             units: 0,
        //             nano: 142800000,
        //         },
        //     ),
        // }

        let exchange = Exchange::try_from(share.real_exchange())?;

        let ticker = Ticker::new(share.ticker.clone())
            .expect("T-Bank returned invalid ticker");

        let name = share.name;

        let price_step = match share.min_price_increment {
            Some(value) => Price::new(f64::from(value)).unwrap(),
            None => {
                let msg = format!(
                    "T-Bank share {} has no minimum price increment",
                    share.ticker
                );
                return Err(ConnectorError::Conversion {
                    message: msg,
                    source: None,
                });
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

        if let Some(value) = share.dlong_client {
            extra_info.insert(
                "long_risk".to_string(),
                f64::from(value).to_string(),
            );
        }

        if let Some(value) = share.dshort_client {
            extra_info.insert(
                "short_risk".to_string(),
                f64::from(value).to_string(),
            );
        }

        if let Some(ts) = share.first_1min_candle_date {
            let time = time(ts);
            extra_info.insert("first_1m".to_string(), time.to_string());
        };

        if let Some(ts) = share.first_1day_candle_date {
            let time = time(ts);
            extra_info.insert("first_d".to_string(), time.to_string());
        };

        let info = InstrumentInfo::new_share(
            exchange, ticker, name, price_step, lot_size, extra_info,
        )
        .map_err(|err| {
            let msg = format!(
                "failed to convert T-Bank share '{}' to InstrumentInfo",
                share.ticker
            );
            ConnectorError::Conversion {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        Ok(info)
    }
}

// private
fn time(ts: prost_types::Timestamp) -> Time {
    let ts = ts.seconds * 1_000_000_000 + ts.nanos as i64;

    Time::new(ts)
}
