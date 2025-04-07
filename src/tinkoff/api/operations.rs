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
/// Запрос получения списка операций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус запрашиваемых операций.
    #[prost(enumeration = "OperationState", tag = "4")]
    pub state: i32,
    /// Figi-идентификатор инструмента для фильтрации.
    #[prost(string, tag = "5")]
    pub figi: ::prost::alloc::string::String,
}
/// Список операций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsResponse {
    /// Массив операций.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
/// Данные по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Идентификатор операции.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции.
    #[prost(string, tag = "2")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Валюта операции.
    #[prost(string, tag = "3")]
    pub currency: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "4")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "5")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Статус операции.
    #[prost(enumeration = "OperationState", tag = "6")]
    pub state: i32,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "7")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "8")]
    pub quantity_rest: i64,
    /// Figi-идентификатор инструмента, связанного с операцией.
    #[prost(string, tag = "9")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента. Возможные значения: </br>**bond** — облигация; </br>**share** — акция; </br>**currency** — валюта; </br>**etf** — фонд; </br>**futures** — фьючерс.
    #[prost(string, tag = "10")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Дата и время операции в формате часовом поясе UTC.
    #[prost(message, optional, tag = "11")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текстовое описание типа операции.
    #[prost(string, tag = "12")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "13")]
    pub operation_type: i32,
    /// Массив сделок.
    #[prost(message, repeated, tag = "14")]
    pub trades: ::prost::alloc::vec::Vec<OperationTrade>,
    /// Идентификатор актива
    #[prost(string, tag = "16")]
    pub asset_uid: ::prost::alloc::string::String,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "17")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "18")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationTrade {
    /// Идентификатор сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Дата и время сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество инструментов.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<MoneyValue>,
}
/// Запрос получения текущего портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Валюта, в которой требуется рассчитать портфель
    #[prost(enumeration = "portfolio_request::CurrencyRequest", tag = "2")]
    pub currency: i32,
}
/// Nested message and enum types in `PortfolioRequest`.
pub mod portfolio_request {
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
    pub enum CurrencyRequest {
        /// Рубли
        Rub = 0,
        /// Доллары
        Usd = 1,
        /// Евро
        Eur = 2,
    }
    impl CurrencyRequest {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CurrencyRequest::Rub => "RUB",
                CurrencyRequest::Usd => "USD",
                CurrencyRequest::Eur => "EUR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RUB" => Some(Self::Rub),
                "USD" => Some(Self::Usd),
                "EUR" => Some(Self::Eur),
                _ => None,
            }
        }
    }
}
/// Текущий портфель по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// Общая стоимость акций в портфеле.
    #[prost(message, optional, tag = "1")]
    pub total_amount_shares: ::core::option::Option<MoneyValue>,
    /// Общая стоимость облигаций в портфеле.
    #[prost(message, optional, tag = "2")]
    pub total_amount_bonds: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фондов в портфеле.
    #[prost(message, optional, tag = "3")]
    pub total_amount_etf: ::core::option::Option<MoneyValue>,
    /// Общая стоимость валют в портфеле.
    #[prost(message, optional, tag = "4")]
    pub total_amount_currencies: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фьючерсов в портфеле.
    #[prost(message, optional, tag = "5")]
    pub total_amount_futures: ::core::option::Option<MoneyValue>,
    /// Текущая относительная доходность портфеля, в %.
    #[prost(message, optional, tag = "6")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Список позиций портфеля.
    #[prost(message, repeated, tag = "7")]
    pub positions: ::prost::alloc::vec::Vec<PortfolioPosition>,
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "8")]
    pub account_id: ::prost::alloc::string::String,
    /// Общая стоимость опционов в портфеле.
    #[prost(message, optional, tag = "9")]
    pub total_amount_options: ::core::option::Option<MoneyValue>,
    /// Общая стоимость структурных нот в портфеле.
    #[prost(message, optional, tag = "10")]
    pub total_amount_sp: ::core::option::Option<MoneyValue>,
    /// Общая стоимость портфеля.
    #[prost(message, optional, tag = "11")]
    pub total_amount_portfolio: ::core::option::Option<MoneyValue>,
    /// Массив виртуальных позиций портфеля.
    #[prost(message, repeated, tag = "12")]
    pub virtual_positions: ::prost::alloc::vec::Vec<VirtualPortfolioPosition>,
}
/// Запрос позиций портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список позиций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Признак идущей в данный момент выгрузки лимитов.
    #[prost(bool, tag = "4")]
    pub limits_loading_in_progress: bool,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
}
/// Запрос доступного для вывода остатка.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Доступный для вывода остаток.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Заблокировано под гарантийное обеспечение фьючерсов.
    #[prost(message, repeated, tag = "3")]
    pub blocked_guarantee: ::prost::alloc::vec::Vec<MoneyValue>,
}
/// Позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioPosition {
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "2")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "3")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "4")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "5")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущий НКД.
    #[prost(message, optional, tag = "6")]
    pub current_nkd: ::core::option::Option<MoneyValue>,
    /// Deprecated Средняя цена позиции в пунктах (для фьючерсов). **Возможна задержка до секунды для пересчёта**.
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub average_position_price_pt: ::core::option::Option<Quotation>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "8")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "9")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
    /// Deprecated Количество лотов в портфеле.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub quantity_lots: ::core::option::Option<Quotation>,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "21")]
    pub blocked: bool,
    /// Количество бумаг, заблокированных выставленными заявками.
    #[prost(message, optional, tag = "22")]
    pub blocked_lots: ::core::option::Option<Quotation>,
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "24")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "25")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Вариационная маржа
    #[prost(message, optional, tag = "26")]
    pub var_margin: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "27")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualPortfolioPosition {
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "4")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "5")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "6")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "7")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "8")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
    /// Дата до которой нужно продать виртуальные бумаги, после этой даты виртуальная позиция "сгорит"
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "10")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "11")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
}
/// Баланс позиции ценной бумаги.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSecurities {
    /// Figi-идентификатор бумаги.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "11")]
    pub exchange_blocked: bool,
    /// Тип инструмента.
    #[prost(string, tag = "16")]
    pub instrument_type: ::prost::alloc::string::String,
}
/// Баланс фьючерса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFutures {
    /// Figi-идентификатор фьючерса.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Баланс опциона.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsOptions {
    /// Уникальный идентификатор позиции опциона.
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "11")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "21")]
    pub balance: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportRequest {
    #[prost(oneof = "broker_report_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_request::Payload>,
}
/// Nested message and enum types in `BrokerReportRequest`.
pub mod broker_report_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportRequest(super::GenerateBrokerReportRequest),
        #[prost(message, tag = "2")]
        GetBrokerReportRequest(super::GetBrokerReportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportResponse {
    #[prost(oneof = "broker_report_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_response::Payload>,
}
/// Nested message and enum types in `BrokerReportResponse`.
pub mod broker_report_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportResponse(super::GenerateBrokerReportResponse),
        #[prost(message, tag = "2")]
        GetBrokerReportResponse(super::GetBrokerReportResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportResponse {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportRequest {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 1), значение по умолчанию: 0.
    #[prost(int32, tag = "2")]
    pub page: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub broker_report: ::prost::alloc::vec::Vec<BrokerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReport {
    /// Номер сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Номер поручения.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Признак исполнения.
    #[prost(string, tag = "4")]
    pub execute_sign: ::prost::alloc::string::String,
    /// Дата и время заключения в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub trade_datetime: ::core::option::Option<::prost_types::Timestamp>,
    /// Торговая площадка.
    #[prost(string, tag = "6")]
    pub exchange: ::prost::alloc::string::String,
    /// Режим торгов.
    #[prost(string, tag = "7")]
    pub class_code: ::prost::alloc::string::String,
    /// Вид сделки.
    #[prost(string, tag = "8")]
    pub direction: ::prost::alloc::string::String,
    /// Сокращённое наименование актива.
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    /// Код актива.
    #[prost(string, tag = "10")]
    pub ticker: ::prost::alloc::string::String,
    /// Цена за единицу.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество.
    #[prost(int64, tag = "12")]
    pub quantity: i64,
    /// Сумма (без НКД).
    #[prost(message, optional, tag = "13")]
    pub order_amount: ::core::option::Option<MoneyValue>,
    /// НКД.
    #[prost(message, optional, tag = "14")]
    pub aci_value: ::core::option::Option<Quotation>,
    /// Сумма сделки.
    #[prost(message, optional, tag = "15")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Комиссия брокера.
    #[prost(message, optional, tag = "16")]
    pub broker_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия биржи.
    #[prost(message, optional, tag = "17")]
    pub exchange_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия клир. центра.
    #[prost(message, optional, tag = "18")]
    pub exchange_clearing_commission: ::core::option::Option<MoneyValue>,
    /// Ставка РЕПО (%).
    #[prost(message, optional, tag = "19")]
    pub repo_rate: ::core::option::Option<Quotation>,
    /// Контрагент/Брокер.
    #[prost(string, tag = "20")]
    pub party: ::prost::alloc::string::String,
    /// Дата расчётов в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub clear_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата поставки в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub sec_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус брокера.
    #[prost(string, tag = "23")]
    pub broker_status: ::prost::alloc::string::String,
    /// Тип дог.
    #[prost(string, tag = "24")]
    pub separate_agreement_type: ::prost::alloc::string::String,
    /// Номер дог.
    #[prost(string, tag = "25")]
    pub separate_agreement_number: ::prost::alloc::string::String,
    /// Дата дог.
    #[prost(string, tag = "26")]
    pub separate_agreement_date: ::prost::alloc::string::String,
    /// Тип расчёта по сделке.
    #[prost(string, tag = "27")]
    pub delivery_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerRequest {
    #[prost(
        oneof = "get_dividends_foreign_issuer_request::Payload",
        tags = "1, 2"
    )]
    pub payload:
        ::core::option::Option<get_dividends_foreign_issuer_request::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerRequest`.
pub mod get_dividends_foreign_issuer_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект запроса формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReport(
            super::GenerateDividendsForeignIssuerReportRequest,
        ),
        /// Объект запроса сформированного отчёта.
        #[prost(message, tag = "2")]
        GetDivForeignIssuerReport(
            super::GetDividendsForeignIssuerReportRequest,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerResponse {
    #[prost(
        oneof = "get_dividends_foreign_issuer_response::Payload",
        tags = "1, 2"
    )]
    pub payload: ::core::option::Option<
        get_dividends_foreign_issuer_response::Payload,
    >,
}
/// Nested message and enum types in `GetDividendsForeignIssuerResponse`.
pub mod get_dividends_foreign_issuer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата задачи запуска формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReportResponse(
            super::GenerateDividendsForeignIssuerReportResponse,
        ),
        /// Отчёт "Справка о доходах за пределами РФ".
        #[prost(message, tag = "2")]
        DivForeignIssuerReport(
            super::GetDividendsForeignIssuerReportResponse,
        ),
    }
}
/// Объект запроса формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект запроса сформированного отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportRequest {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 0), значение по умолчанию: 0.
    #[prost(int32, tag = "2")]
    pub page: i32,
}
/// Объект результата задачи запуска формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportResponse {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends_foreign_issuer_report:
        ::prost::alloc::vec::Vec<DividendsForeignIssuerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
/// Отчёт "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsForeignIssuerReport {
    /// Дата фиксации реестра.
    #[prost(message, optional, tag = "1")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата выплаты.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Наименование ценной бумаги.
    #[prost(string, tag = "3")]
    pub security_name: ::prost::alloc::string::String,
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Страна эмитента. Для депозитарных расписок указывается страна эмитента базового актива.
    #[prost(string, tag = "5")]
    pub issuer_country: ::prost::alloc::string::String,
    /// Количество ценных бумаг.
    #[prost(int64, tag = "6")]
    pub quantity: i64,
    /// Выплаты на одну бумагу
    #[prost(message, optional, tag = "7")]
    pub dividend: ::core::option::Option<Quotation>,
    /// Комиссия внешних платёжных агентов.
    #[prost(message, optional, tag = "8")]
    pub external_commission: ::core::option::Option<Quotation>,
    /// Сумма до удержания налога.
    #[prost(message, optional, tag = "9")]
    pub dividend_gross: ::core::option::Option<Quotation>,
    /// Сумма налога, удержанного агентом.
    #[prost(message, optional, tag = "10")]
    pub tax: ::core::option::Option<Quotation>,
    /// Итоговая сумма выплаты.
    #[prost(message, optional, tag = "11")]
    pub dividend_amount: ::core::option::Option<Quotation>,
    /// Валюта.
    #[prost(string, tag = "12")]
    pub currency: ::prost::alloc::string::String,
}
/// Запрос установки stream-соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по позициям и доходностям портфелей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamResponse {
    #[prost(oneof = "portfolio_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<portfolio_stream_response::Payload>,
}
/// Nested message and enum types in `PortfolioStreamResponse`.
pub mod portfolio_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PortfolioSubscriptionResult),
        /// Объект стриминга портфеля.
        #[prost(message, tag = "2")]
        Portfolio(super::PortfolioResponse),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccountSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PortfolioSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Запрос списка операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorRequest {
    /// Идентификатор счёта клиента. Обязательный параметр для данного метода, остальные параметры опциональны.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента (Figi инструмента или uid инструмента)
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "6")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "7")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор элемента, с которого начать формировать ответ.
    #[prost(string, tag = "11")]
    pub cursor: ::prost::alloc::string::String,
    /// Лимит количества операций. По умолчанию устанавливается значение **100**, максимальное значение 1000.
    #[prost(int32, tag = "12")]
    pub limit: i32,
    /// Тип операции. Принимает значение из списка OperationType.
    #[prost(enumeration = "OperationType", repeated, tag = "13")]
    pub operation_types: ::prost::alloc::vec::Vec<i32>,
    /// Статус запрашиваемых операций, возможные значения указаны в OperationState.
    #[prost(enumeration = "OperationState", tag = "14")]
    pub state: i32,
    /// Флаг возвращать ли комиссии, по умолчанию false
    #[prost(bool, tag = "15")]
    pub without_commissions: bool,
    /// Флаг получения ответа без массива сделок.
    #[prost(bool, tag = "16")]
    pub without_trades: bool,
    /// Флаг не показывать overnight операций.
    #[prost(bool, tag = "17")]
    pub without_overnights: bool,
}
/// Список операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorResponse {
    /// Признак, есть ли следующий элемент.
    #[prost(bool, tag = "1")]
    pub has_next: bool,
    /// Следующий курсор.
    #[prost(string, tag = "2")]
    pub next_cursor: ::prost::alloc::string::String,
    /// Список операций.
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<OperationItem>,
}
/// Данные об операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItem {
    /// Курсор.
    #[prost(string, tag = "1")]
    pub cursor: ::prost::alloc::string::String,
    /// Номер счета клиента.
    #[prost(string, tag = "6")]
    pub broker_account_id: ::prost::alloc::string::String,
    /// Идентификатор операции, может меняться с течением времени.
    #[prost(string, tag = "16")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции, может измениться, если изменился id родительской операции.
    #[prost(string, tag = "17")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Название операции.
    #[prost(string, tag = "18")]
    pub name: ::prost::alloc::string::String,
    /// Дата поручения.
    #[prost(message, optional, tag = "21")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "22")]
    pub r#type: i32,
    /// Описание операции.
    #[prost(string, tag = "23")]
    pub description: ::prost::alloc::string::String,
    /// Статус поручения.
    #[prost(enumeration = "OperationState", tag = "24")]
    pub state: i32,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi.
    #[prost(string, tag = "32")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "33")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "34")]
    pub instrument_kind: i32,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "41")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент.
    #[prost(message, optional, tag = "42")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Комиссия.
    #[prost(message, optional, tag = "43")]
    pub commission: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "44")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "45")]
    pub yield_relative: ::core::option::Option<Quotation>,
    /// Накопленный купонный доход.
    #[prost(message, optional, tag = "46")]
    pub accrued_int: ::core::option::Option<MoneyValue>,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "51")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "52")]
    pub quantity_rest: i64,
    /// Исполненный остаток.
    #[prost(int64, tag = "53")]
    pub quantity_done: i64,
    /// Дата и время снятия заявки.
    #[prost(message, optional, tag = "56")]
    pub cancel_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Причина отмены операции.
    #[prost(string, tag = "57")]
    pub cancel_reason: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, optional, tag = "61")]
    pub trades_info: ::core::option::Option<OperationItemTrades>,
    /// Идентификатор актива
    #[prost(string, tag = "64")]
    pub asset_uid: ::prost::alloc::string::String,
}
/// Массив с информацией о сделках.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrades {
    #[prost(message, repeated, tag = "6")]
    pub trades: ::prost::alloc::vec::Vec<OperationItemTrade>,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrade {
    /// Номер сделки
    #[prost(string, tag = "1")]
    pub num: ::prost::alloc::string::String,
    /// Дата сделки
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество в единицах.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена.
    #[prost(message, optional, tag = "16")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "21")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "22")]
    pub yield_relative: ::core::option::Option<Quotation>,
}
/// Запрос установки stream-соединения позиций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по изменению позиций портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamResponse {
    #[prost(oneof = "positions_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<positions_stream_response::Payload>,
}
/// Nested message and enum types in `PositionsStreamResponse`.
pub mod positions_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PositionsSubscriptionResult),
        /// Объект стриминга позиций.
        #[prost(message, tag = "2")]
        Position(super::PositionData),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<PositionsSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PositionsAccountSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Данные о позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionData {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub money: ::prost::alloc::vec::Vec<PositionsMoney>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "4")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
    /// Дата и время операции в формате UTC.
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Валютная позиция портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsMoney {
    /// Доступное количество валютный позиций.
    #[prost(message, optional, tag = "1")]
    pub available_value: ::core::option::Option<MoneyValue>,
    /// Заблокированное количество валютный позиций.
    #[prost(message, optional, tag = "2")]
    pub blocked_value: ::core::option::Option<MoneyValue>,
}
/// Статус запрашиваемых операций.
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
pub enum OperationState {
    /// Статус операции не определён
    Unspecified = 0,
    /// Исполнена.
    Executed = 1,
    /// Отменена.
    Canceled = 2,
    /// Исполняется.
    Progress = 3,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            OperationState::Executed => "OPERATION_STATE_EXECUTED",
            OperationState::Canceled => "OPERATION_STATE_CANCELED",
            OperationState::Progress => "OPERATION_STATE_PROGRESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATE_EXECUTED" => Some(Self::Executed),
            "OPERATION_STATE_CANCELED" => Some(Self::Canceled),
            "OPERATION_STATE_PROGRESS" => Some(Self::Progress),
            _ => None,
        }
    }
}
/// Тип операции.
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
pub enum OperationType {
    /// Тип операции не определён.
    Unspecified = 0,
    /// Пополнение брокерского счёта.
    Input = 1,
    /// Удержание НДФЛ по купонам.
    BondTax = 2,
    /// Вывод ЦБ.
    OutputSecurities = 3,
    /// Доход по сделке РЕПО овернайт.
    Overnight = 4,
    /// Удержание налога.
    Tax = 5,
    /// Полное погашение облигаций.
    BondRepaymentFull = 6,
    /// Продажа ЦБ с карты.
    SellCard = 7,
    /// Удержание налога по дивидендам.
    DividendTax = 8,
    /// Вывод денежных средств.
    Output = 9,
    /// Частичное погашение облигаций.
    BondRepayment = 10,
    /// Корректировка налога.
    TaxCorrection = 11,
    /// Удержание комиссии за обслуживание брокерского счёта.
    ServiceFee = 12,
    /// Удержание налога за материальную выгоду.
    BenefitTax = 13,
    /// Удержание комиссии за непокрытую позицию.
    MarginFee = 14,
    /// Покупка ЦБ.
    Buy = 15,
    /// Покупка ЦБ с карты.
    BuyCard = 16,
    /// Перевод ценных бумаг из другого депозитария.
    InputSecurities = 17,
    /// Продажа в результате Margin-call.
    SellMargin = 18,
    /// Удержание комиссии за операцию.
    BrokerFee = 19,
    /// Покупка в результате Margin-call.
    BuyMargin = 20,
    /// Выплата дивидендов.
    Dividend = 21,
    /// Продажа ЦБ.
    Sell = 22,
    /// Выплата купонов.
    Coupon = 23,
    /// Удержание комиссии SuccessFee.
    SuccessFee = 24,
    /// Передача дивидендного дохода.
    DividendTransfer = 25,
    /// Зачисление вариационной маржи.
    AccruingVarmargin = 26,
    /// Списание вариационной маржи.
    WritingOffVarmargin = 27,
    /// Покупка в рамках экспирации фьючерсного контракта.
    DeliveryBuy = 28,
    /// Продажа в рамках экспирации фьючерсного контракта.
    DeliverySell = 29,
    /// Комиссия за управление по счёту автоследования.
    TrackMfee = 30,
    /// Комиссия за результат по счёту автоследования.
    TrackPfee = 31,
    /// Удержание налога по ставке 15%.
    TaxProgressive = 32,
    /// Удержание налога по купонам по ставке 15%.
    BondTaxProgressive = 33,
    /// Удержание налога по дивидендам по ставке 15%.
    DividendTaxProgressive = 34,
    /// Удержание налога за материальную выгоду по ставке 15%.
    BenefitTaxProgressive = 35,
    /// Корректировка налога по ставке 15%.
    TaxCorrectionProgressive = 36,
    /// Удержание налога за возмещение по сделкам РЕПО по ставке 15%.
    TaxRepoProgressive = 37,
    /// Удержание налога за возмещение по сделкам РЕПО.
    TaxRepo = 38,
    /// Удержание налога по сделкам РЕПО.
    TaxRepoHold = 39,
    /// Возврат налога по сделкам РЕПО.
    TaxRepoRefund = 40,
    /// Удержание налога по сделкам РЕПО по ставке 15%.
    TaxRepoHoldProgressive = 41,
    /// Возврат налога по сделкам РЕПО по ставке 15%.
    TaxRepoRefundProgressive = 42,
    /// Выплата дивидендов на карту.
    DivExt = 43,
    /// Корректировка налога по купонам.
    TaxCorrectionCoupon = 44,
    /// Комиссия за валютный остаток.
    CashFee = 45,
    /// Комиссия за вывод валюты с брокерского счета.
    OutFee = 46,
    /// Гербовый сбор.
    OutStampDuty = 47,
    /// 	SWIFT-перевод
    OutputSwift = 50,
    /// 	SWIFT-перевод
    InputSwift = 51,
    ///   Перевод на карту
    OutputAcquiring = 53,
    /// 	Перевод с карты
    InputAcquiring = 54,
    /// 	Комиссия за вывод средств
    OutputPenalty = 55,
    /// 	Списание оплаты за сервис Советов
    AdviceFee = 56,
    ///   Перевод ценных бумаг с ИИС на Брокерский счет
    TransIisBs = 57,
    ///   Перевод ценных бумаг с одного брокерского счета на другой
    TransBsBs = 58,
    ///   Вывод денежных средств со счета
    OutMulti = 59,
    ///   Пополнение денежных средств со счета
    InpMulti = 60,
    ///   Размещение биржевого овернайта
    OverPlacement = 61,
    ///   Списание комиссии
    OverCom = 62,
    ///   Доход от оверанайта
    OverIncome = 63,
    /// Экспирация
    OptionExpiration = 64,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            OperationType::Input => "OPERATION_TYPE_INPUT",
            OperationType::BondTax => "OPERATION_TYPE_BOND_TAX",
            OperationType::OutputSecurities => {
                "OPERATION_TYPE_OUTPUT_SECURITIES"
            }
            OperationType::Overnight => "OPERATION_TYPE_OVERNIGHT",
            OperationType::Tax => "OPERATION_TYPE_TAX",
            OperationType::BondRepaymentFull => {
                "OPERATION_TYPE_BOND_REPAYMENT_FULL"
            }
            OperationType::SellCard => "OPERATION_TYPE_SELL_CARD",
            OperationType::DividendTax => "OPERATION_TYPE_DIVIDEND_TAX",
            OperationType::Output => "OPERATION_TYPE_OUTPUT",
            OperationType::BondRepayment => "OPERATION_TYPE_BOND_REPAYMENT",
            OperationType::TaxCorrection => "OPERATION_TYPE_TAX_CORRECTION",
            OperationType::ServiceFee => "OPERATION_TYPE_SERVICE_FEE",
            OperationType::BenefitTax => "OPERATION_TYPE_BENEFIT_TAX",
            OperationType::MarginFee => "OPERATION_TYPE_MARGIN_FEE",
            OperationType::Buy => "OPERATION_TYPE_BUY",
            OperationType::BuyCard => "OPERATION_TYPE_BUY_CARD",
            OperationType::InputSecurities => {
                "OPERATION_TYPE_INPUT_SECURITIES"
            }
            OperationType::SellMargin => "OPERATION_TYPE_SELL_MARGIN",
            OperationType::BrokerFee => "OPERATION_TYPE_BROKER_FEE",
            OperationType::BuyMargin => "OPERATION_TYPE_BUY_MARGIN",
            OperationType::Dividend => "OPERATION_TYPE_DIVIDEND",
            OperationType::Sell => "OPERATION_TYPE_SELL",
            OperationType::Coupon => "OPERATION_TYPE_COUPON",
            OperationType::SuccessFee => "OPERATION_TYPE_SUCCESS_FEE",
            OperationType::DividendTransfer => {
                "OPERATION_TYPE_DIVIDEND_TRANSFER"
            }
            OperationType::AccruingVarmargin => {
                "OPERATION_TYPE_ACCRUING_VARMARGIN"
            }
            OperationType::WritingOffVarmargin => {
                "OPERATION_TYPE_WRITING_OFF_VARMARGIN"
            }
            OperationType::DeliveryBuy => "OPERATION_TYPE_DELIVERY_BUY",
            OperationType::DeliverySell => "OPERATION_TYPE_DELIVERY_SELL",
            OperationType::TrackMfee => "OPERATION_TYPE_TRACK_MFEE",
            OperationType::TrackPfee => "OPERATION_TYPE_TRACK_PFEE",
            OperationType::TaxProgressive => "OPERATION_TYPE_TAX_PROGRESSIVE",
            OperationType::BondTaxProgressive => {
                "OPERATION_TYPE_BOND_TAX_PROGRESSIVE"
            }
            OperationType::DividendTaxProgressive => {
                "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE"
            }
            OperationType::BenefitTaxProgressive => {
                "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE"
            }
            OperationType::TaxCorrectionProgressive => {
                "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE"
            }
            OperationType::TaxRepoProgressive => {
                "OPERATION_TYPE_TAX_REPO_PROGRESSIVE"
            }
            OperationType::TaxRepo => "OPERATION_TYPE_TAX_REPO",
            OperationType::TaxRepoHold => "OPERATION_TYPE_TAX_REPO_HOLD",
            OperationType::TaxRepoRefund => "OPERATION_TYPE_TAX_REPO_REFUND",
            OperationType::TaxRepoHoldProgressive => {
                "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE"
            }
            OperationType::TaxRepoRefundProgressive => {
                "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE"
            }
            OperationType::DivExt => "OPERATION_TYPE_DIV_EXT",
            OperationType::TaxCorrectionCoupon => {
                "OPERATION_TYPE_TAX_CORRECTION_COUPON"
            }
            OperationType::CashFee => "OPERATION_TYPE_CASH_FEE",
            OperationType::OutFee => "OPERATION_TYPE_OUT_FEE",
            OperationType::OutStampDuty => "OPERATION_TYPE_OUT_STAMP_DUTY",
            OperationType::OutputSwift => "OPERATION_TYPE_OUTPUT_SWIFT",
            OperationType::InputSwift => "OPERATION_TYPE_INPUT_SWIFT",
            OperationType::OutputAcquiring => {
                "OPERATION_TYPE_OUTPUT_ACQUIRING"
            }
            OperationType::InputAcquiring => "OPERATION_TYPE_INPUT_ACQUIRING",
            OperationType::OutputPenalty => "OPERATION_TYPE_OUTPUT_PENALTY",
            OperationType::AdviceFee => "OPERATION_TYPE_ADVICE_FEE",
            OperationType::TransIisBs => "OPERATION_TYPE_TRANS_IIS_BS",
            OperationType::TransBsBs => "OPERATION_TYPE_TRANS_BS_BS",
            OperationType::OutMulti => "OPERATION_TYPE_OUT_MULTI",
            OperationType::InpMulti => "OPERATION_TYPE_INP_MULTI",
            OperationType::OverPlacement => "OPERATION_TYPE_OVER_PLACEMENT",
            OperationType::OverCom => "OPERATION_TYPE_OVER_COM",
            OperationType::OverIncome => "OPERATION_TYPE_OVER_INCOME",
            OperationType::OptionExpiration => {
                "OPERATION_TYPE_OPTION_EXPIRATION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_TYPE_INPUT" => Some(Self::Input),
            "OPERATION_TYPE_BOND_TAX" => Some(Self::BondTax),
            "OPERATION_TYPE_OUTPUT_SECURITIES" => {
                Some(Self::OutputSecurities)
            }
            "OPERATION_TYPE_OVERNIGHT" => Some(Self::Overnight),
            "OPERATION_TYPE_TAX" => Some(Self::Tax),
            "OPERATION_TYPE_BOND_REPAYMENT_FULL" => {
                Some(Self::BondRepaymentFull)
            }
            "OPERATION_TYPE_SELL_CARD" => Some(Self::SellCard),
            "OPERATION_TYPE_DIVIDEND_TAX" => Some(Self::DividendTax),
            "OPERATION_TYPE_OUTPUT" => Some(Self::Output),
            "OPERATION_TYPE_BOND_REPAYMENT" => Some(Self::BondRepayment),
            "OPERATION_TYPE_TAX_CORRECTION" => Some(Self::TaxCorrection),
            "OPERATION_TYPE_SERVICE_FEE" => Some(Self::ServiceFee),
            "OPERATION_TYPE_BENEFIT_TAX" => Some(Self::BenefitTax),
            "OPERATION_TYPE_MARGIN_FEE" => Some(Self::MarginFee),
            "OPERATION_TYPE_BUY" => Some(Self::Buy),
            "OPERATION_TYPE_BUY_CARD" => Some(Self::BuyCard),
            "OPERATION_TYPE_INPUT_SECURITIES" => Some(Self::InputSecurities),
            "OPERATION_TYPE_SELL_MARGIN" => Some(Self::SellMargin),
            "OPERATION_TYPE_BROKER_FEE" => Some(Self::BrokerFee),
            "OPERATION_TYPE_BUY_MARGIN" => Some(Self::BuyMargin),
            "OPERATION_TYPE_DIVIDEND" => Some(Self::Dividend),
            "OPERATION_TYPE_SELL" => Some(Self::Sell),
            "OPERATION_TYPE_COUPON" => Some(Self::Coupon),
            "OPERATION_TYPE_SUCCESS_FEE" => Some(Self::SuccessFee),
            "OPERATION_TYPE_DIVIDEND_TRANSFER" => {
                Some(Self::DividendTransfer)
            }
            "OPERATION_TYPE_ACCRUING_VARMARGIN" => {
                Some(Self::AccruingVarmargin)
            }
            "OPERATION_TYPE_WRITING_OFF_VARMARGIN" => {
                Some(Self::WritingOffVarmargin)
            }
            "OPERATION_TYPE_DELIVERY_BUY" => Some(Self::DeliveryBuy),
            "OPERATION_TYPE_DELIVERY_SELL" => Some(Self::DeliverySell),
            "OPERATION_TYPE_TRACK_MFEE" => Some(Self::TrackMfee),
            "OPERATION_TYPE_TRACK_PFEE" => Some(Self::TrackPfee),
            "OPERATION_TYPE_TAX_PROGRESSIVE" => Some(Self::TaxProgressive),
            "OPERATION_TYPE_BOND_TAX_PROGRESSIVE" => {
                Some(Self::BondTaxProgressive)
            }
            "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE" => {
                Some(Self::DividendTaxProgressive)
            }
            "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE" => {
                Some(Self::BenefitTaxProgressive)
            }
            "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE" => {
                Some(Self::TaxCorrectionProgressive)
            }
            "OPERATION_TYPE_TAX_REPO_PROGRESSIVE" => {
                Some(Self::TaxRepoProgressive)
            }
            "OPERATION_TYPE_TAX_REPO" => Some(Self::TaxRepo),
            "OPERATION_TYPE_TAX_REPO_HOLD" => Some(Self::TaxRepoHold),
            "OPERATION_TYPE_TAX_REPO_REFUND" => Some(Self::TaxRepoRefund),
            "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE" => {
                Some(Self::TaxRepoHoldProgressive)
            }
            "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE" => {
                Some(Self::TaxRepoRefundProgressive)
            }
            "OPERATION_TYPE_DIV_EXT" => Some(Self::DivExt),
            "OPERATION_TYPE_TAX_CORRECTION_COUPON" => {
                Some(Self::TaxCorrectionCoupon)
            }
            "OPERATION_TYPE_CASH_FEE" => Some(Self::CashFee),
            "OPERATION_TYPE_OUT_FEE" => Some(Self::OutFee),
            "OPERATION_TYPE_OUT_STAMP_DUTY" => Some(Self::OutStampDuty),
            "OPERATION_TYPE_OUTPUT_SWIFT" => Some(Self::OutputSwift),
            "OPERATION_TYPE_INPUT_SWIFT" => Some(Self::InputSwift),
            "OPERATION_TYPE_OUTPUT_ACQUIRING" => Some(Self::OutputAcquiring),
            "OPERATION_TYPE_INPUT_ACQUIRING" => Some(Self::InputAcquiring),
            "OPERATION_TYPE_OUTPUT_PENALTY" => Some(Self::OutputPenalty),
            "OPERATION_TYPE_ADVICE_FEE" => Some(Self::AdviceFee),
            "OPERATION_TYPE_TRANS_IIS_BS" => Some(Self::TransIisBs),
            "OPERATION_TYPE_TRANS_BS_BS" => Some(Self::TransBsBs),
            "OPERATION_TYPE_OUT_MULTI" => Some(Self::OutMulti),
            "OPERATION_TYPE_INP_MULTI" => Some(Self::InpMulti),
            "OPERATION_TYPE_OVER_PLACEMENT" => Some(Self::OverPlacement),
            "OPERATION_TYPE_OVER_COM" => Some(Self::OverCom),
            "OPERATION_TYPE_OVER_INCOME" => Some(Self::OverIncome),
            "OPERATION_TYPE_OPTION_EXPIRATION" => {
                Some(Self::OptionExpiration)
            }
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
pub enum PortfolioSubscriptionStatus {
    /// Тип не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Счёт не найден или недостаточно прав.
    AccountNotFound = 2,
    /// Произошла ошибка.
    InternalError = 3,
}
impl PortfolioSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PortfolioSubscriptionStatus::Unspecified => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            PortfolioSubscriptionStatus::Success => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS"
            }
            PortfolioSubscriptionStatus::AccountNotFound => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PortfolioSubscriptionStatus::InternalError => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED" => {
                Some(Self::Unspecified)
            }
            "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => {
                Some(Self::AccountNotFound)
            }
            "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => {
                Some(Self::InternalError)
            }
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
pub enum PositionsAccountSubscriptionStatus {
    /// Тип не определён.
    PositionsSubscriptionStatusUnspecified = 0,
    /// Успешно.
    PositionsSubscriptionStatusSuccess = 1,
    /// Счёт не найден или недостаточно прав.
    PositionsSubscriptionStatusAccountNotFound = 2,
    /// Произошла ошибка.
    PositionsSubscriptionStatusInternalError = 3,
}
impl PositionsAccountSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusUnspecified => {
                "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusSuccess => {
                "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusAccountNotFound => {
                "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusInternalError => {
                "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED" => {
                Some(Self::PositionsSubscriptionStatusUnspecified)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS" => {
                Some(Self::PositionsSubscriptionStatusSuccess)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => {
                Some(Self::PositionsSubscriptionStatusAccountNotFound)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => {
                Some(Self::PositionsSubscriptionStatusInternalError)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod operations_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsServiceClient<tonic::transport::Channel> {
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
    impl<T> OperationsServiceClient<T>
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
        ) -> OperationsServiceClient<InterceptedService<T, F>>
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
            OperationsServiceClient::new(InterceptedService::new(
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
        /// Метод получения списка операций по счёту.При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения портфеля по счёту.
        pub async fn get_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPortfolio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка позиций по счёту.
        pub async fn get_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения доступного остатка для вывода средств.
        pub async fn get_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<
            tonic::Response<super::WithdrawLimitsResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetWithdrawLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения брокерского отчёта.
        pub async fn get_broker_report(
            &mut self,
            request: impl tonic::IntoRequest<super::BrokerReportRequest>,
        ) -> Result<tonic::Response<super::BrokerReportResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetBrokerReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения отчёта "Справка о доходах за пределами РФ".
        pub async fn get_dividends_foreign_issuer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetDividendsForeignIssuerRequest,
            >,
        ) -> Result<
            tonic::Response<super::GetDividendsForeignIssuerResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetDividendsForeignIssuer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка операций по счёту с пагинацией. При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> Result<
            tonic::Response<super::GetOperationsByCursorResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperationsByCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod operations_stream_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsStreamServiceClient<tonic::transport::Channel> {
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
    impl<T> OperationsStreamServiceClient<T>
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
        ) -> OperationsStreamServiceClient<InterceptedService<T, F>>
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
            OperationsStreamServiceClient::new(InterceptedService::new(
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
        /// Server-side stream обновлений портфеля
        pub async fn portfolio_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::PortfolioStreamResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PortfolioStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// Server-side stream обновлений информации по изменению позиций портфеля
        pub async fn positions_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::PositionsStreamResponse>,
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
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PositionsStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
