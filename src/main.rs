/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * URL:         http://arsvincere.com                                      *
 * AUTHOR:      Alex Avin                                                  *
 * E-MAIL:      mr.alexavin@gmail.com                                      *
 * LICENSE:     MIT                                                        *
 *   __   _____   ____     _    _  ___  __   _  _____  _____   ____  _____ *
 *  /__\  |___/  |___       \  /    |   | \  |  |      |___   |___/  |___  *
 * |    | |  \_  ____|       \/    _|_  |  \_|  |____  |____  |  \_  |____ *
 *                                                                         *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use avin::*;

#[tokio::main]
async fn main() {
    let mut asset = Asset::from_str("moex_share_sber").unwrap();
    let tf = TimeFrame::new("D");
    asset.load_chart(&tf).unwrap();

    let bar = Bar::new(100500, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
    let bar_event = BarEvent::new(bar, tf);
    let e = Event::Bar(bar_event);

    let tx = asset.sender();
    tx.send(e.clone()).unwrap();
    tx.send(e.clone()).unwrap();
    tx.send(e.clone()).unwrap();

    let handle = tokio::spawn(async move { asset.listen().await });

    handle.await.unwrap();
}
