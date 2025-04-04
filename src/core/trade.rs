/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::asset::Asset;
use crate::core::direction::Direction;
use crate::core::order::{Order, PostedStopOrder};
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Trade {
    New(NewTrade),
    // Posted(PostedTrade),
    Opened(OpenedTrade),
    Closed(ClosedTrade),
}
impl Trade {
    pub fn new(
        dt: &DateTime<Utc>,
        strategy: &str,
        typ: TradeType,
        asset: &Asset,
    ) -> NewTrade {
        NewTrade {
            dt: dt.clone(),
            strategy: strategy.to_string(),
            typ,
            asset: asset.copy_id(),
            info: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradeType {
    Long,
    Short,
}

#[derive(Debug)]
pub struct NewTrade {
    pub dt: DateTime<Utc>,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
    pub info: HashMap<String, String>,
}
impl NewTrade {
    // pub fn post(self, posted_order: Order) -> PostedTrade {
    //     if !posted_order.is_posted() {
    //         panic!("order shoud be posted")
    //     }
    //     PostedTrade {
    //         dt: self.dt,
    //         strategy: self.strategy,
    //         typ: self.typ,
    //         asset: self.asset,
    //         info: self.info,
    //         orders: vec![posted_order],
    //     }
    // }
    pub fn open(self, filled_order: Order) -> OpenedTrade {
        if !filled_order.is_filled() {
            panic!("order shoud be filled")
        }
        OpenedTrade {
            dt: self.dt,
            strategy: self.strategy,
            typ: self.typ,
            asset: self.asset,
            info: self.info,
            orders: vec![filled_order],

            stop_loss: None,
            take_profit: None,
        }
    }
}

// #[derive(Debug)]
// pub struct PostedTrade {
//     pub dt: DateTime<Utc>,
//     pub strategy: String,
//     pub typ: TradeType,
//     pub asset: Asset,
//     pub info: HashMap<String, String>,
//     pub orders: Vec<Order>,
// }
// impl PostedTrade {
//     pub fn add_order(&mut self, order: Order) {
//         self.orders.push(order)
//     }
//     pub fn open(self) -> OpenedTrade {
//         // TODO: проверка что есть хотя бы один исполненный ордер
//         OpenedTrade {
//             dt: self.dt,
//             strategy: self.strategy,
//             typ: self.typ,
//             asset: self.asset,
//             info: self.info,
//             orders: self.orders,
//
//             stop_loss: None,
//             take_profit: None,
//         }
//     }
// }

#[derive(Debug)]
pub struct OpenedTrade {
    pub dt: DateTime<Utc>,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
    pub info: HashMap<String, String>,
    pub orders: Vec<Order>,

    pub stop_loss: Option<PostedStopOrder>,
    pub take_profit: Option<PostedStopOrder>,
}
impl OpenedTrade {
    pub fn add_order(&mut self, filled_order: Order) {
        if !filled_order.is_filled() {
            panic!("order shoud be filled")
        }

        self.orders.push(filled_order)
    }
    pub fn set_stop(&mut self, stop_order: PostedStopOrder) {
        self.stop_loss = Some(stop_order);
    }
    pub fn set_take(&mut self, stop_order: PostedStopOrder) {
        self.take_profit = Some(stop_order);
    }
    pub fn close(self) -> ClosedTrade {
        let trade = ClosedTrade {
            dt: self.dt,
            strategy: self.strategy,
            typ: self.typ,
            asset: self.asset,
            info: self.info,
            orders: self.orders,
        };

        // INFO: проверка что трейд действительно закрыт
        // количество активов в позиции = 0
        if trade.quantity() != 0 {
            panic!("in closed trade quantity != 0");
        }
        trade
    }
}

#[derive(Debug)]
pub struct ClosedTrade {
    pub dt: DateTime<Utc>,
    pub strategy: String,
    pub typ: TradeType,
    pub asset: Asset,
    pub info: HashMap<String, String>,
    pub orders: Vec<Order>,
}
impl ClosedTrade {
    pub fn is_long(&self) -> bool {
        self.typ == TradeType::Long
    }
    pub fn is_short(&self) -> bool {
        self.typ == TradeType::Short
    }
    pub fn is_win(&self) -> bool {
        todo!();
    }
    pub fn is_loss(&self) -> bool {
        todo!();
    }

    pub fn lots(&self) -> i32 {
        todo!();
    }

    pub fn quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.quantity
            } else {
                total -= op.quantity
            }
        }

        total
    }
    pub fn buy_quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.quantity
            }
        }

        total
    }
    pub fn sell_quantity(&self) -> i32 {
        let mut total: i32 = 0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.quantity
            }
        }

        total
    }

    pub fn value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.value
            } else {
                total -= op.value
            }
        }

        total
    }
    pub fn buy_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.value
            }
        }

        total
    }
    pub fn sell_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.value
            }
        }

        total
    }

    pub fn commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            total += op.commission
        }

        total
    }
    pub fn buy_commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Buy {
                total += op.commission
            }
        }

        total
    }
    pub fn sell_commission(&self) -> f64 {
        let mut total: f64 = 0.0;

        for order in self.orders.iter() {
            let op = match order.operation() {
                Some(op) => op,
                None => panic!("in closed trade all orders must be filled"),
            };
            if *order.direction() == Direction::Sell {
                total += op.commission
            }
        }

        total
    }

    pub fn buy_avg(&self) -> f64 {
        self.buy_value() / self.buy_quantity() as f64
    }
    pub fn sell_avg(&self) -> f64 {
        self.sell_value() / self.sell_quantity() as f64
    }

    pub fn open_dt(&self) -> DateTime<Utc> {
        let o = self.orders.first().unwrap();
        match o.transactions() {
            Some(transactions) => transactions.first().unwrap().dt,
            None => panic!("closed trade without transactions in order"),
        }
    }
    pub fn close_dt(&self) -> DateTime<Utc> {
        let o = self.orders.last().unwrap();
        match o.transactions() {
            Some(transactions) => transactions.last().unwrap().dt,
            None => panic!("closed trade without transactions in order"),
        }
    }
    pub fn timedelta(&self) -> TimeDelta {
        self.close_dt() - self.open_dt()
    }
    pub fn result(&self) -> f64 {
        self.sell_value() - self.buy_value() - self.commission()
    }
    pub fn result_p(&self) -> f64 {
        self.result() / self.buy_value() * 100.0
    }
    pub fn speed(&self) -> f64 {
        // INFO: если таймдельту перевести сразу в дни то
        // для трейдов короче одного дня там будет 0.
        // поэтому смотрю на количество минут трейда, делю на 60 и 24
        // получается например 600 / 60 / 24 = 0.42 дня.
        // Беру результат трейда в процентах и делю на это число
        // в итоге получается количество рублей в день
        // используется для сравнения эффективности трейдов с учетом
        // времени которое деньги были заняты в этом трейде.
        self.result() / (self.timedelta().num_minutes() as f64 / 60.0 / 24.0)
    }
    pub fn speed_p(&self) -> f64 {
        // INFO: если таймдельту перевести сразу в дни то
        // для трейдов короче одного дня там будет 0.
        // поэтому смотрю на количество минут трейда, делю на 60 и 24
        // получается например 600 / 60 / 24 = 0.42 дня.
        // Беру результат трейда в процентах и делю на это число
        // в итоге получается количество процентов в день
        // используется для сравнения эффективности трейдов с учетом
        // времени которое деньги были заняты в этом трейде.
        self.result_p()
            / (self.timedelta().num_minutes() as f64 / 60.0 / 24.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn statuses() {
        // create trade
        let asset = Asset::from("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let trade =
            Trade::new(&dt, "Trend T3 Posterior v1", TradeType::Long, &asset);
        assert_eq!(trade.dt, dt);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.asset.ticker, "SBER");
        assert_eq!(trade.info.len(), 0);

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 5, 14, 57, 0).unwrap(),
            100,
            301.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        let mut trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // add second filled order
        let order = LimitOrder::new(Direction::Sell, 10, 311.0);
        let mut order = order.post("broker_id=100501");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 6, 14, 57, 0).unwrap(),
            100,
            311.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        trade.add_order(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 2);

        // close trade
        let trade = trade.close();
        assert_eq!(trade.result(), 994.0);
        assert!(trade.result_p() > 3.3);
        assert_eq!(trade.timedelta().num_seconds(), 86400); // сутки
        assert!(trade.speed() > 990.0);
        assert!(trade.speed_p() > 3.3);
    }
    #[test]
    #[should_panic]
    fn close_unclosed_trade() {
        // create trade
        let asset = Asset::from("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let trade =
            Trade::new(&dt, "Trend T3 Posterior v1", TradeType::Long, &asset);
        assert_eq!(trade.dt, dt);
        assert_eq!(trade.strategy, "Trend T3 Posterior v1");
        assert_eq!(trade.asset.ticker, "SBER");
        assert_eq!(trade.info.len(), 0);

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 5, 14, 57, 0).unwrap(),
            100,
            301.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        let trade = trade.open(Order::Limit(LimitOrder::Filled(order)));
        assert_eq!(trade.orders.len(), 1);

        // try close opened trade - should_panic
        let _ = trade.close();
    }
}

