/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

mod limit_order;
mod market_order;
mod stop_order;

use crate::core::direction::Direction;
use crate::core::operation::Operation;
use crate::core::transaction::Transaction;

pub use limit_order::{
    FilledLimitOrder, LimitOrder, NewLimitOrder, PostedLimitOrder,
    RejectedLimitOrder,
};
pub use market_order::{
    FilledMarketOrder, MarketOrder, NewMarketOrder, PostedMarketOrder,
    RejectedMarketOrder,
};
pub use stop_order::{
    NewStopOrder, PostedStopOrder, RejectedStopOrder, StopOrder,
    TriggeredStopOrder,
};

#[derive(Debug)]
pub enum Order {
    Market(MarketOrder),
    Limit(LimitOrder),
    Stop(StopOrder),
}
impl Order {
    pub fn is_posted(&self) -> bool {
        match self {
            Order::Market(market) => match market {
                MarketOrder::Posted(_) => true,
                _ => false,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::Posted(_) => true,
                _ => false,
            },
            Order::Stop(stop) => match stop {
                StopOrder::Posted(_) => true,
                _ => false,
            },
        }
    }
    pub fn is_filled(&self) -> bool {
        match self {
            Order::Market(o) => match o {
                MarketOrder::Filled(_) => true,
                _ => false,
            },
            Order::Limit(o) => match o {
                LimitOrder::Filled(_) => true,
                _ => false,
            },
            Order::Stop(_) => panic!("Stop order can't be filled"),
        }
    }
    pub fn direction(&self) -> &Direction {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(o) => &o.direction,
                MarketOrder::Posted(o) => &o.direction,
                MarketOrder::Filled(o) => &o.direction,
                MarketOrder::Rejected(o) => &o.direction,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(o) => &o.direction,
                LimitOrder::Posted(o) => &o.direction,
                LimitOrder::Filled(o) => &o.direction,
                LimitOrder::Rejected(o) => &o.direction,
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(o) => &o.direction,
                StopOrder::Posted(o) => &o.direction,
                StopOrder::Rejected(o) => &o.direction,
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(o) => &o.direction,
                    TriggeredStopOrder::Limit(o) => &o.direction,
                },
            },
        }
    }
    pub fn lots(&self) -> u32 {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(o) => o.lots,
                MarketOrder::Posted(o) => o.lots,
                MarketOrder::Filled(o) => o.lots,
                MarketOrder::Rejected(o) => o.lots,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(o) => o.lots,
                LimitOrder::Posted(o) => o.lots,
                LimitOrder::Filled(o) => o.lots,
                LimitOrder::Rejected(o) => o.lots,
            },
            Order::Stop(stop) => match stop {
                StopOrder::New(o) => o.lots,
                StopOrder::Posted(o) => o.lots,
                StopOrder::Rejected(o) => o.lots,
                StopOrder::Triggered(o) => match o {
                    TriggeredStopOrder::Market(o) => o.lots,
                    TriggeredStopOrder::Limit(o) => o.lots,
                },
            },
        }
    }
    pub fn transactions(&self) -> Option<&Vec<Transaction>> {
        match self {
            Order::Market(market) => match market {
                MarketOrder::New(_) => None,
                MarketOrder::Posted(o) => Some(&o.transactions),
                MarketOrder::Filled(o) => Some(&o.transactions),
                MarketOrder::Rejected(_) => None,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::New(_) => None,
                LimitOrder::Posted(o) => Some(&o.transactions),
                LimitOrder::Filled(o) => Some(&o.transactions),
                LimitOrder::Rejected(_) => None,
            },
            Order::Stop(stop) => match stop {
                _ => None,
            },
        }
    }
    pub fn operation(&self) -> Option<&Operation> {
        match self {
            Order::Market(market) => match market {
                MarketOrder::Filled(o) => Some(&o.operation),
                _ => None,
            },
            Order::Limit(limit) => match limit {
                LimitOrder::Filled(o) => Some(&o.operation),
                _ => None,
            },
            Order::Stop(stop) => match stop {
                _ => None,
            },
        }
    }
}

