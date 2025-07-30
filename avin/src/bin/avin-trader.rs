use avin::trader::*;
use avin::utils;

#[tokio::main]
async fn main() {
    utils::init_logger();

    let mut trader = Trader::new();
    trader.start().await;
}
