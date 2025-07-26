/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};
use std::collections::HashMap;

/// One buy/sell deal in order execution.
///
/// # ru
/// Одна конкретная сделка в рамках исполнения ордера. Содержит временную
/// метку и количество.
///
/// Транзакциями владеет операция. А операцией владеет ордер.
/// А ордером владеет трейд. Таким образом всю остальную информацию о
/// направлении сделки, что это за актив, можно получить из родительских
/// структур.
#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Transaction {
    // TODO: пожалуй надо сюда добавить ts_nanos чтобы можно было
    // на графике адекватно отобразить где реально были транзакции
    // и на графике нужно рисовать именно транзакции а не орперацию.
    // Потому что операцию не понятно где рисовать... А трейд
    // в целом уже отображать квадратом например - от сюда до сюда...
    // цвет квадрата - win/loss. Ну как вариант... Или одной
    // наклонной линией... но вряд ли... и так много линий трендов.
    // А если по старому - стоп тейк и опен - то не понятно где
    // реально закрылся трейд если был перескок. И если в несколько
    // транзакций трейд идет - тоже будет нифига не видно. Старая
    // система подходила только для элементарных трейдов в одну
    // операцию покупки и одну операцию продажи.
    pub quantity: i32,
    pub price: f64,
}
impl Transaction {
    pub fn new(quantity: i32, price: f64) -> Self {
        Transaction { quantity, price }
    }
    pub fn from_bin(bytes: &[u8]) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn from_csv(csv: &str) -> Self {
        let parts: Vec<&str> = csv.split(';').collect();

        let quantity: i32 = parts[0].parse().unwrap();
        let price: f64 = parts[1].parse().unwrap();

        Transaction { quantity, price }
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
    pub fn to_csv(&self) -> String {
        format!("{};{};", self.quantity, self.price)
    }
    pub fn to_hash_map(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::new();
        hm.insert("quantity", self.quantity.to_string());
        hm.insert("price", self.price.to_string());

        hm
    }

    pub fn value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Transaction={}x{}", self.quantity, self.price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let t = Transaction::new(10, 325.5);
        assert_eq!(t.quantity, 10);
        assert_eq!(t.price, 325.5);
    }
    #[test]
    fn csv() {
        let t = Transaction::new(10, 325.5);
        let csv = t.to_csv();
        assert_eq!(csv, "10;325.5;");

        let from_csv = Transaction::from_csv(&csv);
        assert_eq!(t, from_csv);
    }
    #[test]
    fn bin() {
        let t = Transaction::new(10, 325.5);
        let bytes = t.to_bin();

        let decoded = Transaction::from_bin(&bytes);
        assert_eq!(t, decoded);
    }
}
