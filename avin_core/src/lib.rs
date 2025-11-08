/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

//! # AVIN  -  Ars Vincere
//!
//! Core structures for crate `avin`.
//!
//! # ru
//! Основные трейдерские объекты: бар, график, таймфрейм, ордер и тп.
//! Используются всеми остальными частями системы (крейтами):
//! - `avin_data`,
//! - `avin_strategy`,
//! - `avin_tester`,
//! - `avin_trader`,
//! - `avin_terminal`,
//!
//! Модуль импортирован в главный крейт `avin`, используйте его для работы.
//!
//! ## Архитектура (Overview).
//!
//! Центральные понятия модуля:
//!
//! 1. [`AssetList`] - список активов. Состоит из списка активов - [`Asset`].
//!    Внутри актива хранится идентификатор инструмента [`Iid`], графики
//!    [`Chart`] разных таймфреймов, тики [`Tic`]. График содержит
//!    [`TimeFrame`], список [`Bar`] и копию [`Iid`]. К графику может быть
//!    добавлен [`Indicator`]. Тики внутри ассета группируются по таймфреймам
//!    в [`Footprint`] и над ними рассчитываются разные метрики [`Cluster`].
//! 2. [`TradeList`] - список трейдов, то что получается после отработки
//!    тестера, или в боевом режиме. Он состоит из названия и списка
//!    [`Trade`]. Когда на счете открывается позиция, открывается трейд.
//!    Дальше все операции [`Operation`] связанные с этой позицией и
//!    ордера [`Order`] привязываются к этому трейду. Когда позиция
//!    закрывается, закрывается трейд. Трейд подсчитывает суммарные значения
//!    по всем операциям. Операция состоит из отдельных сделок
//!    [`Transaction`], потому что на стороне биржи ордер может исполниться
//!    несколькими сделками.
//! 3. Брокер содержит счета [`Account`] на которых непосредственно
//!    совершаются все операции. Брокер крутится в отдельном потоке, и
//!    отправляет через канал события [`Event`]. Пример события: новый бар,
//!    новый тик, ордер исполнен, ордер выставлен... События в боевом режиме
//!    принимает трейдер (см. модуль avin_trader) или тестер в режиме
//!    бэктеста (см. модуль avin_tester). И передает их стратегии (см. модуль
//!    avin_strategy). Стратегия создает действия [`Action`], например:
//!    выставить ордер, отменить ордер, подписаться на бары по инструменту...
//!    И отправляет их через канал тестеру/трейдеру который уже по очереди
//!    их передает брокеру (или VirtualBroker в режиме тестера).

mod action;
mod asset;
mod broker;
mod chart;
mod data;
mod event;
mod footprint;
mod indicator;
mod operation;
mod order;
mod trade;

pub use action::{
    Action, GetAccountAction, GetBarsAction, OrderAction, StreamAction,
};
pub use asset::{Asset, AssetList, Category, Exchange, Iid, Share};
pub use broker::Account;
pub use chart::{Bar, Chart, Range, TimeFrame};
pub use data::{DataSchema, Manager, MarketData, Source};
pub use event::{BarEvent, Event, OrderEvent, TicEvent};
pub use footprint::{Cluster, Footprint, Quant, Quantum, Tic};
pub use operation::{Operation, Transaction};
pub use trade::{Summary, Trade, TradeKind, TradeList};

// order
pub use order::{Direction, LimitOrder, MarketOrder, Order, StopOrder};
// market order statuses
pub use order::{
    FilledMarketOrder, NewMarketOrder, PostedMarketOrder, RejectedMarketOrder,
};
// limit order statuses
pub use order::{
    CanceledLimitOrder, FilledLimitOrder, NewLimitOrder, PostedLimitOrder,
    RejectedLimitOrder,
};
// stop order statuses
pub use order::{
    CanceledStopOrder, NewStopOrder, PostedStopOrder, RejectedStopOrder,
    StopOrderKind, TriggeredStopOrder,
};

// indicator
pub use indicator::Indicator;
// extrumum indicator
pub use indicator::{Extremum, ExtremumIndicator, ExtremumKind, Term, Trend};
