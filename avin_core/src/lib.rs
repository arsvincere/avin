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
//! `avin_strategy`, `avin_tester`, `avin_trader`, `avin_terminal`.
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
//!    добавлен [`Indicator`], а так же датафрейм [`UserData`] с любыми
//!    пользовательскими расчетами по этому графику (если нужно сделать
//!    какой-то свой не стандартный индикатор). Тики внутри ассета
//!    группируются по таймфреймам в [`Footprint`] и над ними рассчитываются
//!    разные метрики [`Cluster`].
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
//! 4. Подмодуль [`data`] отвечает за загрузку рыночных данных, которые
//!    загружены утилитой avin-data.
//!
//! ## Getting start.

mod action;
mod asset;
mod broker;
mod chart;
mod data;
mod enums;
mod event;
mod footprint;
mod indicator;
mod operation;
mod order;
mod trade;

pub use action::{Action, DataAction, OrderAction};
pub use asset::{Asset, AssetList, Share};
pub use broker::Account;
pub use chart::{Bar, Chart, Range, UserData};
pub use data::{Category, Exchange, Iid, Manager, MarketData, Source};
pub use enums::{Direction, Kind, TimeFrame};
pub use event::{BarEvent, Event, OrderEvent, TicEvent};
pub use footprint::{Cluster, Footprint, Quant, Quantum, Tic};
pub use indicator::Indicator;
pub use operation::{Operation, Transaction};
pub use order::{LimitOrder, MarketOrder, Order, StopOrder};
pub use trade::{Summary, Trade, TradeKind, TradeList};

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

// extrumum indicator
pub use indicator::{Extremum, ExtremumIndicator, ExtremumKind, Term, Trend};
