/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use avin_core::{Asset, Event};
use avin_strategy::Strategy;

pub struct Work {
    asset: Asset,
    strategys: Vec<Box<dyn Strategy>>,
    in_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    in_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
}
impl Work {
    pub fn new(asset: Asset) -> Work {
        let (in_tx, in_rx) = tokio::sync::mpsc::unbounded_channel();

        Work {
            asset,
            strategys: Vec::new(),
            in_tx,
            in_rx,
        }
    }

    pub fn figi(&self) -> &String {
        self.asset.figi()
    }
    pub fn add_strategy(&mut self, strategy: impl Strategy) {
        self.strategys.push(Box::new(strategy));
    }
    pub fn get_sender(&self) -> tokio::sync::mpsc::UnboundedSender<Event> {
        self.in_tx.clone()
    }

    pub async fn start(&mut self) {
        while let Some(e) = self.in_rx.recv().await {
            match e {
                Event::Bar(e) => {
                    self.asset.bar_event(e);
                    self.process_all_strategy();
                }
                Event::Tic(e) => {
                    self.asset.tic_event(e);
                    // self.process_strategy();
                }
                Event::Order(e) => {
                    for strategy in self.strategys.iter_mut() {
                        if *strategy.name() == e.owner {
                            strategy.order_event(e);
                            break;
                        }
                    }
                }
            }
        }
    }

    // private
    fn process_all_strategy(&mut self) {
        for strategy in self.strategys.iter_mut() {
            strategy.process(&self.asset);
        }
    }
}
