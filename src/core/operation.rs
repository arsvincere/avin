/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::DT_FMT;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::Transaction;

#[derive(Debug)]
pub struct Operation {
    pub dt: DateTime<Utc>,
    pub quantity: u32,
    pub amount: f64,
    pub commission: Option<f64>,
}
impl Operation {
    pub fn from(transactions: Vec<Transaction>) -> Self {
        if transactions.is_empty() {
            panic!("Empty transactions list! Fail to create operation!");
        }

        let mut quantity: u32 = 0;
        let mut amount: f64 = 0.0;
        for i in transactions.iter() {
            quantity += i.quantity;
            amount += i.quantity as f64 * i.price;
        }

        Self {
            dt: transactions.last().unwrap().dt,
            quantity,
            amount,
            commission: None,
        }
    }

    pub fn to_hash_map(&self) -> HashMap<&str, String> {
        let commission = if self.commission.is_some() {
            self.commission.unwrap().to_string()
        } else {
            String::new()
        };

        let mut info = HashMap::new();
        info.insert("dt", self.dt.to_rfc3339());
        info.insert("quantity", self.quantity.to_string());
        info.insert("amount", self.amount.to_string());
        info.insert("commission", commission);

        info
    }
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formatted = format!("{}", self.dt.format(DT_FMT));
        let commission = if self.commission.is_some() {
            format!("+{}", self.commission.unwrap())
        } else {
            String::new()
        };
        write!(
            f,
            "Operation={} {}={}{}",
            formatted, self.quantity, self.amount, commission
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let dt = Utc::now();
        let t1 = Transaction::new(dt.clone(), 10, 320.0);
        std::thread::sleep(std::time::Duration::new(0, 100));
        let dt = Utc::now();
        let t2 = Transaction::new(dt.clone(), 10, 330.0);

        let op = Operation::from(vec![t1, t2]);
        dbg!(&op);
    }
}

// class Operation:
//     def __init__(  # {{{
//         self,
//         account_name: str,
//         dt: datetime,
//         direction: Direction,
//         instrument: Instrument,
//         lots: int,
//         quantity: int,
//         price: float,
//         amount: float,
//         commission: Optional[float],
//         operation_id: Optional[Id] = None,
//         order_id: Optional[Id] = None,
//         trade_id: Optional[Id] = None,
//         meta: Optional[str] = None,
//     ):
//         logger.debug("Operation.__init__()")
//
//         self.account_name = account_name
//         self.dt = dt
//         self.direction = direction
//         self.instrument = instrument
//         self.price = price
//         self.lots = lots
//         self.quantity = quantity
//         self.amount = amount
//         self.commission = commission
//         self.operation_id = operation_id
//         self.order_id = order_id
//         self.trade_id = trade_id
//         self.meta = meta
//
//     # }}}
//     def __str__(self):  # {{{
//         usr_dt = self.dt + Usr.TIME_DIF
//         str_dt = usr_dt.strftime("%Y-%m-%d %H:%M")
//         string = (
//             f"{str_dt} {self.direction.name} {self.instrument.ticker} "
//             f"{self.quantity} * {self.price} = {self.amount} "
//             f"+ {self.commission}"
//         )
//         return string
//
//     # }}}
//     def pretty(self) -> str:  # {{{
//         logger.debug(f"{self.__class__.__name__}.pretty()")
//
//         text = f"""Operation:
//     id:             {self.operation_id}
//     account:        {self.account_name}
//     dt:             {Usr.localTime(self.dt)}
//     direction:      {self.direction.name}
//     instrument:     {self.instrument}
//     lots:           {self.lots}
//     quantity:       {self.quantity}
//     price:          {self.price}
//     amount:         {self.amount}
//     commission:     {self.commission}
//     order_id:       {self.order_id}
//     trade_it:       {self.trade_id}
//     meta:           {self.meta}
// """
//         return text
//
//     # }}}
//
//     async def setParentTrade(self, trade):  # {{{
//         logger.debug(f"Operation.setParentTrade({trade})")
//         self.trade_id = trade.trade_id
//         await Operation.update(self)
//
//     # }}}
//     @classmethod  # fromRecord  # {{{
//     async def fromRecord(cls, record: asyncpg.Record) -> Operation:
//         logger.debug(f"Operation.fromRecord({record})")
//
//         instrument = await Instrument.fromFigi(record["figi"])
//
//         op = Operation(
//             account_name=record["account"],
//             dt=record["dt"],
//             direction=Direction.fromStr(record["direction"]),
//             instrument=instrument,
//             lots=record["lots"],
//             quantity=record["quantity"],
//             price=record["price"],
//             amount=record["amount"],
//             commission=record["commission"],
//             operation_id=Id.fromStr(record["operation_id"]),
//             order_id=Id.fromStr(record["order_id"]),
//             trade_id=Id.fromStr(record["trade_id"]),
//             meta=record["meta"],
//         )
//         return op
//
//     # }}}
//     @classmethod  # save  # {{{
//     async def save(cls, operation: Operation) -> None:
//         logger.debug(f"Operation.save({operation})")
//         await Keeper.add(operation)
//
//     # }}}
//     @classmethod  # load  # {{{
//     async def load(cls, operation_id: Id) -> Operation:
//         logger.debug(f"Operation.load({operation_id})")
//         op = await Keeper.get(cls, operation_id=operation_id)
//         return op
//
//     # }}}
//     @classmethod  # delete  # {{{
//     async def delete(cls, operation: Operation) -> None:
//         logger.debug(f"Operation.delete({operation})")
//         await Keeper.delete(operation)
//
//     # }}}
//     @classmethod  # update  # {{{
//     async def update(cls, operation: Operation) -> None:
//         logger.debug(f"Operation.update({operation})")
//         await Keeper.update(operation)
//
//     # }}}
