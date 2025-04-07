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

#[tokio::main]
async fn main() {
    // share, iid
    let sber = Share::from_str("moex_share_sber").unwrap();

    // connect broker
    let mut b = Tinkoff::new().await;

    // subscribe bar 1M
    b.subscribe_bar(&sber.iid()).await.unwrap();
    b.subscribe_tic(&sber.iid()).await.unwrap();

    // get event receiver
    let mut r = b.get_receiver();

    // create task - broker start data stream loop
    tokio::spawn(async move { b.start_marketdata_stream().await });

    // event receiving loop
    println!("== start");
    let mut bar = 2;
    let mut tic = 2;
    while let Ok(e) = r.recv().await {
        match e {
            Event::Bar(e) => {
                println!("receive {}", e);
                assert_eq!(e.figi, *sber.figi());
                bar -= 1;
            }
            Event::Tic(e) => {
                println!("receive {}", e);
                assert_eq!(e.figi, *sber.figi());
                tic -= 1;
            }
        }
        if bar == 0 && tic == 0 {
            break;
        }
    }
    println!("== end");

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
