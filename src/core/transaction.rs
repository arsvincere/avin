/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::collections::HashMap;

use crate::DT_FMT;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Transaction {
    pub dt: DateTime<Utc>,
    pub quantity: u32,
    pub price: f64,
}
impl Transaction {
    pub fn new(dt: DateTime<Utc>, quantity: u32, price: f64) -> Self {
        Transaction {
            dt,
            quantity,
            price,
        }
    }
    pub fn to_hash_map(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::new();
        hm.insert("dt", self.dt.to_rfc3339());
        hm.insert("quantity", self.quantity.to_string());
        hm.insert("price", self.price.to_string());

        hm
    }
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formatted = format!("{}", self.dt.format(DT_FMT));
        write!(
            f,
            "Transaction={} {}x{}",
            formatted, self.quantity, self.price
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let dt = Utc::now();
        let t = Transaction::new(dt.clone(), 10, 325.5);
        assert_eq!(t.dt, dt);
        assert_eq!(t.quantity, 10);
        assert_eq!(t.price, 325.5);
    }
}

// class Transaction:
//     def __init__(  # {{{
//         self,
//         order_id: str,
//         dt: datetime,
//         quantity: int,
//         price: float,
//         broker_id: str,
//     ):
//         self.order_id = order_id
//         self.dt = dt
//         self.quantity = quantity
//         self.price = price
//         self.broker_id = broker_id
//
//     # }}}
//
// class TransactionList:
//     def __init__(self, transactions: Optional[list] = None):  # {{{
//         logger.debug(f"{self.__class__.__name__}.__init__()")
//
//         self.__transactions = transactions if transactions else list()
//
//     # }}}
//     def __str__(self):  # {{{
//         return f"TransactionList={self.__transactions}"
//
//     # }}}
//     def __iter__(self) -> Iterator:  # {{{
//         return iter(self.__transactions)
//
//     # }}}
//     def __contains__(self, transaction) -> bool:  # {{{
//         return transaction in self.__transactions
//
//     # }}}
//     def __len__(self) -> int:  # {{{
//         return len(self.__transactions)
//
//     # }}}
//     @property  # first  # {{{
//     def first(self) -> Transaction:
//         assert len(self.__transactions)
//
//         return self.__transactions[0]
//
//     # }}}
//     @property  # last  # {{{
//     def last(self) -> Transaction:
//         assert len(self.__transactions)
//
//         return self.__transactions[-1]
//
//     # }}}
//     @property  # transactions  # {{{
//     def transactions(self) -> list[Transaction]:
//         return self.__transactions
//
//     @transactions.setter
//     def transactions(self, transactions: list[Transaction]) -> None:
//         self.__transactions = transactions
//
//     # }}}
//     def add(self, transaction: Transaction) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.add()")
//
//         assert isinstance(transaction, Transaction)
//         if transaction not in self.__transactions:
//             self.__transactions.append(transaction)
//             return
//
//         logger.warning(f"{transaction} already in list '{self}'")
//
//     # }}}
//     def remove(self, transaction: Transaction) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.remove()")
//
//         try:
//             self.__transactions.remove(transaction)
//         except ValueError:
//             logger.exception(
//                 f"TransactionList.remove(transaction) failed: "
//                 f"{transaction}' not in list",
//             )
//
//     # }}}
//     def clear(self) -> None:  # {{{
//         logger.debug(f"{self.__class__.__name__}.clear()")
//
//         self.__transactions.clear()
//
//     # }}}
//     def quantity(self):  # {{{
//         logger.debug(f"{self.__class__.__name__}.quantity()")
//
//         total = 0
//         for t in self.__transactions:
//             total += t.quantity
//
//         return total
//
//     # }}}
//     def amount(self) -> float:  # {{{
//         logger.debug(f"{self.__class__.__name__}.amount()")
//
//         total = 0
//         for t in self.__transactions:
//             amount = t.price * t.quantity
//             total += amount
//
//         return total
//
//     # }}}
//     def average(self) -> float:  # {{{
//         logger.debug(f"{self.__class__.__name__}.average()")
//
//         if self.quantity() == 0:
//             return 0.0
//
//         return self.amount() / self.quantity()
//
//
// # }}}
