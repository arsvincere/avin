/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 * """"""                  """""""""""""""""""                             *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use avin::*;
use log::LevelFilter;

#[tokio::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    let mut t = Trader::new();
    t.start().await;

    // NOTE:
    // BarEvent, канал, сендер ресейвер...

    // let mut asset = Asset::from_str("moex_share_sber").unwrap();
    // let tf = TimeFrame::new("D");
    // let chart = asset.load_chart(&tf).unwrap();
    //
    // let bar = Bar::new(100500, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
    // let bar_event = BarEvent::new(bar, tf);
    // let e = Event::Bar(bar_event);
    //
    // let mut new_bar_rx = chart.subscribe_upd_bar();
    //
    // let tx = asset.sender();
    // tx.send(e.clone()).unwrap();
    // tx.send(e.clone()).unwrap();
    // tx.send(e.clone()).unwrap();
    //
    // let handle = tokio::spawn(async move { asset.start().await });
    //
    // while let Ok(bar) = new_bar_rx.recv().await {
    //     println!("new bar receive {:?}", &bar);
    // }
    //
    // handle.await.unwrap();
}
