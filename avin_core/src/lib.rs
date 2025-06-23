/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

//! # AVIN  -  Ars Vincere
//!
//! Core structures for crate `avin`
//!
//! # ru
//! Основные трейдерские объекты: бар, график, таймфрейм, ордер и тп.
//! Используются всеми остальными частями системы (крейтами):
//! `avin_strategy`, `avin_tester`, `avin_trader`, `avin_terminal`.
//!
//! Все структуры импортированы в главный крейт `avin`,
//! используйте его для работы.

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
