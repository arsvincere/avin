#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]

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
            "SECURITY_TRADING_STATUS_SESSION_CLOSE" => Some(Self::SessionClose),
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
/// Запрос установки соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamRequest {
    /// Идентификаторы счетов.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговых поручениях.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamResponse {
    #[prost(oneof = "trades_stream_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<trades_stream_response::Payload>,
}
/// Nested message and enum types in `TradesStreamResponse`.
pub mod trades_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Информация об исполнении торгового поручения.
        #[prost(message, tag = "1")]
        OrderTrades(super::OrderTrades),
        /// Проверка активности стрима.
        #[prost(message, tag = "2")]
        Ping(super::Ping),
    }
}
/// Информация об исполнении торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrades {
    /// Идентификатор торгового поручения.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Дата и время создания сообщения в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub figi: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, repeated, tag = "5")]
    pub trades: ::prost::alloc::vec::Vec<OrderTrade>,
    /// Идентификатор счёта.
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "7")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrade {
    /// Дата и время совершения сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена за 1 инструмент, по которой совершена сделка.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество штук в сделке.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "4")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос выставления торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderRequest {
    /// Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Игнорируется для рыночных поручений.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration = "OrderDirection", tag = "4")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag = "5")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "6")]
    pub order_type: i32,
    /// Идентификатор запроса выставления поручения для целей идемпотентности в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "7")]
    pub order_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значения Figi или Instrument_uid.
    #[prost(string, tag = "8")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о выставлении поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderResponse {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная средняя цена одного инструмента в заявке.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия рассчитанная при выставлении заявки.
    #[prost(message, optional, tag = "8")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "9")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату. Подробнее: [НКД при выставлении торговых поручений](<https://tinkoff.github.io/investAPI/head-orders#coupon>)
    #[prost(message, optional, tag = "10")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "14")]
    pub order_type: i32,
    /// Дополнительные данные об исполнении заявки.
    #[prost(string, tag = "15")]
    pub message: ::prost::alloc::string::String,
    /// Начальная цена заявки в пунктах (для фьючерсов).
    #[prost(message, optional, tag = "16")]
    pub initial_order_price_pt: ::core::option::Option<Quotation>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "17")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Результат отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    /// Дата и время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос получения статуса торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderStateRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Запрос получения списка активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersResponse {
    /// Массив активных заявок.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<OrderState>,
}
/// Информация о торговом поручении.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderState {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по сделке.
    #[prost(message, optional, tag = "8")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия, рассчитанная на момент подачи заявки.
    #[prost(message, optional, tag = "9")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "10")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление заявки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Стадии выполнения заявки.
    #[prost(message, repeated, tag = "14")]
    pub stages: ::prost::alloc::vec::Vec<OrderStage>,
    /// Сервисная комиссия.
    #[prost(message, optional, tag = "15")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    /// Валюта заявки.
    #[prost(string, tag = "16")]
    pub currency: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "17")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub order_date: ::core::option::Option<::prost_types::Timestamp>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "19")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "20")]
    pub order_request_id: ::prost::alloc::string::String,
}
/// Сделки в рамках торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStage {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "3")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос изменения выставленной заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceOrderRequest {
    /// Номер счета.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки на бирже.
    #[prost(string, tag = "6")]
    pub order_id: ::prost::alloc::string::String,
    /// Новый идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов. Перезатирает старый ключ.
    #[prost(string, tag = "7")]
    pub idempotency_key: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена за 1 инструмент.
    #[prost(message, optional, tag = "12")]
    pub price: ::core::option::Option<Quotation>,
    /// Тип цены.
    #[prost(enumeration = "PriceType", tag = "13")]
    pub price_type: i32,
}
/// Направление операции.
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
pub enum OrderDirection {
    /// Значение не указано
    Unspecified = 0,
    /// Покупка
    Buy = 1,
    /// Продажа
    Sell = 2,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Unspecified => "ORDER_DIRECTION_UNSPECIFIED",
            OrderDirection::Buy => "ORDER_DIRECTION_BUY",
            OrderDirection::Sell => "ORDER_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_DIRECTION_BUY" => Some(Self::Buy),
            "ORDER_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Тип заявки.
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
pub enum OrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Лимитная
    Limit = 1,
    /// Рыночная
    Market = 2,
    /// Лучшая цена
    Bestprice = 3,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
            OrderType::Bestprice => "ORDER_TYPE_BESTPRICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_TYPE_LIMIT" => Some(Self::Limit),
            "ORDER_TYPE_MARKET" => Some(Self::Market),
            "ORDER_TYPE_BESTPRICE" => Some(Self::Bestprice),
            _ => None,
        }
    }
}
/// Текущий статус заявки (поручения)
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
pub enum OrderExecutionReportStatus {
    ExecutionReportStatusUnspecified = 0,
    /// Исполнена
    ExecutionReportStatusFill = 1,
    /// Отклонена
    ExecutionReportStatusRejected = 2,
    /// Отменена пользователем
    ExecutionReportStatusCancelled = 3,
    /// Новая
    ExecutionReportStatusNew = 4,
    /// Частично исполнена
    ExecutionReportStatusPartiallyfill = 5,
}
impl OrderExecutionReportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderExecutionReportStatus::ExecutionReportStatusUnspecified => {
                "EXECUTION_REPORT_STATUS_UNSPECIFIED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusFill => {
                "EXECUTION_REPORT_STATUS_FILL"
            }
            OrderExecutionReportStatus::ExecutionReportStatusRejected => {
                "EXECUTION_REPORT_STATUS_REJECTED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusCancelled => {
                "EXECUTION_REPORT_STATUS_CANCELLED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusNew => {
                "EXECUTION_REPORT_STATUS_NEW"
            }
            OrderExecutionReportStatus::ExecutionReportStatusPartiallyfill => {
                "EXECUTION_REPORT_STATUS_PARTIALLYFILL"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_REPORT_STATUS_UNSPECIFIED" => {
                Some(Self::ExecutionReportStatusUnspecified)
            }
            "EXECUTION_REPORT_STATUS_FILL" => {
                Some(Self::ExecutionReportStatusFill)
            }
            "EXECUTION_REPORT_STATUS_REJECTED" => {
                Some(Self::ExecutionReportStatusRejected)
            }
            "EXECUTION_REPORT_STATUS_CANCELLED" => {
                Some(Self::ExecutionReportStatusCancelled)
            }
            "EXECUTION_REPORT_STATUS_NEW" => {
                Some(Self::ExecutionReportStatusNew)
            }
            "EXECUTION_REPORT_STATUS_PARTIALLYFILL" => {
                Some(Self::ExecutionReportStatusPartiallyfill)
            }
            _ => None,
        }
    }
}
/// Тип цены.
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
pub enum PriceType {
    /// Значение не определено.
    Unspecified = 0,
    /// Цена в пунктах (только для фьючерсов и облигаций).
    Point = 1,
    /// Цена в валюте расчётов по инструменту.
    Currency = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::Point => "PRICE_TYPE_POINT",
            PriceType::Currency => "PRICE_TYPE_CURRENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_POINT" => Some(Self::Point),
            "PRICE_TYPE_CURRENCY" => Some(Self::Currency),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod orders_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersStreamServiceClient<T>
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
        ) -> OrdersStreamServiceClient<InterceptedService<T, F>>
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
            OrdersStreamServiceClient::new(InterceptedService::new(
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
        /// Stream сделок пользователя
        pub async fn trades_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::TradesStreamResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/TradesStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated client implementations.
pub mod orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersServiceClient<T>
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
        ) -> OrdersServiceClient<InterceptedService<T, F>>
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
            OrdersServiceClient::new(InterceptedService::new(
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
        /// Метод выставления заявки.
        pub async fn post_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/PostOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод отмены биржевой заявки.
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/CancelOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения статуса торгового поручения.
        pub async fn get_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrderState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка активных заявок по счёту.
        pub async fn get_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод изменения выставленной заявки.
        pub async fn replace_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/ReplaceOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