// class Order(metaclass=abc.ABCMeta):  # {{{
//     class Type(enum.Enum):  # {{{
//         UNDEFINE = 0
//         MARKET = 1
//         LIMIT = 2
//         STOP = 3
//         WAIT = 4
//         TRAILING = 5
//         STOP_LOSS = 6
//         TAKE_PROFIT = 7
//
//     # }}}
//     class Status(enum.Enum):  # {{{
//         UNDEFINE = enum.auto()
//         NEW = enum.auto()
//         SUBMIT = enum.auto()
//
//         POSTED = enum.auto()
//         OFF = enum.auto()
//
//         PARTIAL = enum.auto()
//         FILLED = enum.auto()
//
//         EXECUTED = enum.auto()
//         TRIGGERED = enum.auto()
//
//         CANCELED = enum.auto()
//         BLOCKED = enum.auto()
//         REJECTED = enum.auto()
//         EXPIRED = enum.auto()
//
//         @classmethod  # fromStr
//         def fromStr(cls, string: str) -> Order.Status:
//             statuses = {
//                 "NEW": Order.Status.NEW,
//                 "SUBMIT": Order.Status.SUBMIT,
//                 "POSTED": Order.Status.POSTED,
//                 "OFF": Order.Status.OFF,
//                 "PARTIAL": Order.Status.PARTIAL,
//                 "FILLED": Order.Status.FILLED,
//                 "EXECUTED": Order.Status.EXECUTED,
//                 "TRIGGERED": Order.Status.TRIGGERED,
//                 "CANCELED": Order.Status.CANCELED,
//                 "BLOCKED": Order.Status.BLOCKED,
//                 "REJECTED": Order.Status.REJECTED,
//                 "EXPIRED": Order.Status.EXPIRED,
//             }
//             return statuses[string]
//
//     # }}}
//
//     @abc.abstractmethod  # __init__# {{{
//     def __init__(
//         self,
//         order_type,
//         account_name,
//         direction,
//         instrument,
//         lots,
//         quantity,
//         status,
//         order_id,
//         trade_id,
//         exec_lots,
//         exec_quantity,
//         meta,
//         broker_id,
//         transacts,
//     ):
//         logger.debug("Order.__init__()")
//         assert lots > 0
//         assert quantity > 0
//
//         self.type = order_type
//         self.account_name = account_name
//         self.direction = direction
//         self.instrument = instrument
//         self.lots = lots
//         self.quantity = quantity
//         self.status = status
//
//         self.order_id = order_id
//         self.trade_id = trade_id
//         self.exec_lots = exec_lots
//         self.exec_quantity = exec_quantity
//
//         self.meta = meta
//         self.broker_id = broker_id
//         self.transactions = transacts if transacts else TransactionList()
//
//         # Signals
//         self.statusChanged = AsyncSignal(Order)
//         self.posted = AsyncSignal(Order)
//         self.partial = AsyncSignal(Order)
//         self.filled = AsyncSignal(Order)
//         self.executed = AsyncSignal(Order, Operation)
//         self.rejected = AsyncSignal(Order)
//         self.canceled = AsyncSignal(Order)
//
//     # }}}
//     def __str__(self):  # {{{
//         string = (
//             f"Order id={self.order_id} "
//             f"[{self.type.name}] "
//             f"[{self.status.name}] "
//             f"({self.account_name}) "
//             f"{self.direction.name} "
//             f"{self.instrument.ticker} "
//             f"{self.exec_lots}/{self.lots} lot"
//         )
//
//         if self.type == Order.Type.MARKET:
//             pass
//         elif self.type == Order.Type.LIMIT:
//             string += f", {self.quantity}x{self.price}"
//         elif self.type == Order.Type.STOP:
//             string += f", stop={self.stop_price}, exec={self.exec_price}"
//
//         return string
//
//     # }}}
//
//     def pretty(self) -> str:  # {{{
//         logger.debug(f"{self.__class__.__name__}.pretty()")
//
//         text = f"""Order:
//     id:             {self.order_id}
//     type:           {self.type.name}
//     account:        {self.account_name}
//     direction:      {self.direction.name}
//     instrument:     {self.instrument}
//     lots:           {self.lots}
//     quantity:       {self.quantity}
//     status:         {self.status}
//     trade_id:       {self.trade_id}
//     exec_lots:      {self.exec_lots}
//     exec_quantity:  {self.exec_quantity}
//     meta:           {self.meta}
//     broker_id:      {self.broker_id}
//     transacts:      {self.transactions}
// """
//
//         # for limit order appen limit price
//         T = Order.Type
//         if self.type == T.LIMIT:
//             text += f"""--
//     price:           {self.price}
// """
//
//         # for stop order appen stop/exec price
//         if self.type in (T.STOP, T.STOP_LOSS, T.TAKE_PROFIT):
//             text += f"""--
//     stop_price:           {self.stop_price}
//     exec_price:           {self.exec_price}
// """
//         return text
//
//     # }}}
//     def isActive(self) -> bool:  # {{{
//         logger.debug(f"{self.__class__.__name__}.isActive()")
//
//         return self.status.value < Order.Status.EXECUTED.value
//
//     # }}}
//
//     async def setStatus(self, status: Order.Status):  # {{{
//         logger.debug(f"Order.setStatus({status})")
//
//         # NOTE: бывает когда маркет ордер сразу исполняется, или лимитка
//         # сразу исполняется. После этого акканут запрашивает Broker.syncOrder
//         # получает FILLED.
//         # А через доли секунды прилетает TransactionEvent, и в его
//         # обработке снова вызывается Broker.syncOrder и снова получаем
//         # статус FILLED.
//         # --
//         # Не синхронизировать ордер сразу после выставления не вариант
//         # нужне же узнать поставилась ли лимитка/стоп...
//         # --
//         # Как решить этот косяк на уровне общения Account / Broker
//         # не придумал, так что костыляю тут - повторное присвоение
//         # статуса просто пропускаем.
//         if self.status == status:
//             return
//
//         self.status = status
//         await Order.update(self)
//
//         # emitting special signal for this status
//         if status == Order.Status.POSTED:
//             await self.posted.aemit(self)
//         if status == Order.Status.REJECTED:
//             await self.rejected.aemit(self)
//         if status == Order.Status.CANCELED:
//             await self.canceled.aemit(self)
//
//         # emiting common signal
//         await self.statusChanged.aemit(self)
//
//     # }}}
//     async def setParentTrade(self, trade):  # {{{
//         logger.debug(f"Order.setParentTrade({trade})")
//
//         self.trade_id = trade.trade_id
//         await Order.update(self)
//
//     # }}}
//     async def setMeta(self, broker_response: str):  # {{{
//         logger.debug(f"Order.setMeta({broker_response})")
//
//         self.meta = broker_response
//         await Order.update(self)
//
//     # }}}
//     async def attachTransaction(self, transaction: Transaction):  # {{{
//         logger.debug(f"Order.attachTransaction({transaction})")
//
//         assert transaction.order_id == self.order_id
//         self.transactions.add(transaction)
//
//     # }}}
//
//     @classmethod  # fromRecord  # {{{
//     def fromRecord(cls, record):
//         logger.debug(f"Order.fromRecord({record})")
//
//         methods = {
//             "MARKET": Order.__marketOrderFromRecord,
//             "LIMIT": Order.__limitOrderFromRecord,
//             "STOP": Order.__stopOrderFromRecord,
//             "STOP_LOSS": Order.__stopLossFromRecord,
//             "TAKE_PROFIT": Order.__takeProfitFromRecord,
//         }
//         method = methods[record["order_type"]]
//         order = method(record)
//         return order
//
//     # }}}
//     @classmethod  # save  # {{{
//     async def save(cls, order) -> None:
//         logger.debug(f"Order.save({order})")
//         await Keeper.add(order)
//
//     # }}}
//     @classmethod  # load  # {{{
//     async def load(cls, order_id: Id):
//         logger.debug(f"Order.load({order_id})")
//
//         order_list = await Keeper.get(cls, order_id=order_id)
//         assert len(order_list) == 1
//         order = order_list[0]
//         return order
//
//     # }}}
//     @classmethod  # delete  # {{{
//     async def delete(cls, order) -> None:
//         logger.debug(f"Order.delete({order})")
//         await Keeper.delete(order)
//
//     # }}}
//     @classmethod  # update  # {{{
//     async def update(cls, order) -> None:
//         logger.debug(f"Order.update({order})")
//         await Keeper.update(order)
//
//     # }}}
//
//     @classmethod  # __marketOrderFromRecord{{{
//     def __marketOrderFromRecord(cls, record):
//         logger.debug(f"Order.__marketOrderFromRecord({record})")
//
//         order = MarketOrder(
//             account_name=record["order_account"],
//             direction=Direction.fromStr(record["order_direction"]),
//             instrument=Instrument.fromRecord(record),
//             lots=record["order_lots"],
//             quantity=record["order_quantity"],
//             status=Order.Status.fromStr(record["order_status"]),
//             order_id=record["order_id"],
//             trade_id=record["order_trade_id"],
//             exec_lots=record["order_exec_lots"],
//             exec_quantity=record["order_exec_quantity"],
//             meta=record["order_meta"],
//             broker_id=record["order_broker_id"],
//         )
//         return order
//
//     # }}}
//     @classmethod  # __limitOrderFromRecord{{{
//     def __limitOrderFromRecord(cls, record):
//         logger.debug(f"Order.__limitOrderFromRecord({record})")
//
//         order = LimitOrder(
//             account_name=record["order_account"],
//             direction=Direction.fromStr(record["order_direction"]),
//             instrument=Instrument.fromRecord(record),
//             lots=record["order_lots"],
//             quantity=record["order_quantity"],
//             price=record["order_price"],
//             status=Order.Status.fromStr(record["order_status"]),
//             order_id=Id.fromStr(record["order_id"]),
//             trade_id=Id.fromStr(record["order_trade_id"]),
//             exec_lots=record["order_exec_lots"],
//             exec_quantity=record["order_exec_quantity"],
//             meta=record["order_meta"],
//             broker_id=record["order_broker_id"],
//         )
//         return order
//
//     # }}}
//     @classmethod  # __stopOrderFromRecord{{{
//     def __stopOrderFromRecord(cls, record):
//         logger.debug(f"Order.__stopOrderFromRecord({record})")
//
//         order = StopOrder(
//             account_name=record["order_account"],
//             direction=Direction.fromStr(record["order_direction"]),
//             instrument=Instrument.fromRecord(record),
//             lots=record["order_lots"],
//             quantity=record["order_quantity"],
//             stop_price=record["order_stop_price"],
//             exec_price=record["order_exec_price"],
//             status=Order.Status.fromStr(record["order_status"]),
//             order_id=record["order_id"],
//             trade_id=record["order_trade_id"],
//             exec_lots=record["order_exec_lots"],
//             exec_quantity=record["order_exec_quantity"],
//             meta=record["order_meta"],
//             broker_id=record["order_broker_id"],
//         )
//         return order
//
//     # }}}
//     @classmethod  # __stopLossFromRecord{{{
//     def __stopLossFromRecord(cls, record):
//         logger.debug(f"Order.__stopLossFromRecord({record})")
//
//         order = StopLoss(
//             account_name=record["order_account"],
//             direction=Direction.fromStr(record["order_direction"]),
//             instrument=Instrument.fromRecord(record),
//             lots=record["order_lots"],
//             quantity=record["order_quantity"],
//             stop_price=record["order_stop_price"],
//             exec_price=record["order_exec_price"],
//             status=Order.Status.fromStr(record["order_status"]),
//             order_id=record["order_id"],
//             trade_id=record["order_trade_id"],
//             exec_lots=record["order_exec_lots"],
//             exec_quantity=record["order_exec_quantity"],
//             meta=record["order_meta"],
//             broker_id=record["order_broker_id"],
//         )
//         return order
//
//     # }}}
//     @classmethod  # __takeProfitFromRecord{{{
//     def __takeProfitFromRecord(cls, record):
//         logger.debug(f"Order.__takeProfitFromRecord({record})")
//
//         order = TakeProfit(
//             account_name=record["order_account"],
//             direction=Direction.fromStr(record["order_direction"]),
//             instrument=Instrument.fromRecord(record),
//             lots=record["order_lots"],
//             quantity=record["order_quantity"],
//             stop_price=record["order_stop_price"],
//             exec_price=record["order_exec_price"],
//             status=Order.Status.fromStr(record["order_status"]),
//             order_id=record["order_id"],
//             trade_id=record["order_trade_id"],
//             exec_lots=record["order_exec_lots"],
//             exec_quantity=record["order_exec_quantity"],
//             meta=record["order_meta"],
//             broker_id=record["order_broker_id"],
//         )
//         return order
//
//
// # }}}
//
//
// # }}}
// class MarketOrder(Order):  # {{{
//     def __init__(
//         self,
//         account_name: str,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         status: Order.Status = Order.Status.NEW,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         exec_lots: int = 0,
//         exec_quantity: int = 0,
//         meta: str = "",
//         broker_id: str = "",
//         transactions: Optional[TransactionList] = None,
//     ):
//         # TODO: в базовом классе значения по умолчанию пожалуй не нужны
//         # наоборот пусть явно все передается. Явное лучше неявного.
//         # А вот в производных классах для удобства можно в конструкторах
//         # поставить значения по умолчанию.
//         super().__init__(
//             Order.Type.MARKET,
//             account_name,
//             direction,
//             instrument,
//             lots,
//             quantity,
//             status,
//             order_id,
//             trade_id,
//             exec_lots,
//             exec_quantity,
//             meta=meta,
//             broker_id=broker_id,
//             transacts=transactions,
//         )
//
//
// # }}}
// class LimitOrder(Order):  # {{{
//     def __init__(
//         self,
//         account_name: str,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         price: float,
//         status: Order.Status = Order.Status.NEW,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         exec_lots: int = 0,
//         exec_quantity: int = 0,
//         meta: str = "",
//         broker_id: str = "",
//         transactions: Optional[TransactionList] = None,
//     ):
//         super().__init__(
//             Order.Type.LIMIT,
//             account_name,
//             direction,
//             instrument,
//             lots,
//             quantity,
//             status,
//             order_id,
//             trade_id,
//             exec_lots,
//             exec_quantity,
//             meta=meta,
//             broker_id=broker_id,
//             transacts=transactions,
//         )
//         self.price = price
//
//
// # }}}
// class StopOrder(Order):  # {{{
//     def __init__(
//         self,
//         account_name: str,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         stop_price: float,
//         exec_price: float,
//         status: Order.Status = Order.Status.NEW,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         exec_lots: int = 0,
//         exec_quantity: int = 0,
//         meta: str = "",
//         broker_id: str = "",
//         transactions: Optional[TransactionList] = None,
//     ):
//         super().__init__(
//             Order.Type.STOP,
//             account_name,
//             direction,
//             instrument,
//             lots,
//             quantity,
//             status,
//             order_id,
//             trade_id,
//             exec_lots,
//             exec_quantity,
//             meta=meta,
//             broker_id=broker_id,
//             transacts=transactions,
//         )
//         self.stop_price = stop_price
//         self.exec_price = exec_price
//
//
// # }}}
// class StopLoss(Order):  # {{{
//     def __init__(
//         self,
//         account_name: str,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         stop_price: float,
//         exec_price: float | None,
//         status: Order.Status = Order.Status.NEW,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         exec_lots: int = 0,
//         exec_quantity: int = 0,
//         meta: str = "",
//         broker_id: str = "",
//         transactions: Optional[TransactionList] = None,
//     ):
//         super().__init__(
//             Order.Type.STOP_LOSS,
//             account_name,
//             direction,
//             instrument,
//             lots,
//             quantity,
//             status,
//             order_id,
//             trade_id,
//             exec_lots,
//             exec_quantity,
//             meta=meta,
//             broker_id=broker_id,
//             transacts=transactions,
//         )
//         self.stop_price = stop_price
//         self.exec_price = exec_price
//
//
// # }}}
// class TakeProfit(Order):  # {{{
//     def __init__(
//         self,
//         account_name: str,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         stop_price: float,
//         exec_price: float,
//         status: Order.Status = Order.Status.NEW,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         exec_lots: int = 0,
//         exec_quantity: int = 0,
//         meta: str = "",
//         broker_id: str = "",
//         transactions: Optional[TransactionList] = None,
//     ):
//         # TODO: Instrument.min_price_step - все таки надо грузить
//         # сразу при создании из базы, иначе тут жопа, каждый раз загружать
//         # эту хуйню...
//         # инфо же один хер загружается - так вот пусть к инфо полю
//         # и лепится в конструкторе сразу. Будут проблемы с памятью, тогда
//         # и буду решать, а то пока проблемы только с тем что везде
//         # это количество лотов и мин прайс степ приходится грузить.
//
//         super().__init__(
//             Order.Type.TAKE_PROFIT,
//             account_name,
//             direction,
//             instrument,
//             lots,
//             quantity,
//             status,
//             order_id,
//             trade_id,
//             exec_lots,
//             exec_quantity,
//             meta=meta,
//             broker_id=broker_id,
//             transacts=transactions,
//         )
//         self.stop_price = stop_price
//         self.exec_price = exec_price
//
//
// #
// #
// # # }}}
// class WaitOrder(Order):  # {{{
//     ...
//
//
// # }}}
// class TrailingOrder(Order):  # {{{
//     ...
//
//
// # }}}
