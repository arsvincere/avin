/// Денежная сумма в определенной валюте
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyValue {
    /// строковый ISO-код валюты
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "3")]
    pub nano: i32,
}
/// Котировка — денежная сумма без указания валюты
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quotation {
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "1")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "2")]
    pub nano: i32,
}
/// Проверка активности стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Время проверки.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Тип инструмента.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum InstrumentType {
    Unspecified = 0,
    /// Облигация.
    Bond = 1,
    /// Акция.
    Share = 2,
    /// Валюта.
    Currency = 3,
    /// Exchange-traded fund. Фонд.
    Etf = 4,
    /// Фьючерс.
    Futures = 5,
    /// Структурная нота.
    Sp = 6,
    /// Опцион.
    Option = 7,
    /// Clearing certificate.
    ClearingCertificate = 8,
}
impl InstrumentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentType::Unspecified => "INSTRUMENT_TYPE_UNSPECIFIED",
            InstrumentType::Bond => "INSTRUMENT_TYPE_BOND",
            InstrumentType::Share => "INSTRUMENT_TYPE_SHARE",
            InstrumentType::Currency => "INSTRUMENT_TYPE_CURRENCY",
            InstrumentType::Etf => "INSTRUMENT_TYPE_ETF",
            InstrumentType::Futures => "INSTRUMENT_TYPE_FUTURES",
            InstrumentType::Sp => "INSTRUMENT_TYPE_SP",
            InstrumentType::Option => "INSTRUMENT_TYPE_OPTION",
            InstrumentType::ClearingCertificate => {
                "INSTRUMENT_TYPE_CLEARING_CERTIFICATE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTRUMENT_TYPE_BOND" => Some(Self::Bond),
            "INSTRUMENT_TYPE_SHARE" => Some(Self::Share),
            "INSTRUMENT_TYPE_CURRENCY" => Some(Self::Currency),
            "INSTRUMENT_TYPE_ETF" => Some(Self::Etf),
            "INSTRUMENT_TYPE_FUTURES" => Some(Self::Futures),
            "INSTRUMENT_TYPE_SP" => Some(Self::Sp),
            "INSTRUMENT_TYPE_OPTION" => Some(Self::Option),
            "INSTRUMENT_TYPE_CLEARING_CERTIFICATE" => {
                Some(Self::ClearingCertificate)
            }
            _ => None,
        }
    }
}
/// Режим торгов инструмента
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён
    Unspecified = 0,
    /// Недоступен для торгов
    NotAvailableForTrading = 1,
    /// Период открытия торгов
    OpeningPeriod = 2,
    /// Период закрытия торгов
    ClosingPeriod = 3,
    /// Перерыв в торговле
    BreakInTrading = 4,
    /// Нормальная торговля
    NormalTrading = 5,
    /// Аукцион закрытия
    ClosingAuction = 6,
    /// Аукцион крупных пакетов
    DarkPoolAuction = 7,
    /// Дискретный аукцион
    DiscreteAuction = 8,
    /// Аукцион открытия
    OpeningAuctionPeriod = 9,
    /// Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice = 10,
    /// Сессия назначена
    SessionAssigned = 11,
    /// Сессия закрыта
    SessionClose = 12,
    /// Сессия открыта
    SessionOpen = 13,
    /// Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading = 14,
    /// Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading = 15,
    /// Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading = 16,
}
impl SecurityTradingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityTradingStatus::Unspecified => {
                "SECURITY_TRADING_STATUS_UNSPECIFIED"
            }
            SecurityTradingStatus::NotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING"
            }
            SecurityTradingStatus::OpeningPeriod => {
                "SECURITY_TRADING_STATUS_OPENING_PERIOD"
            }
            SecurityTradingStatus::ClosingPeriod => {
                "SECURITY_TRADING_STATUS_CLOSING_PERIOD"
            }
            SecurityTradingStatus::BreakInTrading => {
                "SECURITY_TRADING_STATUS_BREAK_IN_TRADING"
            }
            SecurityTradingStatus::NormalTrading => {
                "SECURITY_TRADING_STATUS_NORMAL_TRADING"
            }
            SecurityTradingStatus::ClosingAuction => {
                "SECURITY_TRADING_STATUS_CLOSING_AUCTION"
            }
            SecurityTradingStatus::DarkPoolAuction => {
                "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION"
            }
            SecurityTradingStatus::DiscreteAuction => {
                "SECURITY_TRADING_STATUS_DISCRETE_AUCTION"
            }
            SecurityTradingStatus::OpeningAuctionPeriod => {
                "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD"
            }
            SecurityTradingStatus::TradingAtClosingAuctionPrice => {
                "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE"
            }
            SecurityTradingStatus::SessionAssigned => {
                "SECURITY_TRADING_STATUS_SESSION_ASSIGNED"
            }
            SecurityTradingStatus::SessionClose => {
                "SECURITY_TRADING_STATUS_SESSION_CLOSE"
            }
            SecurityTradingStatus::SessionOpen => {
                "SECURITY_TRADING_STATUS_SESSION_OPEN"
            }
            SecurityTradingStatus::DealerNormalTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING"
            }
            SecurityTradingStatus::DealerBreakInTrading => {
                "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING"
            }
            SecurityTradingStatus::DealerNotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECURITY_TRADING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::NotAvailableForTrading)
            }
            "SECURITY_TRADING_STATUS_OPENING_PERIOD" => {
                Some(Self::OpeningPeriod)
            }
            "SECURITY_TRADING_STATUS_CLOSING_PERIOD" => {
                Some(Self::ClosingPeriod)
            }
            "SECURITY_TRADING_STATUS_BREAK_IN_TRADING" => {
                Some(Self::BreakInTrading)
            }
            "SECURITY_TRADING_STATUS_NORMAL_TRADING" => {
                Some(Self::NormalTrading)
            }
            "SECURITY_TRADING_STATUS_CLOSING_AUCTION" => {
                Some(Self::ClosingAuction)
            }
            "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION" => {
                Some(Self::DarkPoolAuction)
            }
            "SECURITY_TRADING_STATUS_DISCRETE_AUCTION" => {
                Some(Self::DiscreteAuction)
            }
            "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD" => {
                Some(Self::OpeningAuctionPeriod)
            }
            "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE" => {
                Some(Self::TradingAtClosingAuctionPrice)
            }
            "SECURITY_TRADING_STATUS_SESSION_ASSIGNED" => {
                Some(Self::SessionAssigned)
            }
            "SECURITY_TRADING_STATUS_SESSION_CLOSE" => {
                Some(Self::SessionClose)
            }
            "SECURITY_TRADING_STATUS_SESSION_OPEN" => Some(Self::SessionOpen),
            "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING" => {
                Some(Self::DealerNormalTrading)
            }
            "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING" => {
                Some(Self::DealerBreakInTrading)
            }
            "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::DealerNotAvailableForTrading)
            }
            _ => None,
        }
    }
}
/// Запрос подписки или отписки на определённые биржевые данные.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataRequest {
    #[prost(
        oneof = "market_data_request::Payload",
        tags = "1, 2, 3, 4, 5, 6"
    )]
    pub payload: ::core::option::Option<market_data_request::Payload>,
}
/// Nested message and enum types in `MarketDataRequest`.
pub mod market_data_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Запрос подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesRequest(super::SubscribeCandlesRequest),
        /// Запрос подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookRequest(super::SubscribeOrderBookRequest),
        /// Запрос подписки на ленту обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesRequest(super::SubscribeTradesRequest),
        /// Запрос подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoRequest(super::SubscribeInfoRequest),
        /// Запрос подписки на цены последних сделок.
        #[prost(message, tag = "5")]
        SubscribeLastPriceRequest(super::SubscribeLastPriceRequest),
        /// Запрос своих подписок.
        #[prost(message, tag = "6")]
        GetMySubscriptions(super::GetMySubscriptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataServerSideStreamRequest {
    /// Запрос подписки на свечи.
    #[prost(message, optional, tag = "1")]
    pub subscribe_candles_request:
        ::core::option::Option<SubscribeCandlesRequest>,
    /// Запрос подписки на стаканы.
    #[prost(message, optional, tag = "2")]
    pub subscribe_order_book_request:
        ::core::option::Option<SubscribeOrderBookRequest>,
    /// Запрос подписки на ленту обезличенных сделок.
    #[prost(message, optional, tag = "3")]
    pub subscribe_trades_request:
        ::core::option::Option<SubscribeTradesRequest>,
    /// Запрос подписки на торговые статусы инструментов.
    #[prost(message, optional, tag = "4")]
    pub subscribe_info_request: ::core::option::Option<SubscribeInfoRequest>,
    /// Запрос подписки на цены последних сделок.
    #[prost(message, optional, tag = "5")]
    pub subscribe_last_price_request:
        ::core::option::Option<SubscribeLastPriceRequest>,
}
/// Пакет биржевой информации по подписке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataResponse {
    #[prost(
        oneof = "market_data_response::Payload",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub payload: ::core::option::Option<market_data_response::Payload>,
}
/// Nested message and enum types in `MarketDataResponse`.
pub mod market_data_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Результат подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesResponse(super::SubscribeCandlesResponse),
        /// Результат подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookResponse(super::SubscribeOrderBookResponse),
        /// Результат подписки на поток обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesResponse(super::SubscribeTradesResponse),
        /// Результат подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoResponse(super::SubscribeInfoResponse),
        /// Свеча.
        #[prost(message, tag = "5")]
        Candle(super::Candle),
        /// Сделки.
        #[prost(message, tag = "6")]
        Trade(super::Trade),
        /// Стакан.
        #[prost(message, tag = "7")]
        Orderbook(super::OrderBook),
        /// Торговый статус.
        #[prost(message, tag = "8")]
        TradingStatus(super::TradingStatus),
        /// Проверка активности стрима.
        #[prost(message, tag = "9")]
        Ping(super::Ping),
        /// Результат подписки на цены последние сделок по инструментам.
        #[prost(message, tag = "10")]
        SubscribeLastPriceResponse(super::SubscribeLastPriceResponse),
        /// Цена последней сделки.
        #[prost(message, tag = "11")]
        LastPrice(super::LastPrice),
    }
}
/// subscribeCandles | Изменения статуса подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<CandleInstrument>,
    /// Флаг ожидания закрытия временного интервала для отправки свечи, применяется только для минутных свечей.
    #[prost(bool, tag = "3")]
    pub waiting_close: bool,
}
/// Запрос изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub candles_subscriptions: ::prost::alloc::vec::Vec<CandleSubscription>,
}
/// Статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос на изменение статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<OrderBookInstrument>,
}
/// Запрос подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub order_book_subscriptions:
        ::prost::alloc::vec::Vec<OrderBookSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на поток обезличенных сделок.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<TradeInstrument>,
}
/// Запрос подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на поток сделок.
    #[prost(message, repeated, tag = "2")]
    pub trade_subscriptions: ::prost::alloc::vec::Vec<TradeSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на торговый статус инструмента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<InfoInstrument>,
}
/// Запрос подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub info_subscriptions: ::prost::alloc::vec::Vec<InfoSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на цену последней сделки по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<LastPriceInstrument>,
}
/// Запрос подписки на последнюю цену.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub last_price_subscriptions:
        ::prost::alloc::vec::Vec<LastPriceSubscription>,
}
/// Статус подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет свечей в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечи.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём сделок в лотах.
    #[prost(int64, tag = "7")]
    pub volume: i64,
    /// Время начала интервала свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время последней сделки, вошедшей в свечу в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub last_trade_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "10")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет стаканов в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Флаг консистентности стакана. **false** значит не все заявки попали в стакан по причинам сетевых задержек или нарушения порядка доставки.
    #[prost(bool, tag = "3")]
    pub is_consistent: bool,
    /// Массив предложений.
    #[prost(message, repeated, tag = "4")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Массив спроса.
    #[prost(message, repeated, tag = "5")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Время формирования стакана в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Uid инструмента
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Массив предложений/спроса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество в лотах.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "TradeDirection", tag = "2")]
    pub direction: i32,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество лотов.
    #[prost(int64, tag = "4")]
    pub quantity: i64,
    /// Время сделки в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет изменения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStatus {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Время изменения торгового статуса в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "5")]
    pub market_order_available_flag: bool,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос исторических свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Интервал запрошенных свечей.
    #[prost(enumeration = "CandleInterval", tag = "4")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "5")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Список свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesResponse {
    /// Массив свечей.
    #[prost(message, repeated, tag = "1")]
    pub candles: ::prost::alloc::vec::Vec<HistoricCandle>,
}
/// Информация о свече.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricCandle {
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём торгов в лотах.
    #[prost(int64, tag = "5")]
    pub volume: i64,
    /// Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак завершённости свечи. **false** значит, свеча за текущие интервал ещё сформирована не полностью.
    #[prost(bool, tag = "7")]
    pub is_complete: bool,
}
/// Запрос получения цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, repeated, tag = "1")]
    pub figi: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Массив идентификаторов инструмента, принимает значения figi или instrument_uid.
    #[prost(string, repeated, tag = "2")]
    pub instrument_id:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Список цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesResponse {
    /// Массив цен последних сделок.
    #[prost(message, repeated, tag = "1")]
    pub last_prices: ::prost::alloc::vec::Vec<LastPrice>,
}
/// Информация о цене последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPrice {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Время получения последней цены в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "11")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос стакана.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о стакане.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Множество пар значений на покупку.
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Множество пар значений на продажу.
    #[prost(message, repeated, tag = "4")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub last_price: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close_price: ::core::option::Option<Quotation>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Время получения цены последней сделки.
    #[prost(message, optional, tag = "21")]
    pub last_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время получения цены закрытия.
    #[prost(message, optional, tag = "22")]
    pub close_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время формирования стакана на бирже.
    #[prost(message, optional, tag = "23")]
    pub orderbook_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента.
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, repeated, tag = "1")]
    pub instrument_id:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesResponse {
    /// Массив информации о торговых статусах
    #[prost(message, repeated, tag = "1")]
    pub trading_statuses: ::prost::alloc::vec::Vec<GetTradingStatusResponse>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "3")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub market_order_available_flag: bool,
    /// Признак доступности торгов через API.
    #[prost(bool, tag = "5")]
    pub api_trade_available_flag: bool,
    /// Uid инструмента.
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesResponse {
    /// Массив сделок.
    #[prost(message, repeated, tag = "1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
/// Запрос активных подписок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMySubscriptions {}
/// Запрос цен закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesRequest {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentClosePriceRequest>,
}
/// Запрос цен закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "1")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Цены закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesResponse {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub close_prices: ::prost::alloc::vec::Vec<InstrumentClosePriceResponse>,
}
/// Цена закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceResponse {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Uid инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Цена закрытия торговой сессии.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<Quotation>,
    /// Дата совершения торгов.
    #[prost(message, optional, tag = "21")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Тип операции со списком подписок.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SubscriptionAction {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Подписаться.
    Subscribe = 1,
    /// Отписаться.
    Unsubscribe = 2,
}
impl SubscriptionAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionAction::Unspecified => {
                "SUBSCRIPTION_ACTION_UNSPECIFIED"
            }
            SubscriptionAction::Subscribe => "SUBSCRIPTION_ACTION_SUBSCRIBE",
            SubscriptionAction::Unsubscribe => {
                "SUBSCRIPTION_ACTION_UNSUBSCRIBE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_ACTION_SUBSCRIBE" => Some(Self::Subscribe),
            "SUBSCRIPTION_ACTION_UNSUBSCRIBE" => Some(Self::Unsubscribe),
            _ => None,
        }
    }
}
/// Интервал свечи.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SubscriptionInterval {
    /// Интервал свечи не определён.
    Unspecified = 0,
    /// Минутные свечи.
    OneMinute = 1,
    /// Пятиминутные свечи.
    FiveMinutes = 2,
}
impl SubscriptionInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionInterval::Unspecified => {
                "SUBSCRIPTION_INTERVAL_UNSPECIFIED"
            }
            SubscriptionInterval::OneMinute => {
                "SUBSCRIPTION_INTERVAL_ONE_MINUTE"
            }
            SubscriptionInterval::FiveMinutes => {
                "SUBSCRIPTION_INTERVAL_FIVE_MINUTES"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_INTERVAL_ONE_MINUTE" => Some(Self::OneMinute),
            "SUBSCRIPTION_INTERVAL_FIVE_MINUTES" => Some(Self::FiveMinutes),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum SubscriptionStatus {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Инструмент не найден.
    InstrumentNotFound = 2,
    /// Некорректный статус подписки, список возможных значений: \[SubscriptionAction\](<https://tinkoff.github.io/investAPI/marketdata#subscriptionaction>).
    SubscriptionActionIsInvalid = 3,
    /// Некорректная глубина стакана, доступные значения: 1, 10, 20, 30, 40, 50.
    DepthIsInvalid = 4,
    /// Некорректный интервал свечей, список возможных значений: \[SubscriptionInterval\](<https://tinkoff.github.io/investAPI/marketdata#subscriptioninterval>).
    IntervalIsInvalid = 5,
    /// Превышен лимит на общее количество подписок в рамках стрима, подробнее: [Лимитная политика](<https://tinkoff.github.io/investAPI/limits/>).
    LimitIsExceeded = 6,
    /// Внутренняя ошибка сервиса.
    InternalError = 7,
    /// Превышен лимит на количество запросов на подписки в течение установленного отрезка времени
    TooManyRequests = 8,
    /// Активная подписка не найдена. Ошибка может возникнуть только при отписке от не существующей отписки
    SubscriptionNotFound = 9,
}
impl SubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionStatus::Unspecified => {
                "SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            SubscriptionStatus::Success => "SUBSCRIPTION_STATUS_SUCCESS",
            SubscriptionStatus::InstrumentNotFound => {
                "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND"
            }
            SubscriptionStatus::SubscriptionActionIsInvalid => {
                "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID"
            }
            SubscriptionStatus::DepthIsInvalid => {
                "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID"
            }
            SubscriptionStatus::IntervalIsInvalid => {
                "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID"
            }
            SubscriptionStatus::LimitIsExceeded => {
                "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED"
            }
            SubscriptionStatus::InternalError => {
                "SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
            SubscriptionStatus::TooManyRequests => {
                "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS"
            }
            SubscriptionStatus::SubscriptionNotFound => {
                "SUBSCRIPTION_STATUS_SUBSCRIPTION_NOT_FOUND"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND" => {
                Some(Self::InstrumentNotFound)
            }
            "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID" => {
                Some(Self::SubscriptionActionIsInvalid)
            }
            "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID" => {
                Some(Self::DepthIsInvalid)
            }
            "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID" => {
                Some(Self::IntervalIsInvalid)
            }
            "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED" => {
                Some(Self::LimitIsExceeded)
            }
            "SUBSCRIPTION_STATUS_INTERNAL_ERROR" => Some(Self::InternalError),
            "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS" => {
                Some(Self::TooManyRequests)
            }
            "SUBSCRIPTION_STATUS_SUBSCRIPTION_NOT_FOUND" => {
                Some(Self::SubscriptionNotFound)
            }
            _ => None,
        }
    }
}
/// Направление сделки.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum TradeDirection {
    /// Направление сделки не определено.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl TradeDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDirection::Unspecified => "TRADE_DIRECTION_UNSPECIFIED",
            TradeDirection::Buy => "TRADE_DIRECTION_BUY",
            TradeDirection::Sell => "TRADE_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRADE_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "TRADE_DIRECTION_BUY" => Some(Self::Buy),
            "TRADE_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Интервал свечей.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum CandleInterval {
    /// Интервал не определён.
    Unspecified = 0,
    /// от 1 минуты до 1 дня.
    CandleInterval1Min = 1,
    /// от 5 минут до 1 дня.
    CandleInterval5Min = 2,
    /// от 15 минут до 1 дня.
    CandleInterval15Min = 3,
    /// от 1 часа до 1 недели.
    Hour = 4,
    /// от 1 дня до 1 года.
    Day = 5,
    /// от 2 минут до 1 дня.
    CandleInterval2Min = 6,
    /// от 3 минут до 1 дня.
    CandleInterval3Min = 7,
    /// от 10 минут до 1 дня.
    CandleInterval10Min = 8,
    /// от 30 минут до 2 дней.
    CandleInterval30Min = 9,
    /// от 2 часов до 1 месяца.
    CandleInterval2Hour = 10,
    /// от 4 часов до 1 месяца.
    CandleInterval4Hour = 11,
    /// от 1 недели до 2 лет.
    Week = 12,
    /// от 1 месяца до 10 лет.
    Month = 13,
}
impl CandleInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandleInterval::Unspecified => "CANDLE_INTERVAL_UNSPECIFIED",
            CandleInterval::CandleInterval1Min => "CANDLE_INTERVAL_1_MIN",
            CandleInterval::CandleInterval5Min => "CANDLE_INTERVAL_5_MIN",
            CandleInterval::CandleInterval15Min => "CANDLE_INTERVAL_15_MIN",
            CandleInterval::Hour => "CANDLE_INTERVAL_HOUR",
            CandleInterval::Day => "CANDLE_INTERVAL_DAY",
            CandleInterval::CandleInterval2Min => "CANDLE_INTERVAL_2_MIN",
            CandleInterval::CandleInterval3Min => "CANDLE_INTERVAL_3_MIN",
            CandleInterval::CandleInterval10Min => "CANDLE_INTERVAL_10_MIN",
            CandleInterval::CandleInterval30Min => "CANDLE_INTERVAL_30_MIN",
            CandleInterval::CandleInterval2Hour => "CANDLE_INTERVAL_2_HOUR",
            CandleInterval::CandleInterval4Hour => "CANDLE_INTERVAL_4_HOUR",
            CandleInterval::Week => "CANDLE_INTERVAL_WEEK",
            CandleInterval::Month => "CANDLE_INTERVAL_MONTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CANDLE_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "CANDLE_INTERVAL_1_MIN" => Some(Self::CandleInterval1Min),
            "CANDLE_INTERVAL_5_MIN" => Some(Self::CandleInterval5Min),
            "CANDLE_INTERVAL_15_MIN" => Some(Self::CandleInterval15Min),
            "CANDLE_INTERVAL_HOUR" => Some(Self::Hour),
            "CANDLE_INTERVAL_DAY" => Some(Self::Day),
            "CANDLE_INTERVAL_2_MIN" => Some(Self::CandleInterval2Min),
            "CANDLE_INTERVAL_3_MIN" => Some(Self::CandleInterval3Min),
            "CANDLE_INTERVAL_10_MIN" => Some(Self::CandleInterval10Min),
            "CANDLE_INTERVAL_30_MIN" => Some(Self::CandleInterval30Min),
            "CANDLE_INTERVAL_2_HOUR" => Some(Self::CandleInterval2Hour),
            "CANDLE_INTERVAL_4_HOUR" => Some(Self::CandleInterval4Hour),
            "CANDLE_INTERVAL_WEEK" => Some(Self::Week),
            "CANDLE_INTERVAL_MONTH" => Some(Self::Month),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod market_data_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(
            dst: D,
        ) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn =
                tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T:
                tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                        Response = http::Response<
                            <T as tonic::client::GrpcService<
                                tonic::body::BoxBody,
                            >>::ResponseBody,
                        >,
                    >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Метод запроса исторических свечей по инструменту.
        pub async fn get_candles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCandlesRequest>,
        ) -> Result<tonic::Response<super::GetCandlesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetCandles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса цен последних сделок по инструментам.
        pub async fn get_last_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPricesRequest>,
        ) -> Result<
            tonic::Response<super::GetLastPricesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastPrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения стакана по инструменту.
        pub async fn get_order_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderBookRequest>,
        ) -> Result<tonic::Response<super::GetOrderBookResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetOrderBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusRequest>,
        ) -> Result<
            tonic::Response<super::GetTradingStatusResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusesRequest>,
        ) -> Result<
            tonic::Response<super::GetTradingStatusesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса обезличенных сделок за последний час.
        pub async fn get_last_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastTradesRequest>,
        ) -> Result<
            tonic::Response<super::GetLastTradesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastTrades",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса цен закрытия торговой сессии по инструментам.
        pub async fn get_close_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClosePricesRequest>,
        ) -> Result<
            tonic::Response<super::GetClosePricesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetClosePrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod market_data_stream_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(
            dst: D,
        ) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn =
                tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T:
                tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                        Response = http::Response<
                            <T as tonic::client::GrpcService<
                                tonic::body::BoxBody,
                            >>::ResponseBody,
                        >,
                    >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataStreamServiceClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Bi-directional стрим предоставления биржевой информации.
        pub async fn market_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::MarketDataRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::MarketDataResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Server-side стрим предоставления биржевой информации.
        pub async fn market_data_server_side_stream(
            &mut self,
            request: impl tonic::IntoRequest<
                super::MarketDataServerSideStreamRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::MarketDataResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataServerSideStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
