use avin_trader::*;

#[tokio::main]
async fn main() {
    avin_utils::init_logger();

    let mut trader = Trader::new();
    trader.start().await;
}
