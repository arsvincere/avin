/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::path::Path;

use crate::core::trade::Trade;
use crate::utils::Cmd;
use bitcode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct TradeList {
    name: String,
    trades: Vec<Trade>,
}
impl TradeList {
    pub fn new(name: &str) -> TradeList {
        TradeList {
            name: name.to_string(),
            trades: Vec::new(),
        }
    }
    pub fn new_with_trades(name: &str, trades: Vec<Trade>) -> TradeList {
        TradeList {
            name: name.to_string(),
            trades,
        }
    }
    pub fn from_bin(bytes: &Vec<u8>) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn trades(&self) -> &Vec<Trade> {
        &self.trades
    }
    pub fn len(&self) -> usize {
        self.trades.len()
    }

    pub fn add_trade(&mut self, trade: Trade) {
        self.trades.push(trade);
    }
    pub fn clear_trades(&mut self) {
        self.trades.clear();
    }

    pub fn save(
        trade_list: &TradeList,
        path: &Path,
    ) -> Result<(), &'static str> {
        let bytes = trade_list.to_bin();
        Cmd::write_bin(&bytes, path).unwrap();

        Ok(())
    }
    pub fn load(file_path: &Path) -> Result<TradeList, &'static str> {
        let bytes = Cmd::read_bin(file_path).unwrap();
        let trade_list = TradeList::from_bin(&bytes);

        Ok(trade_list)
    }

    // pub fn save(trade_list: TradeList) -> Result<(), &'static str> {
    //     // let n = trade_list.len();
    //     // let mut dt: Vec<NaiveDateTime> = Vec::with_capacity(n);
    //     // let mut strategy: Vec<String> = Vec::with_capacity(n);
    //     // let mut typ: Vec<String> = Vec::with_capacity(n);
    //     // let mut asset: Vec<String> = Vec::with_capacity(n);
    //     // let mut orders: Vec<HashMap<String, String>> = Vec::with_capacity(n);
    //     //
    //     // for trade in trade_list.trades.borrow().iter() {
    //     //     if let Trade::Closed(t) = trade {
    //     //         dt.push(t.dt.naive_utc());
    //     //         strategy.push(t.strategy.clone());
    //     //         typ.push(t.typ.to_string());
    //     //         asset.push(t.asset.to_string());
    //     //         // orders.push(t.export_orders());
    //     //     }
    //     // }
    //
    //     todo!();
    // }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn save_load() {
        // create trade
        let iid = IID::from("moex_share_sber").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 4, 5, 14, 50, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let trade =
            Trade::new(ts, "Trend T3 Posterior v1", TradeType::Long, iid);

        // open trade - add first filled order
        let order = LimitOrder::new(Direction::Buy, 10, 301.0);
        let mut order = order.post("broker_id=100500");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 5, 14, 57, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            100,
            301.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        let mut trade = trade.open(Order::Limit(LimitOrder::Filled(order)));

        // add second filled order
        let order = LimitOrder::new(Direction::Sell, 10, 311.0);
        let mut order = order.post("broker_id=100501");
        let tr = Transaction::new(
            Utc.with_ymd_and_hms(2025, 4, 6, 14, 57, 0)
                .unwrap()
                .timestamp_nanos_opt()
                .unwrap(),
            100,
            311.0,
        );
        order.add_transaction(tr);
        let order = order.fill(3.0);
        trade.add_order(Order::Limit(LimitOrder::Filled(order)));

        // close trade
        let trade = trade.close();

        // create trade list
        let mut trade_list = TradeList::new("unit_test");

        // wrap & add trade
        trade_list.add_trade(Trade::Closed(trade));

        // save trade list
        let path = Path::new("tmp/trades.bin");
        TradeList::save(&trade_list, path).unwrap();

        // load trade list
        let loaded = TradeList::load(path).unwrap();

        assert_eq!(trade_list, loaded);

        // delete file
        Cmd::delete(path).unwrap();
    }
}
