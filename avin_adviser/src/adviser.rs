/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_connect::Tinkoff;
use avin_core::{Action, AssetList, Event, MarketData, StreamAction};
use avin_utils::CFG;

pub struct Adviser {
    asset_list: AssetList,
}
impl Adviser {
    pub fn new() -> Self {
        let name = &CFG.core.default_asset_list;
        let asset_list = AssetList::load_name(name).unwrap();

        Self { asset_list }
    }

    pub async fn start(&mut self) {
        let (broker_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();
        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

        log::info!("Load broker");
        let mut broker = Tinkoff::new(action_rx, event_tx);
        broker.connect().await.unwrap();
        tokio::spawn(async move { broker.start().await });

        log::info!("Subscribe assets");
        for asset in self.asset_list.assets().iter() {
            let a = Action::Subscribe(StreamAction::new(
                asset.iid().clone(),
                vec![MarketData::BAR_1M],
                // vec![MarketData::BAR_1M, MarketData::TIC], // TODO: ассет прием тиков не сделан
            ));
            broker_tx.send(a).unwrap();
        }

        log::info!("Start main loop");
        loop {
            // await events from broker -> send to asset
            if let Some(e) = event_rx.recv().await {
                let figi = e.figi();
                let asset = self.asset_list.find_figi_mut(figi).unwrap();

                log::debug!("Event {e}");

                match e {
                    Event::Bar(e) => asset.bar_event(e),
                    Event::Tic(e) => asset.tic_event(e),
                    Event::Order(_e) => todo!(),
                }

                // TODO:
                // Тут теперь когда ассеты обновлены можно применять к
                // ним фильтр и выдавать звуковой сигнал
            }
        }
    }

    pub fn asset_list(&self) -> &AssetList {
        &self.asset_list
    }
}
impl Default for Adviser {
    fn default() -> Self {
        Adviser::new()
    }
}
