/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_connect::Tinkoff;
use avin_core::{Action, AssetList, Event, MarketData, StreamAction};
use avin_utils::CFG;

use crate::{Condition, Notice};

pub struct Adviser {
    asset_list: AssetList,
    conditions: Vec<Box<dyn Condition>>,
}
impl Adviser {
    pub fn new() -> Self {
        let name = &CFG.core.default_asset_list;
        let asset_list = AssetList::load_name(name).unwrap();

        Self {
            asset_list,
            conditions: Vec::new(),
        }
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
        // await events from broker -> send to asset
        while let Some(e) = event_rx.recv().await {
            let figi = e.figi();
            let asset = self.asset_list.find_figi_mut(figi).unwrap();

            match e {
                Event::Bar(e) => asset.bar_event(e),
                Event::Tic(e) => asset.tic_event(e),
                Event::OrderBook(_e) => todo!(),
                Event::Order(_) => continue,
            }

            // Тут теперь когда ассет обновлен можно применять к
            // ним условие и выдавать уведомление
            for condition in self.conditions.iter() {
                if let Some(notice) = condition.apply(asset) {
                    dbg!(&notice);
                    Self::notify(notice);
                }
            }
        }

        // цикл какого то хрена закончился...
        let notice = Notice::new("Цикл капут!", "Все пиздец!");
        Self::notify(notice);
    }
    pub fn asset_list(&self) -> &AssetList {
        &self.asset_list
    }

    // private
    fn notify(notice: Notice) {
        let mut command = std::process::Command::new("/bin/notify-send");
        command.arg("-u"); // silent
        command.arg("critical");
        command.arg(notice.title);
        command.arg(notice.body);
        command.spawn().unwrap().wait().unwrap();
    }
}
impl Default for Adviser {
    fn default() -> Self {
        Adviser::new()
    }
}
