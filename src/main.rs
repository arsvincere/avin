/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 *                                                                         *
 * * * * * * * * Open source cross-platform trading system * * * * * * * * */

use avin::*;

#[tokio::main]
async fn main() {
    log::set_logger(&utils::LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Welcome to AVIN Trade System!");

    let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();
    let mut broker = Tinkoff::new(event_tx.clone());
    broker.connect().await.unwrap();
    dbg!("connected!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    let acc = broker.get_accounts().await.unwrap();
    dbg!(&acc);
    // broker.create_marketdata_stream().await.unwrap();
    broker.create_transactions_stream().await.unwrap();
    broker.start().await
}
