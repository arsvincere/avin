// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#![allow(unused)]

use std::collections::HashMap;

use avin_core::{Price, Quantity};
use avin_domain::{Exchange, InstrumentInfo, InstrumentList, Ticker};

use crate::ConnectError;

use super::api;
use super::api::instruments_service_client::InstrumentsServiceClient;
use super::interceptor::TBankInterceptor;

const ENDPOINT: &str = "https://invest-public-api.tbank.ru:443";

pub struct TBankClient {
    channel: tonic::transport::Channel,
    interceptor: TBankInterceptor,
}

impl TBankClient {
    pub async fn connect(token: &str) -> Result<Self, ConnectError> {
        let interceptor = TBankInterceptor::new(token).map_err(|err| {
            ConnectError::Authorization {
                message: "invalid TBank authorization token".to_string(),
                source: Some(Box::new(err)),
            }
        })?;

        let tls =
            tonic::transport::ClientTlsConfig::new().with_native_roots();

        let channel = tonic::transport::Channel::from_static(ENDPOINT)
            .tls_config(tls)
            .map_err(|err| ConnectError::TBank {
                message: "failed to configure TBank TLS".to_string(),
                source: Some(Box::new(err)),
            })?
            .connect()
            .await
            .map_err(|err| ConnectError::TBank {
                message: "failed to connect to TBank API".to_string(),
                source: Some(Box::new(err)),
            })?;

        Ok(Self {
            channel,
            interceptor,
        })
    }

    pub async fn shares(&self) -> Result<InstrumentList, ConnectError> {
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
                let msg = "TODO msg".to_string();
                ConnectError::TBank {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?
            .into_inner();

        let mut instruments = InstrumentList::new();

        for share in response.instruments {
            if !supported(&share) {
                continue;
            }

            let info = convert_share(share)?;

            instruments.add(info).map_err(|err| {
                let msg = "TBank failed add instrument".to_string();
                ConnectError::TBank {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?;
        }

        Ok(instruments)
    }
}

fn convert_share(share: api::Share) -> Result<InstrumentInfo, ConnectError> {
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

    let exchange = convert_exchange(share.real_exchange())?;
    let ticker = Ticker::new(share.ticker.clone()).unwrap();
    let name = share.name;

    let price_step = match share.min_price_increment {
        Some(value) => Price::new(convert_qutation(value)).unwrap(),
        None => {
            let msg = format!(
                "TBank share {} has no min_price_increment",
                share.ticker
            );
            return Err(ConnectError::TBank {
                message: msg,
                source: None,
            });
        }
    };

    let lot_size = Quantity::new(share.lot as f64).unwrap();

    let short_enabled = share.short_enabled_flag.to_string();

    let k_long: f64 = match share.dlong {
        Some(value) => convert_qutation(value),
        None => 1.0,
    };
    let k_short: f64 = match share.dshort {
        Some(value) => convert_qutation(value),
        None => 1.0,
    };
    let k_long_qual: f64 = match share.dlong_min {
        Some(value) => convert_qutation(value),
        None => 1.0,
    };
    let k_short_qual: f64 = match share.dshort_min {
        Some(value) => convert_qutation(value),
        None => 1.0,
    };
    let first_1m_ts = match share.first_1min_candle_date {
        Some(ts) => {
            let ts = ts.seconds * 1_000_000_000 + ts.nanos as i64;
            ts.to_string()
        }
        None => String::new(),
    };
    let first_d_ts = match share.first_1day_candle_date {
        Some(ts) => {
            let ts = ts.seconds * 1_000_000_000 + ts.nanos as i64;
            ts.to_string()
        }
        None => String::new(),
    };

    let mut extra_info = HashMap::new();
    extra_info.insert("country".to_string(), share.country_of_risk);
    extra_info.insert("currency".to_string(), share.currency);
    extra_info.insert("sector".to_string(), share.sector);
    extra_info.insert("exchange_section".to_string(), share.exchange);
    extra_info.insert("class_code".to_string(), share.class_code);
    extra_info.insert("figi".to_string(), share.figi);
    extra_info.insert("isin".to_string(), share.isin);
    extra_info.insert("uid".to_string(), share.uid);
    extra_info.insert("short_enabled".to_string(), short_enabled);
    extra_info.insert("k_long".to_string(), k_long.to_string());
    extra_info.insert("k_short".to_string(), k_short.to_string());
    extra_info.insert("k_long_qual".to_string(), k_long_qual.to_string());
    extra_info.insert("k_short_qual".to_string(), k_short_qual.to_string());
    extra_info.insert("first_1m".to_string(), first_1m_ts);
    extra_info.insert("first_d".to_string(), first_d_ts);

    let info = InstrumentInfo::new_share(
        exchange, ticker, name, price_step, lot_size, extra_info,
    )
    .unwrap();

    Ok(info)
}

fn convert_exchange(
    exchange: api::RealExchange,
) -> Result<Exchange, ConnectError> {
    match exchange {
        api::RealExchange::Moex => Ok(Exchange::Moex),
        api::RealExchange::Rts => Ok(Exchange::Spb),
        api::RealExchange::Unspecified => todo!(),
        api::RealExchange::Otc => todo!(),
        api::RealExchange::Dealer => todo!(),
    }
}

fn convert_qutation(value: api::Quotation) -> f64 {
    let frac: f64 = value.nano as f64 / 1_000_000_000.0;

    value.units as f64 + frac
}

fn supported(share: &api::Share) -> bool {
    // бывает для инструментов которые уже не торгуются
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