// class Trade:  # {{{
//     class Status(enum.Enum):  # {{{
//         INITIAL = enum.auto()
//         PENDING = enum.auto()
//         TRIGGERED = enum.auto()
//
//         MAKE_ORDER = enum.auto()
//         POST_ORDER = enum.auto()
//         AWAIT_EXEC = enum.auto()
//         OPENED = enum.auto()
//
//         MAKE_STOP = enum.auto()
//         MAKE_TAKE = enum.auto()
//         POST_STOP = enum.auto()
//         POST_TAKE = enum.auto()
//
//         ACTIVE = enum.auto()
//         CLOSING = enum.auto()
//
//         CLOSED = enum.auto()
//         CANCELED = enum.auto()
//         BLOCKED = enum.auto()
//
//         @classmethod  # fromStr
//         def fromStr(cls, string: str) -> Trade.Status:
//             statuses = {
//                 "INITIAL": Trade.Status.INITIAL,
//                 "PENDING": Trade.Status.PENDING,
//                 "TRIGGERED": Trade.Status.TRIGGERED,
//                 "MAKE_ORDER": Trade.Status.MAKE_ORDER,
//                 "POST_ORDER": Trade.Status.POST_ORDER,
//                 "AWAIT_EXEC": Trade.Status.AWAIT_EXEC,
//                 "OPENED": Trade.Status.OPENED,
//                 "MAKE_STOP": Trade.Status.MAKE_STOP,
//                 "MAKE_TAKE": Trade.Status.MAKE_TAKE,
//                 "POST_STOP": Trade.Status.POST_STOP,
//                 "POST_TAKE": Trade.Status.POST_TAKE,
//                 "ACTIVE": Trade.Status.ACTIVE,
//                 "CLOSING": Trade.Status.CLOSING,
//                 "CLOSED": Trade.Status.CLOSED,
//                 "CANCELED": Trade.Status.CANCELED,
//                 "BLOCKED": Trade.Status.BLOCKED,
//             }
//             return statuses[string]
//
//     # }}}
//
//     def __init__(  # {{{
//         self,
//         dt: DateTime,
//         strategy: str,
//         version: str,
//         trade_type: Trate.Type,
//         instrument: Instrument,
//         status: Trade.Status = Status.INITIAL,
//         trade_id: Optional[Id] = None,
//         trade_list_name: Optional[str] = "",
//         orders: Optional[list] = None,
//         operations: Optional[list] = None,
//         info: Optional[dict] = None,
//     ):
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//
//         self.dt = dt
//         self.strategy = strategy
//         self.version = version
//         self.type = trade_type
//         self.instrument = instrument
//         self.status = status
//         self.trade_id = trade_id
//         self.trade_list_name = trade_list_name
//         self.orders = orders if orders else list()
//         self.operations = operations if operations else list()
//         self.info = info if info else dict()
//
//         # signals
//         self.opened = AsyncSignal(object)
//         self.closed = AsyncSignal(object)
//         self.statusChanged = AsyncSignal(object)
//
//         # connect order signals
//         if self.orders:
//             for order in self.orders:
//                 self.__connectOrderSignals(order)
//
//     # }}}
//     def __str__(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__str__()")
//
//         dt = self.dt + Usr.TIME_DIF
//         dt = dt.strftime("%Y-%m-%d %H:%M")
//         string = (
//             f"Trade="
//             f"{dt} [{self.status.name}] {self.strategy}-{self.version} "
//             f"{self.instrument.ticker}"
//         )
//         return string
//
//     # }}}
//     def pretty(self) -> str:  # {{{
//         logger.debug(f"{self.__class__.__name__}.pretty()")
//
//         trade_text = f"""
// id:         {self.trade_id}
// dt:         {Usr.localTime(self.dt)}
// strategy:   {self.strategy}
// version:    {self.version}
// type:       {self.type.name}
// instrument: {self.instrument}
// status:     {self.status.name}
// trade_list: {self.trade_list_name}
// """
//         if self.status == Trade.Status.CLOSED:
//             trade_text += f"""
// ------------------------------------------------------------------------------
// buy:        {self.buyAverage()} * {self.buyQuantity()} = {self.buyAmount()}
// sell:       {self.sellAverage()} * {self.sellQuantity()} = {self.sellAmount()}
// commission: {self.commission()}
// open_dt:    {Usr.localTime(self.openDateTime())}
// close_dt:   {Usr.localTime(self.closeDateTime())}
// open:       {self.openPrice()}
// stop:       {self.stopPrice()} / {self.stopAbs()} / {self.stopPercent()}%
// take:       {self.takePrice()} / {self.takeAbs()} / {self.takePercent()}%
// ------------------------------------------------------------------------------
// result:     {self.result()}
// days:       {self.holdingDays()}
// percent:    {self.percent()}
// ppd:        {self.percentPerDay()}
// info:       {Cmd.toJson(self.info, indent=4)}
// """
//
//         orders_text = ""
//         for order in self.orders:
//             text = order.pretty()
//             orders_text += text
//
//         operations_text = ""
//         for operation in self.operations:
//             text = operation.pretty()
//             operations_text += text
//
//         trade_text += f"""
// == Orders ====================================================================
// {orders_text}
// == Operations ================================================================
// {operations_text}
// """
//         return trade_text
//
//     # }}}
//
//     # @async_slot  #onOrderPosted # {{{
//     async def onOrderPosted(self, order):
//         assert order.trade_id == self.trade_id
//
//         if self.status.value < Trade.Status.AWAIT_EXEC.value:
//             await self.setStatus(Trade.Status.AWAIT_EXEC)
//
//         # otherwise trade already open, and this order is stop/take
//         # or another. Essence - trade already open - do nothing with status
//         pass
//
//     # }}}
//     # @async_slot  #onOrderExecuted # {{{
//     async def onOrderExecuted(self, order, operation):
//         assert order.trade_id == self.trade_id
//         await self.attachOperation(operation)
//
//         if self.status.value < Trade.Status.OPENED.value:
//             await self.setStatus(Trade.Status.OPENED)
//
//     # }}}
//     async def setStatus(self, status: Trade.Status):  # {{{
//         logger.debug(f"{self.__class__.__name__}.setStatus()")
//
//         self.status = status
//         await Trade.update(self)
//
//         # emiting special signal for this status
//         if status == Trade.Status.OPENED:
//             await self.opened.aemit(self)
//         elif status == Trade.Status.CLOSED:
//             await self.closed.aemit(self)
//
//         # emiting common signal
//         await self.statusChanged.aemit(self)
//
//     # }}}
//     async def attachOrder(self, order: Order):  # {{{
//         logger.debug(f"{self.__class__.__name__}.attachOrder()")
//
//         await order.setParentTrade(self)
//         self.__connectOrderSignals(order)
//         self.orders.append(order)
//
//     # }}}
//     async def attachOperation(self, operation: Operation):  # {{{
//         logger.debug(f"{self.__class__.__name__}.attachOperation()")
//
//         await operation.setParentTrade(self)
//         self.operations.append(operation)
//
//         # check lots count & update status
//         if self.lots() == 0:
//             await self.setStatus(Trade.Status.CLOSED)
//
//     # }}}
//     async def loadChart(  # {{{
//         self, timeframe: TimeFrame | str, n=None
//     ) -> Chart:
//         logger.debug(f"{self.__class__.__name__}.chart()")
//         assert self.instrument.type == Instrument.Type.SHARE
//
//         if isinstance(timeframe, str):
//             timeframe = TimeFrame(timeframe)
//
//         if n is None:
//             n = 5000  # default bars count
//
//         end = self.dt
//         begin = self.dt - n * timeframe
//
//         chart = await Chart.load(self.instrument, timeframe, begin, end)
//         chart.setHeadDatetime(self.dt)
//         return chart
//
//     # }}}
//
//     def isLong(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.isLong()")
//
//         return self.type == Trade.Type.LONG
//
//     # }}}
//     def isShort(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.isShort()")
//
//         return self.type == Trade.Type.SHORT
//
//     # }}}
//     def isWin(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.isWin()")
//
//         assert self.status == Trade.Status.CLOSED
//         return self.result() > 0
//
//     # }}}
//     def isLoss(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.isLoss()")
//
//         assert self.status == Trade.Status.CLOSED
//         return self.result() <= 0
//
//     # }}}
//     def lots(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.lots()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.lots
//             elif op.direction == Direction.SELL:
//                 total -= op.lots
//         return total
//
//     # }}}
//     def quantity(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.quantity()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.quantity
//             elif op.direction == Direction.SELL:
//                 total -= op.quantity
//         return total
//
//     # }}}
//     def buyQuantity(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.buyQuantity()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.quantity
//         return total
//
//     # }}}
//     def sellQuantity(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.sellQuantity()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.SELL:
//                 total += op.quantity
//         return total
//
//     # }}}
//     def amount(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.amount()")
//
//         if self.status == Trade.Status.CLOSED:
//             return 0.0
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.amount
//             elif op.direction == Direction.SELL:
//                 total -= op.amount
//         return total
//
//     # }}}
//     def buyAmount(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.buyAmount()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.amount
//         return total
//
//     # }}}
//     def sellAmount(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.sellAmount()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.SELL:
//                 total += op.amount
//         return total
//
//     # }}}
//     def commission(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.commission()")
//
//         return self.buyCommission() + self.sellCommission()
//
//     # }}}
//     def buyCommission(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.buyCommission()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.BUY:
//                 total += op.commission
//         return total
//
//     # }}}
//     def sellCommission(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.sellCommission()")
//
//         total = 0
//         for op in self.operations:
//             if op.direction == Direction.SELL:
//                 total += op.commission
//         return total
//
//     # }}}
//     def average(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.average()")
//
//         if self.quantity() == 0:
//             return 0.0
//         return self.amount() / self.quantity()
//
//     # }}}
//     def buyAverage(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.buyAverage()")
//
//         if self.buyQuantity() == 0:
//             return 0.0
//         return self.buyAmount() / self.buyQuantity()
//
//     # }}}
//     def sellAverage(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.sellAverage()")
//
//         if self.sellQuantity() == 0:
//             return 0.0
//         return self.sellAmount() / self.sellQuantity()
//
//     # }}}
//     def openDateTime(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.openDateTime()")
//
//         assert self.status.value >= Trade.Status.OPENED.value
//         return self.operations[0].dt
//
//     # }}}
//     def openPrice(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.openPrice()")
//
//         assert self.status.value >= Trade.Status.OPENED.value
//         assert self.status.value != Trade.Status.CANCELED.value
//
//         if self.isLong():
//             return self.buyAverage()
//
//         return self.sellAverage()
//
//     # }}}
//     def closeDateTime(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.closeDateTime()")
//         assert self.status == Trade.Status.CLOSED
//
//         # FIX:
//         # возможно из БД они в произвольном порядке загрузятся...
//         # надо проверить, добавить сортировку по дате при загрузке
//
//         return self.operations[-1].dt
//
//     # }}}
//     def closePrice(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.closePrice()")
//         assert self.status == Trade.Status.CLOSED
//
//         if self.isLong():
//             return self.sellAverage()
//
//         return self.buyAverage()
//
//     # }}}
//     def stopLoss(self) -> StopLoss | None:  # {{{
//         for order in self.orders:
//             if order.type == Order.Type.STOP_LOSS:
//                 return order
//
//         return None
//
//     # }}}
//     def takeProfit(self) -> TakeProfit | None:  # {{{
//         for order in self.orders:
//             if order.type == Order.Type.TAKE_PROFIT:
//                 return order
//
//         return None
//
//     # }}}
//     def stopPrice(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.stopPrice()")
//
//         for order in self.orders:
//             if order.type == Order.Type.STOP_LOSS:
//                 return order.stop_price
//
//         return None
//
//     # }}}
//     def takePrice(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.takePrice()")
//
//         for order in self.orders:
//             if order.type == Order.Type.TAKE_PROFIT:
//                 return order.stop_price
//
//         return None
//
//     # }}}
//     def stopAbs(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.stopAbs()")
//
//         open_price = self.openPrice()
//         stop_price = self.stopPrice()
//         if stop_price is None:
//             return None
//
//         risk = abs(stop_price - open_price)
//         return round(risk, 2)
//
//     # }}}
//     def takeAbs(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.takeAbs()")
//
//         open_price = self.openPrice()
//         take_price = self.takePrice()
//         if take_price is None:
//             return None
//
//         profit = abs(take_price - open_price)
//         return round(profit, 2)
//
//     # }}}
//     def stopPercent(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.stopPercent()")
//
//         open_price = self.openPrice()
//         stop_price = self.stopPrice()
//         if stop_price is None:
//             return None
//
//         if self.type == Trade.Type.LONG:
//             stop_range = Range(stop_price, open_price)
//         else:
//             stop_range = Range(open_price, stop_price)
//
//         percent = stop_range.percent()
//         return percent
//
//     # }}}
//     def takePercent(self) -> float | None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.takePercent()")
//
//         open_price = self.openPrice()
//         take_price = self.takePrice()
//         if take_price is None:
//             return None
//
//         if self.type == Trade.Type.LONG:
//             take_range = Range(open_price, take_price)
//         else:
//             take_range = Range(take_price, open_price)
//
//         percent = take_range.percent()
//         return percent
//
//     # }}}
//     def result(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.result()")
//         assert self.status == Trade.Status.CLOSED
//
//         result = self.sellAmount() - self.buyAmount() - self.commission()
//         return round(result, 2)
//
//     # }}}
//     def holdingDays(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.holdingDays()")
//
//         opn_dt = self.operations[0].dt
//         cls_dt = self.operations[-1].dt
//         holding = cls_dt - opn_dt + ONE_DAY
//         return holding.days
//
//     # }}}
//     def percent(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.percent()")
//         assert self.status == Trade.Status.CLOSED
//
//         persent = self.result() / self.buyAmount() * 100
//         return round(persent, 2)
//
//     # }}}
//     def percentPerDay(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.percentPerDay()")
//         assert self.status == Trade.Status.CLOSED
//
//         persent = self.result() / self.buyAmount() * 100
//         holding = self.holdingDays()
//         persent_per_day = persent / holding
//         return round(persent_per_day, 2)
//
//     # }}}
//
//     @classmethod  # fromRecord{{{
//     async def fromRecord(cls, record):
//         logger.debug(f"{cls.__name__}.fromRecord()")
//
//         trade_id = record["trade_id"]
//
//         # request operations of trade
//         operations = await Keeper.get(
//             Operation,
//             trade_id=trade_id,
//         )
//
//         # request orders of trade
//         orders = await Keeper.get(
//             Order,
//             trade_id=trade_id,
//         )
//
//         # request instrument
//         instrument = await Instrument.fromFigi(record["figi"])
//
//         # create trade
//         trade = Trade(
//             dt=record["dt"],
//             strategy=record["strategy"],
//             version=record["version"],
//             trade_type=Trade.Type.fromStr(record["trade_type"]),
//             instrument=instrument,
//             status=Trade.Status.fromStr(record["status"]),
//             trade_id=Id.fromStr(record["trade_id"]),
//             trade_list_name=record["trade_list"],
//             orders=orders,
//             operations=operations,
//         )
//
//         # create info
//         trade.info = Cmd.fromJson(record["trade_info"], Trade.decoderJson)
//
//         # connect signals of attached orders
//         for order in trade.orders:
//             trade.__connectOrderSignals(order)
//
//         return trade
//
//     # }}}
//     @classmethod  # save  # {{{
//     async def save(cls, trade: Trade) -> None:
//         logger.debug(f"{cls.__name__}.save()")
//
//         await Keeper.add(trade)
//
//     # }}}
//     @classmethod  # load  # {{{
//     async def load(cls, trade_id: Id) -> Trade:
//         logger.debug(f"{cls.__name__}.load()")
//
//         response = await Keeper.get(cls, trade_id=trade_id)
//         if len(response) == 1:  # response == [ Trade, ]
//             return response[0]
//
//         # else: error, trade not found
//         logger.error(f"trade_id='{trade_id}' does not exist!")
//         exit(3)
//
//     # }}}
//     @classmethod  # delete  # {{{
//     async def delete(cls, trade: Trade) -> None:
//         logger.debug(f"{cls.__name__}.delete()")
//
//         await Keeper.delete(trade)
//
//     # }}}
//     @classmethod  # update  # {{{
//     async def update(cls, trade: Trade) -> None:
//         logger.debug(f"{cls.__name__}.update()")
//
//         await Keeper.update(trade)
//
//     # }}}
//
//     @staticmethod  # encoderJson# {{{
//     def encoderJson(obj) -> Any:
//         if isinstance(obj, (DateTime, Date)):
//             return obj.isoformat()
//
//     # }}}
//     @staticmethod  # decoderJson# {{{
//     def decoderJson(obj) -> Any:
//         for k, v in obj.items():
//             if isinstance(v, str) and "+00:00" in v:
//                 obj[k] = DateTime.fromisoformat(obj[k])
//         return obj
//
//     # }}}
//
//     def __connectOrderSignals(self, order: Order):  # {{{
//         logger.debug(
//             f"{self.__class__.__name__}.__connectOrderSignals('{order}')"
//         )
//
//         order.posted.aconnect(self.onOrderPosted)
//         order.executed.aconnect(self.onOrderExecuted)
//
//     # }}}
//
//
// # }}}
